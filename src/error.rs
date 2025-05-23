//! Error handling for the ppd-rs library
//!
//! This module provides custom error types and a convenient Result type
//! for all operations in the ppd-rs library.

use thiserror::Error;

use crate::PowerProfile;

/// Errors that can occur when using the ppd-rs library
#[derive(Error, Debug)]
pub enum PpdError {
    /// Error communicating with the D-Bus service
    #[error("DBus error: {0}")]
    DBusError(#[from] zbus::Error),

    /// Requested profile was not found or is invalid
    #[error("Invalid profile: {0}")]
    InvalidProfile(PowerProfile),

    /// Configuration options provided were invalid
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Feature is not yet implemented
    #[error("Unimplemented feature: {0}")]
    Unimplemented(String),
}

/// A specialized Result type for ppd-rs operations
///
/// This type is used throughout the ppd-rs library for any operation
/// that can produce an error.
pub type Result<T> = std::result::Result<T, PpdError>;
