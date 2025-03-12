#[macro_use]
extern crate rocket;

pub mod models;

use handlers::recipe_routes;
pub use models::*;
pub mod handlers;
pub mod query_utils;

use rocket::{
    Build, Config, Request, Response, Rocket,
    fairing::{self, AdHoc, Fairing, Info, Kind},
    figment::{
        Figment,
        providers::{Format, Toml},
    },
    fs::{FileServer, NamedFile, relative},
    http::Header,
};
use rocket_db_pools::{
    Database,
    sqlx::{self},
};

const ROCKET_TOML: &str = include_str!("../Rocket.toml");

#[derive(Database)]
#[database("recipe_keeper")]
pub struct RecipeKeeper(sqlx::SqlitePool);

/// CORS Fairing to attach necessary headers for all requests
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*")); // Change * to your frontend's URL if needed
        res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, OPTIONS",
        ));
        res.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        ));
    }
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match RecipeKeeper::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

#[get("/<_..>", rank = 20)]
async fn fallback() -> Option<NamedFile> {
    NamedFile::open(relative!("static/index.html")).await.ok()
}

/// Handle OPTIONS requests (CORS preflight)
#[options("/<_..>", rank = 21)]
fn options_route() -> &'static str {
    "" // Empty response body
}

#[launch]
async fn rocket() -> _ {
    let figment = Figment::from(Config::default()).merge(Toml::string(ROCKET_TOML).nested());

    rocket::custom(figment)
        .attach(CORS)
        .attach(RecipeKeeper::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
        .mount("/", routes![options_route])
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![fallback])
        .mount("/api/recipes", recipe_routes())
}
