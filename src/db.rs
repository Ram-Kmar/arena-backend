use sqlx::SqlitePool;

pub async fn init_db() -> SqlitePool {
    SqlitePool::connect("sqlite:app.db")
        .await
        .expect("Failed to connect to DB")
}

