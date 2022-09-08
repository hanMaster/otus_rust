pub mod errors;
pub mod store;

use crate::errors::AppError::{
    AddDeviceError, AddRoomError, GetHouseError, RemoveDeviceError, RemoveRoomError,
};
use errors::Result;
use http_api::house::House;
use models::device::{AddDevice, DeviceType};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct HttpClient {
    base_url: String,
    house: House,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Items(Vec<String>);

impl HttpClient {
    pub fn new(url: &str) -> Result<Self> {
        let mut client = HttpClient {
            base_url: url.to_string(),
            ..Default::default()
        };
        client.get_house_structure()?;
        Ok(client)
    }

    pub fn add_room(&mut self, room: &str) -> Result<()> {
        let url = format!("{}{}{}", &self.base_url, "house/add-room/", room);
        reqwest::blocking::get(url).map_err(|_| AddRoomError)?;
        Ok(())
    }

    pub fn add_device(&mut self, room: &str, device: &str, device_type: DeviceType) -> Result<()> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}{}", &self.base_url, "house/add-device");
        let device = AddDevice {
            room_name: room.to_string(),
            device_name: device.to_string(),
            device_type,
        };
        client
            .post(url)
            .json(&device)
            .send()
            .map_err(|_| AddDeviceError)?;
        Ok(())
    }

    pub fn remove_room(&mut self, room: &str) -> Result<()> {
        let url = format!("{}{}{}", &self.base_url, "house/remove-room/", room);
        reqwest::blocking::get(url).map_err(|_| RemoveRoomError)?;
        Ok(())
    }

    pub fn remove_device(&mut self, room: &str, device: &str) -> Result<()> {
        let url = format!(
            "{}{}{}{}{}",
            &self.base_url, "house/remove-device/", room, "/", device
        );
        reqwest::blocking::get(url).map_err(|_| RemoveDeviceError)?;
        Ok(())
    }

    pub fn get_house_structure(&mut self) -> Result<()> {
        let url = format!("{}{}", &self.base_url, "house");
        let house: House = reqwest::blocking::get(url)?
            .json()
            .map_err(|_| GetHouseError)?;
        self.house = house;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_rooms() -> Result<()> {
        let mut client = HttpClient::new("http://localhost:8080/")?;
        println!("Start: {:?}", client.house);
        client.add_room("bedroom")?;
        client.get_house_structure()?;
        println!("After add room: {:?}", client.house);
        client.add_device("bedroom", "sock", DeviceType::Socket)?;
        client.get_house_structure()?;
        println!("After add device: {:?}", client.house);
        client.remove_device("bedroom", "sock")?;
        client.get_house_structure()?;
        println!("After remove device: {:?}", client.house);
        client.remove_room("bedroom")?;
        client.get_house_structure()?;
        println!("After remove room: {:?}", client.house);
        Ok(())
    }
}
