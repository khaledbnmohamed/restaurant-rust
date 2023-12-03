use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        create_item_handler, delete_item_handler, get_item_handler, item_list_handler, get_items_for_table_handler
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/items/", post(create_item_handler))
        .route("/api/items", get(item_list_handler))
        .route(
            "/api/items/:id",
            get(get_item_handler)
                .delete(delete_item_handler),
        )
        .route("/api/tables/:table_number/items", get(get_items_for_table_handler)) // New route

        .with_state(app_state)
}
