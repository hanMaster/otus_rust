use crate::switcher::Switcher;
use crate::thermometer::Thermometer;

pub enum Device {
    DevSwitcher(Switcher),
    DevThermometer(Thermometer),
}

pub trait Info {
    fn get_info(&self) -> String {
        String::from("Device info")
    }

    fn get_info_by_room_and_device(&self, _room_name: &str, _device_name: &str) -> String {
        String::from("Device info")
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
