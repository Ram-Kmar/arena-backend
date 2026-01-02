mod db;
mod models;
mod routes;

use axum::routing::{Router, get};
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
        .with_state(db);

    let port = std::env::var("PORT").unwrap_or("3000".into());
    let addr = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
