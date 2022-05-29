use crate::device::Info;

#[derive(Debug)]
pub struct Thermometer {
    current_temperature: f64,
}

impl Default for Thermometer {
    fn default() -> Self {
        Self {
            current_temperature: 0.0,
        }
    }
}

impl Info for Thermometer {
    fn get_info(&self) -> String {
        format!(
            "Thermometer: current_temperature: {}",
            self.current_temperature
        )
    }
}

impl Thermometer {
    pub fn get_temperature(&self) -> f64 {
        self.current_temperature
    }
}
