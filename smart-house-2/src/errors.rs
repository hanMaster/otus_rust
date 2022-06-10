use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum DeviceErrors {
    DeviceNotFound,
}

impl Display for DeviceErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            DeviceErrors::DeviceNotFound => write!(f, "!!!Device not found!!!"),
        }
    }
}

impl Error for DeviceErrors {}

#[derive(Debug)]
pub struct StoreError {
    source: DeviceErrors,
}

impl Display for StoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = "Store error ".to_string();
        result.push_str(self.source.to_string().as_str());
        write!(f, "{}", result)
    }
}

impl Error for StoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl StoreError {
    pub fn with_source(err: DeviceErrors) -> Self {
        Self { source: err }
    }
}
