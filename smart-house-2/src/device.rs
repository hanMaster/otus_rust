use crate::errors::StoreError;
use crate::socket::Socket;
use crate::thermometer::Thermometer;

pub enum Device {
    DevSwitcher(Socket),
    DevThermometer(Thermometer),
}

pub trait Info {
    fn get_info(&self) -> String {
        String::from("Device info")
    }

    fn get_info_by_room_and_device(
        &self,
        _room_name: &str,
        _device_name: &str,
    ) -> Result<String, StoreError> {
        Ok(String::from("Device info"))
    }
}

impl Info for Device {
    fn get_info(&self) -> String {
        match self {
            Device::DevSwitcher(switcher) => switcher.get_info(),
            Device::DevThermometer(thermometer) => thermometer.get_info(),
        }
    }
}
