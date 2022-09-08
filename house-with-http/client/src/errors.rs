use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to add room")]
    AddRoomError,

    #[error("Failed to add device")]
    AddDeviceError,

    #[error("Failed to remove room")]
    RemoveRoomError,

    #[error("Failed to remove device")]
    RemoveDeviceError,

    #[error("Failed to get house structure")]
    GetHouseError,

    #[error("Failed to make request")]
    RequestFailed(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
