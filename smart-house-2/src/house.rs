use crate::device::Info;
use std::collections::{hash_map::Iter, HashMap, HashSet};

pub type RoomName = String;
pub type DeviceList = HashSet<String>;

pub struct House {
    name: String,
    rooms: HashMap<RoomName, DeviceList>,
}

impl Default for House {
    fn default() -> Self {
        Self::with_name("house1")
    }
}

impl House {
    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rooms: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_mut_devices_list_for_room(&mut self, room_name: &str) -> &mut DeviceList {
        self.rooms
            .entry(String::from(room_name))
            .or_insert_with(HashSet::new)
    }

    pub fn remove_room(&mut self, room_name: &str) -> Option<DeviceList> {
        self.rooms.remove(room_name)
    }

    pub fn get_rooms_list(&self) -> Iter<RoomName, DeviceList> {
        self.rooms.iter()
    }

    pub fn summary<T: Info>(&self, store: T) {
        println!("House devices summary:");
        self.get_rooms_list().for_each(|room| {
            let room_name = room.0;
            let devices = room.1;
            devices.iter().for_each(|device_name| {
                println!(
                    "Room: {}, Device: {}, Status: {}",
                    room_name,
                    device_name,
                    store.get_info_by_room_and_device(room_name, device_name)
                );
            })
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_name() {
        let house = House::with_name("Test house");
        assert_eq!("Test house", house.get_name());
    }

    #[test]
    fn test_add_remove_room() {
        let mut house = House::with_name("Test house");

        house.get_mut_devices_list_for_room("room1");
        assert_eq!(1, house.rooms.len());

        house.get_mut_devices_list_for_room("room2");
        assert_eq!(2, house.rooms.len());

        let result = house.remove_room("abc");
        assert_eq!(2, house.rooms.len());
        assert_eq!(None, result);

        house.remove_room("room2");
        assert_eq!(1, house.rooms.len());
    }

    #[test]
    // cargo test -- --show-output
    fn test_rooms_list() {
        let mut house = House::with_name("Test house");

        house.get_mut_devices_list_for_room("room1");
        house.get_mut_devices_list_for_room("room2");
        for room_name in house.get_rooms_list().map(|item| item.0) {
            println!("Room: {room_name}");
        }
    }
}
