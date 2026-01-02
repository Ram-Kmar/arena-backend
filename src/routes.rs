use axum::{extract::State, Json};
use sqlx::SqlitePool;

use crate::models::User;

pub async fn get_users(
    State(db): State<SqlitePool>,
) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>(
        "SELECT id, username FROM users"
    )
    .fetch_all(&db)
    .await
    .unwrap();

    Json(users)
}

pub async fn create_user(
    State(db): State<SqlitePool>,
    Json(payload): Json<crate::models::CreateUser>,
) -> Json<User> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username) VALUES (?) RETURNING id, username"
    )
    .bind(payload.username)
    .fetch_one(&db)
    .await
    .unwrap();

    Json(user)
}
