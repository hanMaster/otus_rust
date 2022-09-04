use crate::house::{House, NewHouse, NewRoom, Room};
use mongodb::results::UpdateResult;
use mongodb::{bson::doc, Database};

const HOUSE_COLLECTION: &str = "houses";

impl NewHouse {
    pub async fn save(&self, mongo: &Database) -> Result<(), String> {
        mongo
            .collection::<NewHouse>(HOUSE_COLLECTION)
            .insert_one(self, None)
            .await
            .map_err(|err| format!("DB_ERROR: {:?}", err))?;
        Ok(())
    }
}

impl House {
    pub async fn get_by_name(name: &str, mongo: &Database) -> Result<Self, String> {
        let house = mongo
            .collection::<House>(HOUSE_COLLECTION)
            .find_one(doc! {"name": name}, None)
            .await
            .expect("House not found");
        Ok(house.unwrap())
    }
}

// impl NewRoom {
//     pub async fn save(&self, mongo: &Database) -> Result<(), String> {
//         let mut house = House::get_by_name(self.house_name.as_str(), &mongo).await?;
//         house.rooms.insert(
//             0,
//             Room {
//                 name: self.room_name.clone(),
//                 devices: vec![],
//             },
//         );
//         mongo
//             .collection::<House>(HOUSE_COLLECTION)
//             .update_one(doc! {"name": &house.name}, house, None)
//             .await
//             .map_err(|err| format!("DB_ERROR: {:?}", err))?;
//         Ok(())
//     }
// }
