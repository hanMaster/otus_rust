use crate::device::Info;
use std::fmt::{Display, Formatter};

pub enum SwitchState {
    OFF,
    ON,
}

impl Display for SwitchState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            SwitchState::ON => write!(f, "ON"),
            SwitchState::OFF => write!(f, "OFF"),
        }
    }
}

pub struct Switcher {
    description: String,
    state: SwitchState,
    current_power_consumption: f64,
}

impl Default for Switcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Info for Switcher {
    fn get_info(&self) -> String {
        format!(
            "Switcher: {}, state: {}, current_power_consumption: {}",
            self.description, self.state, self.current_power_consumption
        )
    }
}

impl Switcher {
    pub fn new() -> Self {
        let description = String::from("new switcher");
        Self {
            description,
            state: SwitchState::OFF,
            current_power_consumption: 0.0,
        }
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn set_description(&mut self, new_description: &str) {
        self.description = String::from(new_description);
    }

    pub fn get_state(&self) -> String {
        match self.state {
            SwitchState::ON => String::from("Switched ON"),
            SwitchState::OFF => String::from("Switched OFF"),
        }
    }

    pub fn toggle_switch(&mut self) {
        if let SwitchState::OFF = self.state {
            self.state = SwitchState::ON;
        } else {
            self.state = SwitchState::OFF;
        }
    }

    pub fn get_current_power_consumption(&self) -> f64 {
        self.current_power_consumption
    }
}
