use axum::{response::IntoResponse, Json};
use tracing::info;

use super::{entity::InsertItemReq, usecase};

pub async fn insert_one_item(Json(req): Json<InsertItemReq>) -> impl IntoResponse {
    usecase::insert_item_usecase(req).await
}
