use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
