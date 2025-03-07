use anyhow::Error;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use handlers::get_recipe_by_id;
use sqlx::SqlitePool;
use tracing::info;

pub mod handlers;
pub mod models;
pub use models::*;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

#[derive(Debug)]
pub enum AppError {
    InternalServer(anyhow::Error),
    NotFound(anyhow::Error),
    BadRequest(anyhow::Error),
    NotImplemented(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::InternalServer(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            }
            AppError::NotFound(err) => (StatusCode::NOT_FOUND, err.to_string()).into_response(),
            AppError::BadRequest(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
            AppError::NotImplemented(err) => {
                (StatusCode::NOT_IMPLEMENTED, err.to_string()).into_response()
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let pool = SqlitePool::connect("sqlite://RecipeKeeper.db3?mode=rwc")
        .await
        .expect("Could not initialize database");

    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await?;

    sqlx::query("PRAGMA journal_mode = WAL;")
        .execute(&pool)
        .await?;

    // build our application with a single route
    let app = Router::new()
        .route("/", get(say_hello))
        .route("/recipes/{id}", get(get_recipe_by_id))
        .with_state(AppState { pool: pool.clone() });

    let addr = "127.0.0.1:3000";

    // run our app with hyper, listening globally on port 3000
    info!("application running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

pub async fn say_hello() -> String {
    "Hello, World!".into()
}
