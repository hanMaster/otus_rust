use crate::device::Device;
use std::collections::HashMap;

pub struct Room {
    devices: HashMap<String, Device>,
}

impl Default for Room {
    fn default() -> Self {
        Self::new()
    }
}

impl Room {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, device_name: &str, device: Device) {
        self.devices.insert(String::from(device_name), device);
    }

    pub fn remove_device(&mut self, device_name: &str) {
        self.devices.remove(device_name);
    }

    pub fn get_devices_list(&self) -> Vec<String> {
        let mut res = vec![];
        for r in &self.devices {
            res.push(r.0.clone());
        }
        res
    }
}
