use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        create_item_handler, delete_item_handler, edit_item_handler, get_item_handler, item_list_handler
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
                .patch(edit_item_handler)
                .delete(delete_item_handler),
        )

        .with_state(app_state)
}
