pub mod errors;

use crate::errors::AppError::{GetDevicesError, GetRoomsError};
use errors::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default)]
pub struct HttpClient {
    rooms: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Items(Vec<String>);

impl HttpClient {
    pub fn new() -> Result<Self> {
        let mut client = HttpClient::default();
        client.request_rooms()?;
        client.request_devices()?;
        Ok(client)
    }

    fn request_rooms(&mut self) -> Result<()> {
        let rooms = reqwest::blocking::get("http://localhost:8080/house/rooms")?
            .json::<Items>()
            .or_else(|_|{ Err(GetRoomsError)})?;
        self.rooms.clear();
        for room in rooms.0.iter() {
            self.rooms.insert(room.clone(), vec![]);
        }
        Ok(())
    }

    fn request_devices(&mut self) -> Result<()> {
        for room in self.rooms.iter_mut() {
            let devices: Items =
                reqwest::blocking::get("http://localhost:8080/house/devices/".to_owned() + room.0)?
                    .json().or_else(|_|{ Err(GetDevicesError)})?;
            for device in devices.0.iter() {
                room.1.push(device.clone());
            }
        }
        Ok(())
    }

    pub fn add_room(&mut self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_rooms() -> Result<()> {
        let client = HttpClient::new()?;
        println!("{:?}", client.rooms);
        Ok(())
    }
}
