use crate::switcher::Switcher;
use crate::thermometer::Thermometer;

pub enum Device {
    DevSwitcher(Switcher),
    DevThermometer(Thermometer),
}
