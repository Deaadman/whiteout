use std::time::Instant;
use axum::{routing::get, Extension, Json, Router};
use axum::http::StatusCode;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&url).await?;
    info!("Connected to the database!");

    let app = Router::new()
        .route("/", get(root))
        .route("/mods", get(get_mods))
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("localhost:5001").await?;
    info!("Server is running on http://localhost:5001");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize, Deserialize)]
struct Mod {
    id: i32,
    name: String,
}

async fn get_mods(Extension(pool): Extension<Pool<Postgres>>) -> Result<Json<Vec<Mod>>, StatusCode> {
    let start = Instant::now();

    let mods = sqlx::query_as!(Mod, "SELECT id, name FROM mods")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let duration = start.elapsed();
    info!("Time elapsed in get_mods() was: {:?}", duration);

    Ok(Json(mods))
}