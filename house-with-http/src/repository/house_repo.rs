use std::sync::Arc;

use crate::models::house_model::House;
use mongodb::{bson::{doc}, results::InsertOneResult, Client, Collection, Database};

#[derive(Clone)]
pub struct HouseRepo {
    collection: Collection<House>,
}

impl HouseRepo {
    pub async fn init(db: Arc<Database>) -> Self {
        let collection: Collection<House> = db.collection("houses");
        HouseRepo { collection }
    }

    pub async fn create_house(&self, new_house: House) -> Result<InsertOneResult, &'static str> {
        let house = self
            .collection
            .insert_one(new_house, None)
            .await
            .expect("Failed to create house");
        Ok(house)
    }

    pub async fn get_house(&self, name: &str) -> Result<House, &'static str> {
        let house = self
            .collection
            .find_one(doc! {"name" : name}, None)
            .await.unwrap();
        house.ok_or("Failed to find house")
    }
}
