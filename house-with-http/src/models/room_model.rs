use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub house_id: ObjectId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddRoom {
    pub house_name: String,
    pub room_name: String
}