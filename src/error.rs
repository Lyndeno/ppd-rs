use thiserror::Error;

#[derive(Error, Debug)]
pub enum PpdError {
    #[error("DBus error: {0}")]
    DBusError(#[from] zbus::Error),

    #[error("Invalid profile: {0}")]
    InvalidProfile(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Unimplemented feature: {0}")]
    Unimplemented(String),
}

pub type Result<T> = std::result::Result<T, PpdError>;
