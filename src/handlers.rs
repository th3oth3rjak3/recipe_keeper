use std::{collections::HashSet, error::Error};

use rocket::{Route, http::Status, response::status::Custom, serde::json::Json};
use rocket_db_pools::Connection;
use sqlx::{Acquire, QueryBuilder, Sqlite};

use crate::{
    CreateRecipeRequest, Ingredient, Instruction, Recipe, RecipeBase, RecipeKeeper,
    UpdateIngredientRequest, UpdateInstructionRequest, UpdateRecipeRequest,
};

use crate::query_utils::add_in_expression;

#[get("/<id>")]
async fn get_recipe_by_id(
    db: Connection<RecipeKeeper>,
    id: i64,
) -> Result<Json<Recipe>, Custom<String>> {
    get_recipe(db, id).await.map(Json)
}

async fn get_recipe(mut db: Connection<RecipeKeeper>, id: i64) -> Result<Recipe, Custom<String>> {
    let maybe_recipe = sqlx::query_as!(
        RecipeBase,
        "SELECT id, name, author, description, difficulty, estimated_duration FROM recipes WHERE id = ?",
        id)
    .fetch_optional(&mut **db)
    .await
    .map_err(handle_internal_server_error)?;

    let recipe = match maybe_recipe {
        None => Err(Custom(
            Status::NotFound,
            format!("recipe with id {id} not found"),
        )),
        Some(recipe) => Ok(recipe),
    }?;

    let ingredients = sqlx::query_as!(
        Ingredient,
        "SELECT id, recipe_id, position, description FROM ingredients WHERE recipe_id = ?",
        id
    )
    .fetch_all(&mut **db)
    .await
    .map_err(handle_internal_server_error)?;

    let instructions = sqlx::query_as!(
        Instruction,
        "SELECT id, recipe_id, position, description FROM instructions WHERE recipe_id = ?",
        id
    )
    .fetch_all(&mut **db)
    .await
    .map_err(handle_internal_server_error)?;

    Ok(Recipe::new(recipe, ingredients, instructions))
}

#[get("/?<query>&<include_ingredients>&<include_instructions>")]
async fn search_recipes(
    mut db: Connection<RecipeKeeper>,
    query: &str,
    include_ingredients: bool,
    include_instructions: bool,
) -> Result<Json<Vec<RecipeBase>>, Custom<String>> {
    let like_query = format!("%{}%", query);

    let mut builder = QueryBuilder::<Sqlite>::new(
        "SELECT id, author, name, description, difficulty, estimated_duration FROM recipes WHERE name LIKE ",
    );

    builder
        .push_bind(&like_query)
        .push(" OR description LIKE ")
        .push_bind(&like_query);

    let mut other_ids: HashSet<i64> = HashSet::new();

    if include_ingredients {
        let ids = sqlx::query_as!(
            Ingredient,
            "SELECT * FROM ingredients WHERE description LIKE ?",
            like_query
        )
        .fetch_all(&mut **db)
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

    if include_instructions {
        let ids = sqlx::query_as!(
            Instruction,
            "SELECT * FROM instructions WHERE description LIKE ?",
            like_query
        )
        .fetch_all(&mut **db)
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

    let results = query.fetch_all(&mut **db).await.map_err(|err| {
        error!("{err}");
        Custom(Status::InternalServerError, err.to_string())
    })?;

    Ok(Json(results))
}

#[post("/", data = "<recipe>")]
async fn create_recipe(
    mut db: Connection<RecipeKeeper>,
    recipe: Json<CreateRecipeRequest>,
) -> Result<Json<Recipe>, Custom<String>> {
    let mut tx = db.begin().await.map_err(handle_internal_server_error)?;
    let recipe = recipe.into_inner();

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

#[delete("/<id>")]
async fn delete_recipe(mut db: Connection<RecipeKeeper>, id: i64) -> Custom<String> {
    let found = sqlx::query_as!(RecipeBase, "SELECT * from recipes WHERE id = ?", id)
        .fetch_optional(&mut **db)
        .await
        .ok()
        .flatten();

    if found.is_none() {
        return Custom(Status::NotFound, format!("recipe with id {id} not found"));
    }

    sqlx::query!("DELETE FROM recipes WHERE id = ?", id)
        .execute(&mut **db)
        .await
        .map(|_| Custom(Status::NoContent, format!("deleted recipe with id {id}")))
        .unwrap_or_else(handle_internal_server_error)
}

#[put("/<id>", data = "<recipe>")]
async fn update_recipe(
    mut db: Connection<RecipeKeeper>,
    id: i64,
    recipe: Json<UpdateRecipeRequest>,
) -> Result<Json<Recipe>, Custom<String>> {
    let mut tx = db.begin().await.map_err(handle_internal_server_error)?;
    let recipe = recipe.into_inner();

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

pub fn recipe_routes() -> Vec<Route> {
    routes![
        create_recipe,
        search_recipes,
        get_recipe_by_id,
        delete_recipe,
        update_recipe
    ]
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

fn handle_internal_server_error(err: impl Error) -> Custom<String> {
    error!("{err}");
    Custom(
        Status::InternalServerError,
        String::from("Something bad happened, please try again or ask your IT guru for help."),
    )
}
