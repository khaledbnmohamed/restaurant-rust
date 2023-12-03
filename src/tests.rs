mod config;

use axum::http::Request;
use serde_json::json;
use std::sync::Arc;
use axum::http;
use tower::ServiceExt;
use crate::AppState;
use crate::route::create_router;

use axum::Router;
use sqlx::MySqlPool;

// Define a struct to hold the shared state for tests
struct TestState {
    app: Router,
    db_pool: Arc<MySqlPool>,
}

impl TestState {
    // A function to create a new instance of the shared state
    async fn new() -> Self {
        // Set up your test database pool
        let db_pool = create_test_pool();
        // Set up your application with the test database
        let app_state = Arc::new(AppState { db: db_pool.clone() });
        let app = create_router(app_state);

        Self { app, Arc<db_pool> }
    }
}


#[tokio::test]
async fn test_create_item() {
    // Create a test instance of the app
    let test_state = TestState::new().await;

    // Create a test request with a JSON body
    let request = Request::builder()
        .uri("/api/items")
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(json!({
            "name": "Test Item",
            "table_number": "Test Table",
            "preparation_time_minutes": 10
        }))
        .unwrap().unwrap();

    // Send the request to the app
    let response = test_state
        .oneshot(request)
        .await
        .expect("Failed to execute request");

    // Assert that the response is successful (HTTP status 2xx)
    assert!(response.status().is_success());
}


// Implement create_test_pool as per your testing strategy
fn create_test_pool() -> MySqlPool {
    // Implement your test database setup here
    // You may want to use an in-memory database or another strategy suitable for testing
    // For simplicity, this example just uses the production database, which is not recommended for real testing.
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MySqlPool::connect(&database_url).expect("Failed to connect to the database")
}
