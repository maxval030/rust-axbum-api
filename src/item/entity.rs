use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub _id: String,
    pub name: String,
    pub description: String,
    pub price: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemBson {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub price: f32,
}

//ควรไปอยู่ที่ Model
#[derive(Debug, Serialize, Deserialize)]
pub struct InsertItemReq {
    pub name: String,
    pub description: String,
    pub price: f32,
}

impl Item {
    pub fn new() -> Self {
        Item {
            _id: String::from(""),
            name: String::from(""),
            description: String::from(""),
            price: 0.0,
        }
    }
}
