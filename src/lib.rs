//! # ppd-rs
//!
//! `ppd-rs` is a Rust interface for the Power Profiles Daemon (PPD), which allows applications
//! to interact with system power profiles on Linux systems.
//!
//! This library provides a Rust API for the D-Bus interface exposed by the Power Profiles Daemon,
//! allowing applications to:
//!
//! - Get the current active power profile
//! - List available power profiles
//! - Switch between power profiles
//! - Hold a profile for a specific application
//! - Release profile holds
//! - Configure power-related actions
//! - Query and configure battery-aware behavior
//!
//! ## Usage
//!
//! ```no_run
//! use ppd::{PpdProxyBlocking, Result};
//! use zbus::blocking::Connection;
//!
//! fn main() -> Result<()> {
//!     let connection = Connection::system()?;
//!     let proxy = PpdProxyBlocking::new(&connection)?;
//!     
//!     // Get current profile
//!     let current_profile = proxy.active_profile()?;
//!     println!("Current profile: {}", current_profile);
//!     
//!     // List available profiles
//!     let profiles = proxy.profiles()?;
//!     for profile in profiles {
//!         println!("Profile: {}", profile.profile);
//!     }
//!     
//!     Ok(())
//! }
//! ```

use std::fmt::Display;

use serde::{Deserialize, Serialize};
use zbus::zvariant::{Optional, OwnedValue, Type, Value};

use zbus::{Result as ZbusResult, proxy};

#[derive(Deserialize, Serialize, Type, Value, OwnedValue, Debug, PartialEq, Clone, Eq, Hash)]
#[zvariant(signature = "s", rename_all = "kebab-case")]
pub enum PowerProfile {
    PowerSaver,
    Balanced,
    Performance,
}

impl Display for PowerProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PowerProfile::Balanced => "balanced",
                PowerProfile::PowerSaver => "power-saver",
                PowerProfile::Performance => "performance",
            }
        )
    }
}

impl TryFrom<String> for PowerProfile {
    type Error = ();
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.as_str() {
            "balanced" => Ok(PowerProfile::Balanced),
            "power-saver" => Ok(PowerProfile::PowerSaver),
            "performance" => Ok(PowerProfile::Performance),
            _ => Err(()),
        }
    }
}

/// Represents a power profile configuration
///
/// This struct contains information about a power profile including its name,
/// driver information, and CPU settings.
#[derive(Serialize, Deserialize, Debug, Type, OwnedValue, Value)]
#[zvariant(signature = "dict", rename_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub struct Profile {
    /// The name of the profile (e.g., "power-saver", "balanced", "performance")
    pub profile: PowerProfile,
    /// The name of the driver used for this profile
    pub driver: String,
    /// Optional platform-specific driver information
    pub platform_driver: Option<String>,
    /// The CPU driver used for this profile
    pub cpu_driver: Option<String>,
}

/// Represents a configurable power-related action
///
/// Actions can be enabled or disabled to control system behavior
/// in different power scenarios.
#[derive(Serialize, Deserialize, Debug, Type, OwnedValue, Value)]
#[zvariant(signature = "dict", rename_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub struct Action {
    /// The name of the action
    pub name: String,
    /// A human-readable description of the action
    pub description: String,
    /// Whether the action is currently enabled
    pub enabled: bool,
}

/// Represents an active profile hold
///
/// When an application needs to temporarily hold a specific power profile,
/// this structure tracks that information.
#[derive(Serialize, Deserialize, Debug, Type, OwnedValue, Value)]
#[zvariant(signature = "dict", rename_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub struct ActiveHold {
    /// The reason provided for holding this profile
    pub reason: String,
    /// The name of the profile being held
    pub profile: PowerProfile,
    /// The application ID of the application holding the profile
    pub application_id: String,
}

/// Power Profiles Daemon D-Bus interface
///
/// This trait defines all the methods, signals, and properties available
/// through the Power Profiles Daemon D-Bus interface. It's automatically
/// implemented for proxy objects.
#[proxy(
    interface = "org.freedesktop.UPower.PowerProfiles",
    default_service = "org.freedesktop.UPower.PowerProfiles",
    default_path = "/org/freedesktop/UPower/PowerProfiles"
)]
pub trait Ppd {
    /// Request to hold a specific power profile for an application
    ///
    /// # Arguments
    ///
    /// * `profile` - The name of the profile to hold
    /// * `reason` - The reason for holding the profile
    /// * `application_id` - The application ID requesting the hold
    ///
    /// # Returns
    ///
    /// A cookie that can be used later to release the hold
    fn hold_profile(
        &self,
        profile: PowerProfile,
        reason: String,
        application_id: String,
    ) -> ZbusResult<u32>;

