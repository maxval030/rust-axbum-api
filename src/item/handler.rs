use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use bson::oid::ObjectId;
use serde_json::json;

use super::{entity::InsertItemReq, usecase};

pub async fn insert_one_item(Json(req): Json<InsertItemReq>) -> impl IntoResponse {
    match usecase::insert_item_usecase(req).await {
        Ok(object_id) => (
            StatusCode::CREATED,
            Json(json!(
               {
                  "message": format!("success insert item id: {}", object_id),
               }
            ))
            .into_response(),
        ),
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!(
                   {
                      "message": err,
                   }
                ))
                .into_response(),
            )
        }
    }
}

pub async fn find_one_item_by_id(Path(item_id): Path<String>) -> impl IntoResponse {
    let item_object_id = match ObjectId::parse_str(item_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                   "message": "Error: Passing objectID failed",
                }))
                .into_response(),
            )
        }
    };

    match usecase::find_one_item_by_id(item_object_id).await {
        Ok(r) => (StatusCode::OK, Json(json!(r)).into_response()),
        Err(err) => (
            StatusCode::NOT_FOUND,
            Json(json!(
               {
                  "message": err,
               }
            ))
            .into_response(),
        ),
    }
}
