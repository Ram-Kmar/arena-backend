mod db;
mod models;
mod routes;

use std::env;

use axum::{
    routing::{Router, get},
    extract::Request,
    http::{StatusCode, HeaderMap},
    middleware::{self, Next},
    response::Response,
};

use db::init_db;
use dotenvy::dotenv;
use routes::{create_user, get_users};
use tokio::net::TcpListener;
// Ensure you have this import to run the query
use sqlx::query; 

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // 1. Initialize the pool
    let db = init_db().await;

    // 2. CHECK CONNECTION HERE
    // This runs a dummy query. If it fails, the app crashes immediately (which is what you want).
    match query("SELECT 1").execute(&db).await {
        Ok(_) => println!("✅ Connected to Neon database successfully!"),
        Err(e) => {
            eprintln!("❌ Failed to connect to Neon: {}", e);
            std::process::exit(1);
        }
    }

    let app = Router::new()
        .route("/users", get(get_users).post(create_user))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(db);

    let port = std::env::var("PORT").unwrap_or("3000".into());
    let addr = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // A. Get the key from the environment (or default to a test key)
    let api_secret = env::var("API_SECRET").unwrap_or_else(|_| "my-secret-key".to_string());

    // B. Check if the "x-api-key" header exists and matches
    match headers.get("x-api-key") {
        Some(key) if key == &api_secret => {
            // Key matches! Pass the request to the next handler
            Ok(next.run(request).await)
        }
        _ => {
            // Key is missing or wrong -> Return 401 Unauthorized
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}