    /// Release a previously requested profile hold
    ///
    /// # Arguments
    ///
    /// * `cookie` - The cookie returned from a previous hold_profile call
    fn release_profile(&self, cookie: u32) -> ZbusResult<()>;

    /// Enable or disable a specific action
    ///
    /// # Arguments
    ///
    /// * `action` - The name of the action to configure
    /// * `enabled` - Whether to enable or disable the action
    fn set_action_enabled(&self, action: String, enabled: bool) -> ZbusResult<()>;

    /// Signal emitted when a profile is released
    ///
    /// # Returns
    ///
    /// The cookie of the released profile hold
    #[zbus(signal)]
    fn profile_released(&self) -> ZbusResult<u32>;

    /// Get the currently active power profile
    ///
    /// # Returns
    ///
    /// The name of the active profile
    #[zbus(property)]
    fn active_profile(&self) -> ZbusResult<PowerProfile>;

    /// Set the active power profile
    ///
    /// # Arguments
    ///
    /// * `string` - The name of the profile to activate
    #[zbus(property)]
    fn set_active_profile(&self, string: PowerProfile) -> ZbusResult<()>;

    /// Get information about why performance might be inhibited
    ///
    /// # Returns
    ///
    /// A string describing why performance is inhibited, or empty if not inhibited
    #[zbus(property)]
    fn performance_inhibited(&self) -> ZbusResult<String>;

    /// Get information about why performance might be degraded
    ///
    /// # Returns
    ///
    /// An optional string describing why performance is degraded, or None if not degraded
    #[zbus(property)]
    fn performance_degraded(&self) -> ZbusResult<Optional<String>>;

    /// Get the list of available power profiles
    ///
    /// # Returns
    ///
    /// A vector of Profile structures
    #[zbus(property)]
    fn profiles(&self) -> ZbusResult<Vec<Profile>>;

    /// Get the list of available actions
    ///
    /// # Returns
    ///
    /// A vector of action names
    #[zbus(property)]
    fn actions(&self) -> ZbusResult<Vec<String>>;

    /// Get the version of the Power Profiles Daemon
    ///
    /// # Returns
    ///
    /// The version string
    #[zbus(property)]
    fn version(&self) -> ZbusResult<String>;

    /// Get detailed information about available actions
    ///
    /// # Returns
    ///
    /// A vector of Action structures
    #[zbus(property)]
    fn actions_info(&self) -> ZbusResult<Vec<Action>>;

    /// Get information about active profile holds
    ///
    /// # Returns
    ///
    /// A vector of ActiveHold structures
    #[zbus(property)]
    fn active_profile_holds(&self) -> ZbusResult<Vec<ActiveHold>>;

    /// Check if battery-aware behavior is enabled
    ///
    /// When enabled, the system may automatically adjust profiles based on power source
    ///
    /// # Returns
    ///
    /// True if battery-aware behavior is enabled
    #[zbus(property)]
    fn battery_aware(&self) -> ZbusResult<bool>;

    /// Configure battery-aware behavior
    ///
    /// # Arguments
    ///
    /// * `value` - True to enable, false to disable battery-aware behavior
    #[zbus(property)]
    fn set_battery_aware(&self, value: bool) -> ZbusResult<()>;
}

/// Error handling for the ppd library
pub mod error;
pub use error::{PpdError, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let p = PowerProfile::Performance;
        let s = "performance";
        assert_eq!(p, PowerProfile::try_from(s.to_owned()).unwrap())
    }

    #[test]
    fn test_from_value() {
        let p = PowerProfile::Performance;
        let v = Value::new("performance");

        assert_eq!(p, PowerProfile::try_from(v).unwrap())
    }

    #[test]
    fn test_display() {
        let p = PowerProfile::PowerSaver;

        assert_eq!(p.to_string(), "power-saver".to_owned())
    }
}
