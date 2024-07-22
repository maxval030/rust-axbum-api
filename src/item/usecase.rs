use axum::{http::StatusCode, response::IntoResponse, Json};
use bson::oid::ObjectId;
use serde_json::json;

use super::{
    entity::{InsertItemReq, Item},
    repository,
};

//TODO: create usecase
pub async fn insert_item_usecase(req: InsertItemReq) -> Result<ObjectId, String> {
    repository::insert_one_item(req).await
}

pub async fn find_one_item_by_id(item_id: ObjectId) -> Result<Item, String> {
    repository::find_one_item_by_id(item_id).await
}
