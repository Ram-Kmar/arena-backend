use serde::{Serialize,Deserialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}