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

impl Thermometer {
    pub fn get_temperature(&self) -> f64 {
        self.current_temperature
    }
}
