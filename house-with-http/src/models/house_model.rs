use crate::{DeviceType, HouseRepo, Room};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct House {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub rooms: Vec<Room>,
}

impl House {
    pub async fn with_name(name: &str, repo: &HouseRepo) -> Self {
        let house_result = repo.load_house(name).await;
        match house_result {
            Ok(house) => house,
            Err(_) => {
                let new_house = Self {
                    id: None,
                    name: name.to_string(),
                    rooms: vec![],
                };
                repo.insert_house(&new_house).await.unwrap();
                repo.load_house(name).await.unwrap()
            }
        }
    }

    pub fn add_room(&mut self, room_name: &str) -> Result<(), String> {
        let room_set: HashSet<String> =
            HashSet::from_iter(self.rooms.iter().map(|r| r.name.clone()));
        if room_set.contains(room_name) {
            return Err(format!("Room {} already in list", room_name));
        } else {
            let room = Room::with_name(room_name);
            self.rooms.push(room);
        }
        Ok(())
    }

    pub fn remove_room(&mut self, room_name: &str) {
        self.rooms.retain(|r| r.name.ne(room_name));
    }

    pub fn add_device_in_room(
        &mut self,
        room_name: &str,
        device_name: &str,
        device_type: DeviceType,
    ) -> Result<(), String> {
        for item in self.rooms.iter_mut() {
            if item.name.eq(room_name) {
                if let Err(err) = item.add_device(device_name, device_type) {
                    return Err(err);
                };
            }
        }
        Ok(())
    }

    pub fn remove_device_from_room(&mut self, room_name: &str, device_name: &str) {
        for item in self.rooms.iter_mut() {
            if item.name.eq(room_name) {
                item.remove_device(device_name);
            }
        }
    }
}
