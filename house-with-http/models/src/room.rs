use crate::device::{Device, DeviceType};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Room {
    pub name: String,
    pub devices: Vec<Device>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddRoom {
    pub house_name: String,
    pub room_name: String,
}

impl Room {
    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            devices: vec![],
        }
    }

    pub fn add_device(&mut self, device_name: &str, device_type: DeviceType) -> Result<(), String> {
        let device_set: HashSet<String> =
            HashSet::from_iter(self.devices.iter().map(|d| d.name.clone()));
        if device_set.contains(device_name) {
            return Err(format!("Device {} already in list", device_name));
        } else {
            let device = Device::with_name_and_type(device_name, device_type);
            self.devices.push(device);
        }
        Ok(())
    }

    pub fn remove_device(&mut self, device_name: &str) {
        self.devices.retain(|d| d.name.ne(device_name));
    }
}
