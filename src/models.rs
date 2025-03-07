use rocket_db_pools::sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RecipeBase {
    pub id: i64,
    pub author: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub estimated_duration: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub id: i64,
    pub author: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub estimated_duration: Option<String>,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<Instruction>,
}

impl Recipe {
    pub fn new(
        recipe_base: RecipeBase,
        ingredients: Vec<Ingredient>,
        instructions: Vec<Instruction>,
    ) -> Self {
        Self {
            id: recipe_base.id,
            author: recipe_base.author,
            name: recipe_base.name,
            description: recipe_base.description,
            difficulty: recipe_base.difficulty,
            estimated_duration: recipe_base.estimated_duration,
            ingredients,
            instructions,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
    pub id: i64,
    pub recipe_id: i64,
    pub position: i64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    pub id: i64,
    pub recipe_id: i64,
    pub position: i64,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecipeRequest {
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub estimated_duration: Option<String>,
    pub ingredients: Vec<CreateIngredientRequest>,
    pub instructions: Vec<CreateInstructionRequest>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngredientRequest {
    pub position: i64,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInstructionRequest {
    pub position: i64,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRecipeRequest {
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub estimated_duration: Option<String>,
    pub ingredients: Vec<UpdateIngredientRequest>,
    pub instructions: Vec<UpdateInstructionRequest>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIngredientRequest {
    pub id: Option<i64>, // if it has an id, update it otherwise create
    pub position: i64,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInstructionRequest {
    pub id: Option<i64>, // if it has an id, update it otherwise create
    pub position: i64,
    pub description: String,
}
