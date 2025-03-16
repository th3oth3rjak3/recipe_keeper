pub mod models;

pub mod handlers;
pub mod query_utils;

use std::env;

use axum::{
    Router,
    http::{Uri, header},
    response::{IntoResponse, Response},
    routing::get,
};

use rust_embed::RustEmbed;
use sqlx::{Sqlite, SqlitePool};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[derive(RustEmbed, Clone)]
#[folder = "static"]
struct Static;

async fn connect_to_database() -> SqlitePool {
    dotenvy::dotenv().ok();

    // Retrieve database URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Set up the connection pool
    let pool = sqlx::Pool::<Sqlite>::connect(&database_url)
        .await
        .expect("could not connect to database");

    // Set wal mode and foreign key support
    sqlx::query("PRAGMA journal_mode = WAL;")
        .execute(&pool)
        .await
        .expect("could not enable WAL mode");

    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await
        .expect("could not enable foreign keys");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migrations should have succeeded");

    pool
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.starts_with("static/") {
        path = path.replace("static/", "");
    }

    StaticFile(path)
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter("debug")
        .with_writer(std::io::stdout)
        .pretty()
        .with_ansi(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let sqlite_pool = connect_to_database().await;

    let app = Router::new()
        .route(
            "/api/recipes/{id}",
            get(handlers::get_recipe_by_id)
                .put(handlers::update_recipe)
                .delete(handlers::delete_recipe),
        )
        .route("/api/recipes", get(handlers::search_recipes))
        .fallback_service(get(static_handler))
        .with_state(AppState { db: sqlite_pool });

    let address = "0.0.0.0:8000";

    info!("Starting listener on http://{}", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Static::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => {
                let index = Static::get("index.html").unwrap();
                let mime = mime_guess::from_path("index.html").first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], index.data).into_response()
            }
        }
    }
}
