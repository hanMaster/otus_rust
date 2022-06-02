use crate::device::{Device, Info};
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

    pub fn get_device_names_list(&self) -> Vec<String> {
        let mut res = vec![];
        for r in &self.devices {
            res.push(r.0.clone());
        }
        res
    }

    pub fn get_device_info_list(&self) -> Vec<String> {
        let mut res = vec![];
        for r in &self.devices {
            let device = r.1;
            res.push(device.get_info());
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::switcher::Switcher;

    #[test]
    fn test_add_remove_devices() {
        let mut room = Room::new();
        let sw = Device::DevSwitcher(Switcher::new());
        room.add_device("switch1", sw);
        assert_eq!(1, room.devices.len());
        room.remove_device("switch1");
        assert_eq!(0, room.devices.len());
    }

    #[test]
    fn test_get_device_names_list() {
        let mut room = Room::new();
        let sw = Device::DevSwitcher(Switcher::new());
        room.add_device("switch1", sw);
        assert_eq!(vec!["switch1"], room.get_device_names_list());
    }

    #[test]
    fn test_get_device_info_list() {
        let mut room = Room::new();
        let sw = Device::DevSwitcher(Switcher::new());
        room.add_device("switch1", sw);
        assert_eq!(
            vec!["Switcher: new switcher, state: OFF, current_power_consumption: 0"],
            room.get_device_info_list()
        );
    }
}
