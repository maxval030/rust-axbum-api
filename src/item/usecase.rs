use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use super::{entity::InsertItemReq, repository};

//TODO: create usecase
pub async fn insert_item_usecase(req: InsertItemReq) -> impl IntoResponse {
    let result_object_id = repository::insert_one_item(req).await;
    match result_object_id {
        Ok(object_id) => (
            StatusCode::CREATED,
            Json(json!(
               {
                  "message": format!("success insert item id: {}", object_id),
               }
            ))
            .into_response(),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!(
               {
                  "message": err,
               }
            ))
            .into_response(),
        ),
    }
}
