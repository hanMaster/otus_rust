use std::sync::Arc;
use mongodb::results::UpdateResult;
use mongodb::{
    bson::{doc, ser},
    results::InsertOneResult,
    Collection, Database,
};
use crate::house::House;

#[derive(Clone)]
pub struct HouseRepo {
    collection: Collection<House>,
}

impl HouseRepo {
    pub async fn init(db: Arc<Database>) -> Self {
        let collection: Collection<House> = db.collection("houses");
        HouseRepo { collection }
    }

    pub async fn persist_house(&self, house: &House) {
        if house.id.is_some() {
            self.update_house(house).await.expect("Failed to update");
        } else {
            self.insert_house(house).await.expect("Failed to insert");
        }
    }

    pub async fn insert_house(&self, new_house: &House) -> Result<InsertOneResult, &'static str> {
        let house = self
            .collection
            .insert_one(new_house, None)
            .await
            .expect("Failed to create house");
        Ok(house)
    }

    pub async fn update_house(&self, new_house: &House) -> Result<UpdateResult, &'static str> {
        let query = doc! {"_id": new_house.id};
        let update = doc! { "$set": ser::to_bson(&new_house).expect("failed to parse update") };
        let house = self
            .collection
            .update_one(query, update, None)
            .await
            .expect("Failed to update house");
        Ok(house)
    }

    pub async fn load_house(&self, name: &str) -> Result<House, &'static str> {
        let house = self
            .collection
            .find_one(doc! {"name" : name}, None)
            .await
            .unwrap();
        house.ok_or("Failed to find house")
    }
}
