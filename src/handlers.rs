use std::collections::HashSet;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::{Response, StatusCode},
    response::{ErrorResponse, IntoResponse},
};

use serde::Deserialize;
use sqlx::{QueryBuilder, Sqlite, sqlite::SqlitePool};
use tracing::error;

use crate::{
    AppState,
    models::{
        CreateRecipeRequest, Ingredient, Instruction, Recipe, RecipeBase, UpdateIngredientRequest,
        UpdateInstructionRequest, UpdateRecipeRequest,
    },
    query_utils::add_in_expression,
};

#[axum::debug_handler]
pub async fn get_recipe_by_id(
    State(AppState { db }): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Recipe>, ErrorResponse> {
    get_recipe(db, id).await.map(Json)
}

async fn get_recipe(db: SqlitePool, id: i64) -> Result<Recipe, ErrorResponse> {
    let maybe_recipe = sqlx::query_as!(
        RecipeBase,
        "SELECT id, name, author, description, difficulty, estimated_duration FROM recipes WHERE id = ?",
        id)
    .fetch_optional(&db)
    .await
    .map_err(handle_internal_server_error)?;

    let recipe = match maybe_recipe {
        None => Err(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(format!("recipe with id {id} not found"))
            .unwrap()),
        Some(recipe) => Ok(recipe),
    }?;

    let ingredients = sqlx::query_as!(
        Ingredient,
        "SELECT id, recipe_id, position, description FROM ingredients WHERE recipe_id = ?",
        id
    )
    .fetch_all(&db)
    .await
    .map_err(handle_internal_server_error)?;

    let instructions = sqlx::query_as!(
        Instruction,
        "SELECT id, recipe_id, position, description FROM instructions WHERE recipe_id = ?",
        id
    )
    .fetch_all(&db)
    .await
    .map_err(handle_internal_server_error)?;

    Ok(Recipe::new(recipe, ingredients, instructions))
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchQueryParams {
    query: String,
    include_ingredients: bool,
    include_instructions: bool,
}

// #[get("/?<query>&<include_ingredients>&<include_instructions>")]
pub async fn search_recipes(
    State(AppState { db }): State<AppState>,
    Query(params): Query<SearchQueryParams>,
) -> Result<Json<Vec<RecipeBase>>, ErrorResponse> {
    let like_query = format!("%{}%", params.query);

    let mut builder = QueryBuilder::<Sqlite>::new(
        "SELECT id, author, name, description, difficulty, estimated_duration FROM recipes WHERE name LIKE ",
    );

    builder
        .push_bind(&like_query)
        .push(" OR description LIKE ")
        .push_bind(&like_query);

    let mut other_ids: HashSet<i64> = HashSet::new();

    if params.include_ingredients {
        let ids = sqlx::query_as!(
            Ingredient,
            "SELECT * FROM ingredients WHERE description LIKE ?",
            like_query
        )
        .fetch_all(&db)
        .await
        .map(|ingredients| {
            ingredients
                .into_iter()
                .map(|ingredient| ingredient.recipe_id)
                .collect::<Vec<_>>()
        })
        .map_err(handle_internal_server_error)?;

        other_ids.extend(ids);
    }

    if params.include_instructions {
        let ids = sqlx::query_as!(
            Instruction,
            "SELECT * FROM instructions WHERE description LIKE ?",
            like_query
        )
        .fetch_all(&db)
        .await
        .map(|instructions| {
            instructions
                .into_iter()
                .map(|instruction| instruction.recipe_id)
                .collect::<Vec<_>>()
        })
        .map_err(handle_internal_server_error)?;

        other_ids.extend(ids);
    }

    if !other_ids.is_empty() {
        builder
            .push(" OR id IN ")
            .push_tuples(other_ids, |mut b, id| {
                b.push_bind(id);
            });
    }

    builder.push(" ORDER BY id ASC");

    let query = builder.build_query_as::<RecipeBase>();

    let results = query.fetch_all(&db).await.map_err(|err| {
        error!("{err}");

        handle_internal_server_error(err)
    })?;

    Ok(Json(results))
}

pub async fn create_recipe(
    State(AppState { db }): State<AppState>,
    Json(recipe): Json<CreateRecipeRequest>,
) -> Result<Json<Recipe>, ErrorResponse> {
    let mut tx = db.begin().await.map_err(handle_internal_server_error)?;

    let recipe_result = sqlx::query!(
        "INSERT INTO recipes (name, author, description, difficulty, estimated_duration) VALUES (?, ?, ?, ?, ?)",
        recipe.name,
        recipe.author,
        recipe.description,
        recipe.difficulty,
        recipe.estimated_duration
    ).execute(&mut *tx)
    .await
    .map_err(handle_internal_server_error)?;

    let id = recipe_result.last_insert_rowid();

    if !recipe.ingredients.is_empty() {
        let mut builder = QueryBuilder::<Sqlite>::new(
            "INSERT INTO ingredients (recipe_id, position, description) ",
        );
        builder.push_values(recipe.ingredients, |mut b, ingredient| {
            b.push_bind(id)
                .push_bind(ingredient.position)
                .push_bind(ingredient.description);
        });

        let query = builder.build();
        query
            .execute(&mut *tx)
            .await
            .map_err(handle_internal_server_error)?;
    }

    if !recipe.instructions.is_empty() {
        let mut builder = QueryBuilder::<Sqlite>::new(
            "INSERT INTO instructions (recipe_id, position, description) ",
        );
        builder.push_values(recipe.instructions, |mut b, instruction| {
            b.push_bind(id)
                .push_bind(instruction.position)
                .push_bind(instruction.description);
        });

        let query = builder.build();
        query
            .execute(&mut *tx)
            .await
            .map_err(handle_internal_server_error)?;
    }

    tx.commit().await.map_err(handle_internal_server_error)?;

    let recipe = get_recipe(db, id).await?;
    Ok(Json(recipe))
}

pub async fn delete_recipe(
    State(AppState { db }): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let found = sqlx::query_as!(RecipeBase, "SELECT * from recipes WHERE id = ?", id)
        .fetch_optional(&db)
        .await
        .ok()
        .flatten();

    if found.is_none() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(format!("recipe with id {id} not found"))
            .unwrap();
    }

    sqlx::query!("DELETE FROM recipes WHERE id = ?", id)
        .execute(&db)
        .await
        .map(|_| {
            Response::builder()
                .status(StatusCode::NO_CONTENT)
                .body(format!("deleted recipe with id {id}"))
                .unwrap()
        })
        .unwrap_or_else(handle_internal_server_error)
}

// #[put("/<id>", data = "<recipe>")]
pub async fn update_recipe(
    State(AppState { db }): State<AppState>,
    Path(id): Path<i64>,
    Json(recipe): Json<UpdateRecipeRequest>,
) -> Result<Json<Recipe>, ErrorResponse> {
    let mut tx = db.begin().await.map_err(handle_internal_server_error)?;

    // update the main recipe contents.
    sqlx::query!(
        r#"
            UPDATE recipes
            SET name = ?, description = ?, author = ?, difficulty = ?, estimated_duration = ?
            WHERE id = ?;
        "#,
        recipe.name,
        recipe.description,
        recipe.author,
        recipe.difficulty,
        recipe.estimated_duration,
        id
    )
    .execute(&mut *tx)
    .await
    .map_err(handle_internal_server_error)?;

    // find the ids for all existing ingredients.
    let existing_ingredients = sqlx::query_as!(
        Ingredient,
        "SELECT * FROM ingredients WHERE recipe_id = ?",
        id
    )
    .fetch_all(&mut *tx)
    .await
    .map_err(handle_internal_server_error)?
    .iter()
    .map(|ingredient| ingredient.id)
    .collect::<Vec<_>>();

    // find the ids which appear in the original list, but not the new one.
    let ingredients_to_delete = items_to_delete(existing_ingredients, &recipe.ingredients);

    if !ingredients_to_delete.is_empty() {
        // delete the removed ingredients.
        let mut delete_ingredient_builder =
            QueryBuilder::<Sqlite>::new("DELETE FROM ingredients WHERE id");
        add_in_expression(&mut delete_ingredient_builder, &ingredients_to_delete);

        let delete_ingredient_query = delete_ingredient_builder.build();

        delete_ingredient_query
            .execute(&mut *tx)
            .await
            .map_err(handle_internal_server_error)?;
    }

    // find the new ingredients which have an id and update them
    let ingredients_to_update = recipe
        .ingredients
        .iter()
        .filter(|ingredient| ingredient.id.is_some())
        .collect::<Vec<_>>();

    for ingredient in ingredients_to_update.iter() {
        let id = ingredient.id.unwrap();
        sqlx::query!(
            r#"
                UPDATE ingredients
                SET position = ?, description = ?
                WHERE id = ?
            "#,
            ingredient.position,
            ingredient.description,
            id
        )
        .execute(&mut *tx)
        .await
        .map_err(handle_internal_server_error)?;
    }

    // create the ingredients which don't have an id.
    let ingredients_to_make = recipe
        .ingredients
        .iter()
        .filter(|ingredient| ingredient.id.is_none())
        .collect::<Vec<_>>();

    for ingredient in ingredients_to_make.iter() {
        sqlx::query!(
            "INSERT INTO ingredients (recipe_id, position, description) VALUES (?, ?, ?)",
            id,
            ingredient.position,
            ingredient.description
        )
        .execute(&mut *tx)
        .await
        .map_err(handle_internal_server_error)?;
    }

    let existing_instructions = sqlx::query_as!(
        Ingredient,
        "SELECT * FROM instructions WHERE recipe_id = ?",
        id
    )
    .fetch_all(&mut *tx)
    .await
    .map_err(handle_internal_server_error)?
    .iter()
    .map(|instruction| instruction.id)
    .collect::<Vec<_>>();

    // find the ids which appear in the original list, but not the new one.
    let instructions_to_delete = items_to_delete(existing_instructions, &recipe.instructions);

    if !instructions_to_delete.is_empty() {
        // delete the removed instructions.
        let mut delete_instruction_builder =
            QueryBuilder::<Sqlite>::new("DELETE FROM instructions WHERE id");
        add_in_expression(&mut delete_instruction_builder, &instructions_to_delete);

        let delete_instruction_query = delete_instruction_builder.build();

        delete_instruction_query
            .execute(&mut *tx)
            .await
            .map_err(handle_internal_server_error)?;
    }
    // find the new instructions which have an id and update them
    let instructions_to_update = recipe
        .instructions
        .iter()
        .filter(|instruction| instruction.id.is_some())
        .collect::<Vec<_>>();

    for instruction in instructions_to_update.iter() {
        let id = instruction.id.unwrap();
        sqlx::query!(
            r#"
                    UPDATE instructions
                    SET position = ?, description = ?
                    WHERE id = ?
                "#,
            instruction.position,
            instruction.description,
            id
        )
        .execute(&mut *tx)
        .await
        .map_err(handle_internal_server_error)?;
    }

    // create the instructions which don't have an id.
    let instructions_to_make = recipe
        .instructions
        .iter()
        .filter(|instruction| instruction.id.is_none())
        .collect::<Vec<_>>();

    for instruction in instructions_to_make.iter() {
        sqlx::query!(
            "INSERT INTO instructions (recipe_id, position, description) VALUES (?, ?, ?)",
            id,
            instruction.position,
            instruction.description
        )
        .execute(&mut *tx)
        .await
        .map_err(handle_internal_server_error)?;
    }

    tx.commit().await.map_err(handle_internal_server_error)?;

    let recipe = get_recipe(db, id).await?;

    Ok(Json(recipe))
}

trait MaybeIdentifiable {
    fn id(&self) -> Option<i64>;
}

impl MaybeIdentifiable for UpdateIngredientRequest {
    fn id(&self) -> Option<i64> {
        self.id
    }
}

impl MaybeIdentifiable for UpdateInstructionRequest {
    fn id(&self) -> Option<i64> {
        self.id
    }
}

fn items_to_delete<T: MaybeIdentifiable>(original: Vec<i64>, updated: &[T]) -> Vec<i64> {
    let updated_ids = updated
        .iter()
        .filter_map(|ingredient| ingredient.id())
        .collect::<Vec<_>>();

    original
        .into_iter()
        .filter(|id| !updated_ids.contains(id))
        .collect::<Vec<_>>()
}

fn handle_internal_server_error(err: impl std::error::Error) -> Response<String> {
    error!("{err}");

    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("Something bad happened, please try again or ask your IT guru for help.".to_owned())
        .unwrap()
}
