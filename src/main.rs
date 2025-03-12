#[macro_use]
extern crate rocket;

pub mod models;
use handlers::recipe_routes;
pub use models::*;
pub mod handlers;
pub mod query_utils;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_db_pools::{
    Database,
    sqlx::{self},
};

#[derive(Database)]
#[database("recipe_keeper")]
pub struct RecipeKeeper(sqlx::SqlitePool);

#[launch]
async fn rocket() -> _ {
    // TODO: remove cors once self-hosted.
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Delete,
            Method::Put,
            Method::Options,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build()
        .attach(RecipeKeeper::init())
        .attach(cors)
        .mount("/api/recipes", recipe_routes())
}
