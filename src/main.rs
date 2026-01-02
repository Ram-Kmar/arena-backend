mod db;
mod routes;
mod models;

use axum::{routing::{get,post}, Router};
use db::init_db;
use routes::{get_users, create_user};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let db = init_db().await;
    let app = Router::new().route("/users", get(get_users).post(create_user)).with_state(db);

    // let app = Router::new()
    //     .route("/users", get(get_users))
    //     .with_state(db);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
