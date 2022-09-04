use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewHouse {
    name: String,
}



#[derive(Serialize, Deserialize)]
pub struct NewRoom {
    pub house_name: String,
    pub room_name: String,
}


