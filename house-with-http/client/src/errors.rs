use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to add room")]
    AddRoomError,

    #[error("Failed to add device")]
    AddDeviceError,

    #[error("Failed to get rooms")]
    GetRoomsError,

    #[error("Failed to get devices")]
    GetDevicesError,

    #[error("Failed to make request")]
    RequestFailed(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
