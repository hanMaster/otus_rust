use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    pub name: String,
    pub device_type: DeviceType,
    pub ip_address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum DeviceType {
    Socket,
    Thermometer,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddDevice {
    pub room_name: String,
    pub device_name: String,
    pub device_type: DeviceType,
}

impl Device {
    pub fn with_name_and_type(name: &str, device_type: DeviceType) -> Self {
        Self {
            name: name.to_string(),
            device_type,
            ip_address: "localhost".into(),
        }
    }
}

pub trait Info {
    fn get_info(&self) -> String {
        String::from("Device info")
    }

    fn get_info_by_room_and_device(
        &self,
        _room_name: &str,
        _device_name: &str,
    ) -> Option<String> {
        Some(String::from("Device info"))
    }
}

impl Info for Device {
    fn get_info(&self) -> String {
        let device_type = self.device_type;
        match device_type {
            DeviceType::Socket => "Socket info".to_string(),
            DeviceType::Thermometer => "Thermometer info".to_string(),
        }
    }
}