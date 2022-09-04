use dotenv::dotenv;
use std::env;
use std::sync::Arc;

use crate::models::room_model::Room;
use mongodb::{bson::{doc, extjson::de::Error}, results::InsertOneResult, Client, Collection, Database};

#[derive(Clone)]
pub struct RoomRepo {
    collection: Collection<Room>,
}

impl RoomRepo {
    pub async fn init(db: Arc<Database>) -> Self {
        let collection: Collection<Room> = db.collection("rooms");
        RoomRepo { collection }
    }

    pub async fn create_room(&self, new_room: Room) -> Result<InsertOneResult, Error> {
        let room = self
            .collection
            .insert_one(new_room, None)
            .await
            .expect("Error creating room");
        Ok(room)
    }

    pub async fn get_room(&self, name: &str) -> Result<Room, Error> {
        let room = self
            .collection
            .find_one(doc! {"name" : name}, None)
            .await
            .expect("Failed to find house");
        Ok(room.unwrap())
    }
}
