use crate::device::{Device, Info};
use crate::house::RoomName;
use std::collections::HashMap;

type DeviceName = String;

pub struct DeviceStore {
    devices: HashMap<(RoomName, DeviceName), Device>,
}

impl Default for DeviceStore {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceStore {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, room_name: &str, device_name: &str, device: Device) {
        self.devices
            .insert((room_name.to_string(), device_name.to_string()), device);
    }
}

impl Info for DeviceStore {
    fn get_info_by_room_and_device(&self, room_name: &str, device_name: &str) -> String {
        if let Some(device) = self
            .devices
            .get(&(room_name.to_string(), device_name.to_string()))
        {
            device.get_info()
        } else {
            String::from("Device not found")
        }
    }
}
