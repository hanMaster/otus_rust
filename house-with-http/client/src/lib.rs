pub mod errors;

use crate::errors::AppError::{GetDevicesError, GetRoomsError};
use errors::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use models::device::{AddDevice, DeviceType};

#[derive(Default)]
pub struct HttpClient {
    base_url: String,
    rooms: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Items(Vec<String>);

impl HttpClient {
    pub fn new(url: &str) -> Result<Self> {
        let mut client = HttpClient {
            base_url: url.to_string(),
            ..Default::default()
        };
        client.load_house()?;
        Ok(client)
    }

    fn request_rooms(&mut self) -> Result<()> {
        let url = format!("{}{}", &self.base_url, "house/rooms");
        let rooms = reqwest::blocking::get(url)?
            .json::<Items>()
            .map_err(|_| GetRoomsError)?;
        self.rooms.clear();
        for room in rooms.0.iter() {
            self.rooms.insert(room.clone(), vec![]);
        }
        Ok(())
    }

    fn request_devices(&mut self) -> Result<()> {
        for room in self.rooms.iter_mut() {
            let url = format!("{}{}{}", &self.base_url, "house/devices/", &room.0);
            let devices: Items = reqwest::blocking::get(url)?
                .json()
                .map_err(|_| GetDevicesError)?;
            for device in devices.0.iter() {
                room.1.push(device.clone());
            }
        }
        Ok(())
    }

    pub fn load_house(&mut self) -> Result<()> {
        self.request_rooms()?;
        self.request_devices()?;
        Ok(())
    }

    pub fn add_room(&mut self, room: &str) -> Result<()> {
        let url = format!("{}{}{}", &self.base_url, "house/add-room/", room);
        reqwest::blocking::get(url)?;
        Ok(())
    }

    pub fn add_device(&mut self, room: &str, device: &str, device_type: DeviceType) -> Result<()> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}{}", &self.base_url, "house/add-device");
        let device = AddDevice {
            room_name: room.to_string(),
            device_name: device.to_string(),
            device_type
        };
        client.post(url).json(&device).send()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_rooms() -> Result<()> {
        let mut client = HttpClient::new("http://localhost:8080/")?;
        println!("Start: {:?}", client.rooms);
        client.add_room("bedroom")?;
        client.load_house()?;
        println!("After add room: {:?}", client.rooms);
        client.add_device("bedroom", "sock", DeviceType::Socket)?;
        client.load_house()?;
        println!("After add device: {:?}", client.rooms);

        Ok(())
    }
}
