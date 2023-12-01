use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    model::{ItemModel, ItemModelResponse},
    schema::{CreateItemSchema, FilterOptions, UpdateItemSchema},
    AppState,
};

pub async fn item_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;
    let table_number = opts.table_number;

    let items = if let Some(table_number) = table_number {
        sqlx::query_as!(
            ItemModel,
            r#"SELECT * FROM items WHERE table_number = ? ORDER by id LIMIT ? OFFSET ?"#,
            table_number,
            limit as i32,
            offset as i32
        )
            .fetch_all(&data.db)
            .await
    } else {
        sqlx::query_as!(
            ItemModel,
            r#"SELECT * FROM items ORDER by id LIMIT ? OFFSET ?"#,
            limit as i32,
            offset as i32
        )
            .fetch_all(&data.db)
            .await
    }
        .map_err(|e| {
            let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let item_responses = items
        .iter()
        .map(|item| filter_db_record(&item))
        .collect::<Vec<ItemModelResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results": item_responses.len(),
        "items": item_responses
    });

    Ok(Json(json_response))
}

pub async fn create_item_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateItemSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = uuid::Uuid::new_v4().to_string();
    let query_result =
        sqlx::query(r#"INSERT INTO items (id,name,table_number,preparation_time_minutes) VALUES (?, ?, ?, ?)"#)
            .bind(user_id.clone())
            .bind(body.name.to_string())
            .bind(body.table_number)
            .bind(body.preparation_time_minutes.to_string())
            .execute(&data.db)
            .await
            .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Item with that name already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    let item = sqlx::query_as!(ItemModel, r#"SELECT * FROM items WHERE id = ?"#, user_id)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            )
        })?;

    let item_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "item": filter_db_record(&item)
    })});

    Ok(Json(item_response))
}

pub async fn get_item_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        ItemModel,
        r#"SELECT * FROM items WHERE id = ?"#,
        id.to_string()
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(item) => {
            let item_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "item": filter_db_record(&item)
            })});

            return Ok(Json(item_response));
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Item with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    };
}

pub async fn edit_item_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateItemSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        ItemModel,
        r#"SELECT * FROM items WHERE id = ?"#,
        id.to_string()
    )
        .fetch_one(&data.db)
        .await;

    let item = match query_result {
        Ok(item) => item,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Item with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    };


    let update_result = sqlx::query(
        r#"UPDATE items SET name = ?, table_number = ?, preparation_time_minutes = ? WHERE id = ?"#,
    )
        .bind(body.name.to_owned().unwrap_or_else(|| item.name.clone()))
        .bind(
            body.table_number
        )
        .bind(
            body.preparation_time_minutes
        )
        .bind(id.to_string())
        .execute(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            )
        })?;

    if update_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let updated_item = sqlx::query_as!(
        ItemModel,
        r#"SELECT * FROM items WHERE id = ?"#,
        id.to_string()
    )
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            )
        })?;

    let item_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "item": filter_db_record(&updated_item)
    })});

    Ok(Json(item_response))
}

pub async fn delete_item_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query!(r#"DELETE FROM items WHERE id = ?"#, id.to_string())
        .execute(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            )
        })?;

    if query_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}

fn filter_db_record(item: &ItemModel) -> ItemModelResponse {
    ItemModelResponse {
        id: item.id.to_owned(),
        name: item.name.to_owned(),
        table_number: item.table_number.to_owned(),
        preparation_time_minutes: item.preparation_time_minutes.to_owned(),
        created_at: item.created_at.unwrap(),
        updated_at: item.updated_at.unwrap(),
    }
}