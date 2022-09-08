use models::device::{Device, Info};
use std::collections::HashMap;

type DeviceName = String;
type RoomName = String;

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
    fn get_info_by_room_and_device(&self, room_name: &str, device_name: &str) -> Option<String> {
        if let Some(device) = self
            .devices
            .get(&(room_name.to_string(), device_name.to_string()))
        {
            Some(device.get_info())
        } else {
            None
        }
    }
}
