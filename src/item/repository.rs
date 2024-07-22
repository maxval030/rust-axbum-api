use crate::config::db::db_connect;

use super::entity::{InsertItemReq, Item, ItemBson};
use bson::{from_document, oid::ObjectId};
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

pub async fn find_one_item_by_id(id: ObjectId) -> Result<Item, String> {
    let db = match db_connect().await {
        Ok(r) => r,
        Err(e) => panic!("Error: {}", e),
    };

    let col = db.collection::<Document>("item");

    let cursor = col.find_one(doc! {"_id":id}).await;

    let doc = match cursor {
        Ok(r) => r,
        Err(e) => {
            error!("Error: {:?}", e);
            return Result::Err(format!("Error: {:?}", e));
        }
    };

    let item: ItemBson = match doc {
        Some(r) => match from_document(r).map_err(|e| format!("Error: {:?}", e)) {
            Ok(i) => i,
            Err(e) => {
                error!("Error: {}", e);
                return Result::Err(format!("Error: {}", e));
            }
        },
        None => {
            error!("Error: element not found");
            return Result::Err(format!("Error: element not found"));
        }
    };

    Ok(Item {
        _id: item._id.to_hex(),
        name: item.name,
        description: item.description,
        price: item.price,
    })
}
