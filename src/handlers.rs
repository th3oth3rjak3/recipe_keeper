use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    Json,
};

use crate::{AppError, AppState, Ingredient, Instruction, Recipe, RecipeBase};

#[axum::debug_handler]
pub async fn get_recipe_by_id(
    Path(id): Path<i64>,
    State(app_state): State<AppState>,
) -> Result<Json<Recipe>, AppError> {
    let maybe_recipe = sqlx::query_as!(
        RecipeBase,
        "SELECT id, name, author, description, difficulty, estimated_duration FROM recipes WHERE id = ?",
        id)
    .fetch_optional(&app_state.pool)
    .await
    .map_err(|err| AppError::InternalServer(anyhow!(err)))?;

    let recipe = match maybe_recipe {
        Some(recipe_base) => Ok(recipe_base),
        None => Err(AppError::NotFound(anyhow!("Recipe not found"))),
    }?;

    let ingredients = sqlx::query_as!(
        Ingredient,
        "SELECT id, recipe_id, position, description FROM ingredients WHERE recipe_id = ?",
        id
    )
    .fetch_all(&app_state.pool)
    .await
    .map_err(|err| AppError::InternalServer(anyhow!(err)))?;

    let instructions = sqlx::query_as!(
        Instruction,
        "SELECT id, recipe_id, position, description FROM instructions WHERE recipe_id = ?",
        id
    )
    .fetch_all(&app_state.pool)
    .await
    .map_err(|err| AppError::InternalServer(anyhow!(err)))?;

    Ok(Json(Recipe::new(recipe, ingredients, instructions)))
}

pub fn search_recipes(
    query: String,
    include_ingredients: bool,
    include_instructions: bool,
) -> Result<Json<Vec<RecipeBase>>, AppError> {
    Err(AppError::NotImplemented(anyhow!(
        "search_recipes not implemented"
    )))
}
