use crate::device::Device;
use crate::room::Room;
use std::collections::hash_map::Iter;
use std::collections::HashMap;

pub struct House {
    house_name: String,
    rooms: HashMap<String, Room>,
}

impl House {
    pub fn new(name: String) -> Self {
        Self {
            house_name: name,
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room_name: &str, room: Room) {
        self.rooms.insert(String::from(room_name), room);
    }

    pub fn remove_room(&mut self, room_name: &str) -> Option<Room> {
        self.rooms.remove(room_name)
    }

    pub fn get_name(&self) -> &str {
        self.house_name.as_str()
    }

    pub fn get_rooms_list(&self) -> Iter<String, Room> {
        self.rooms.iter()
    }

    pub fn get_room_devices(&self, room_name: &str) -> Option<Vec<String>> {
        let room = self.rooms.get(room_name);
        room.map(|r| r.get_device_names_list())
    }

    pub fn get_room_devices_info(&self, room_name: &str) -> Option<Vec<String>> {
        let room = self.rooms.get(room_name);
        room.map(|r| r.get_device_info_list())
    }

    pub fn set_room_device(
        &mut self,
        room_name: &str,
        device_name: &str,
        device: Device,
    ) -> Result<(), &'static str> {
        let room = self.rooms.get_mut(room_name);
        if let Some(r) = room {
            r.add_device(device_name, device);
            Ok(())
        } else {
            Err("Room not found")
        }
    }

    pub fn unset_room_device(
        &mut self,
        room_name: &str,
        device_name: &str,
    ) -> Result<(), &'static str> {
        let room = self.rooms.get_mut(room_name);
        if let Some(r) = room {
            r.remove_device(device_name);
            Ok(())
        } else {
            Err("Room not found")
        }
    }

    pub fn print_house_summary(&self) {
        println!("Device report for house: {}", self.get_name());
        let rooms = self.get_rooms_list();
        for room in rooms {
            let devices_info = self
                .get_room_devices_info(room.0)
                .expect("Unable to get devices info");

            println!("{}:", room.0);
            for item in devices_info {
                println!("{}", item);
            }
        }
    }
}
