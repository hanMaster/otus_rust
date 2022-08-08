use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceErrors {
    #[error("!!!Device not found!!!")]
    DeviceNotFound,
}

#[derive(Debug, Error)]
#[error("Store error {}", self.source)]
pub struct StoreError {
    source: DeviceErrors,
}

impl StoreError {
    pub fn with_source(err: DeviceErrors) -> Self {
        Self { source: err }
    }
}
