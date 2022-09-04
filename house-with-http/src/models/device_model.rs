use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<ObjectId>,
    pub name: String,
    pub device_type: DeviceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceType {
    Socket,
    Thermometer,
}