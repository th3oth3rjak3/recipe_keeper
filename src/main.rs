#[macro_use]
extern crate rocket;

pub mod models;
use handlers::recipe_routes;
pub use models::*;
pub mod handlers;
pub mod query_utils;

use rocket_db_pools::{
    Database,
    sqlx::{self},
};

#[derive(Database)]
#[database("recipe_keeper")]
pub struct RecipeKeeper(sqlx::SqlitePool);

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/api/recipes", recipe_routes())
        .attach(RecipeKeeper::init())
}
