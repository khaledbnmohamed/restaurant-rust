use crate::{create_mysql_pool, setup_in_memory_database};
use axum::{http::Request, Router};
use sqlx::SqlitePool;

#[tokio::test]
async fn test_create_item() {
    // Setup in-memory database for testing
    let db_pool: SqlitePool = setup_in_memory_database().await;

    // Setup MySQL connection pool for the actual application
    let mysql_pool = create_mysql_pool().await;

    // Create a test instance of the app, passing the in-memory database pool
    let app = create_router(Arc::new(AppState { db: mysql_pool }));

    // ... rest of your test code
}
