use crate::config::db::db_connect;

use super::entity::InsertItemReq;
use bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use std::result::Result;
use tracing::log::error;

pub async fn insert_one_item(req: InsertItemReq) -> Result<ObjectId, String> {
    let db = match db_connect().await {
        Ok(r) => r,
        Err(e) => panic!("Error: {}", e),
    };

    let col = db.collection::<Document>("item");

    let result = col
        .insert_one({
            doc! {
                "name": req.name,
                "description": req.description,
                "price": req.price
            }
        })
        .await;

    let insert_item_id_bson = match result {
        Ok(r) => r.inserted_id,
        Err(e) => {
            error!("Error: {}", e);
            return Result::Err(format!("Error: {}", e));
        }
    };

    match insert_item_id_bson.as_object_id() {
        Some(r) => return Result::Ok(r),
        None => {
            error!("Error: Passed value is not an ObjectId");
            return Result::Err(format!("Error:"));
        }
    }
}
