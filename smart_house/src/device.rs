use crate::switcher::Switcher;
use crate::thermometer::Thermometer;

pub enum Device {
    DevSwitcher(Switcher),
    DevThermometer(Thermometer),
}

pub trait Info {
    fn get_info(&self) -> String;
}

impl Info for Device {
    fn get_info(&self) -> String {
        match self {
            Device::DevSwitcher(switcher) => switcher.get_info(),
            Device::DevThermometer(thermometer) => thermometer.get_info(),
        }
    }
}
