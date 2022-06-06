use crate::device::Info;

#[derive(Debug)]
pub struct Thermometer {
    current_temperature: f64,
}

impl Default for Thermometer {
    fn default() -> Self {
        Self::new()
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
    pub fn new() -> Self {
        Self {
            current_temperature: 18.0,
        }
    }
    pub fn get_temperature(&self) -> f64 {
        self.current_temperature
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let term = Thermometer::default();
        assert_eq!(18.0, term.get_temperature());
    }

    #[test]
    fn test_get_info() {
        let term = Thermometer::default();
        assert_eq!("Thermometer: current_temperature: 18", term.get_info());
    }
}
