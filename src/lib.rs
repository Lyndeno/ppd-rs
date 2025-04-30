use zbus::zvariant::{DeserializeDict, Optional, OwnedValue, SerializeDict, Type, Value};

use zbus::{Result as ZbusResult, proxy};

#[derive(SerializeDict, DeserializeDict, Debug, Type, OwnedValue, Value)]
#[zvariant(signature = "dict", rename_all = "PascalCase")]
pub struct Profile {
    pub profile: String,
    pub driver: String,
    pub platform_driver: Option<String>,
    pub cpu_driver: String,
}

#[derive(SerializeDict, DeserializeDict, Debug, Type, OwnedValue, Value)]
#[zvariant(signature = "dict", rename_all = "PascalCase")]
pub struct Action {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(SerializeDict, DeserializeDict, Debug, Type, OwnedValue, Value)]
#[zvariant(signature = "dict", rename_all = "PascalCase")]
pub struct ActiveHold {
    pub reason: String,
    pub profile: String,
    pub application_id: String,
}

#[proxy(
    interface = "org.freedesktop.UPower.PowerProfiles",
    default_service = "org.freedesktop.UPower.PowerProfiles",
    default_path = "/org/freedesktop/UPower/PowerProfiles"
)]
pub trait Ppd {
    fn hold_profile(
        &self,
        profile: String,
        reason: String,
        application_id: String,
    ) -> ZbusResult<u32>;

    fn release_profile(&self, cookie: u32) -> ZbusResult<()>;

    fn set_action_enabled(&self, action: String, enabled: bool) -> ZbusResult<()>;

    #[zbus(signal)]
    fn profile_released(&self) -> ZbusResult<u32>;

    #[zbus(property)]
    fn active_profile(&self) -> ZbusResult<String>;

    #[zbus(property)]
    fn set_active_profile(&self, string: String) -> ZbusResult<()>;

    #[zbus(property)]
    fn performance_inhibited(&self) -> ZbusResult<String>;

    #[zbus(property)]
    fn performance_degraded(&self) -> ZbusResult<Optional<String>>;

    #[zbus(property)]
    fn profiles(&self) -> ZbusResult<Vec<Profile>>;

    #[zbus(property)]
    fn actions(&self) -> ZbusResult<Vec<String>>;

    #[zbus(property)]
    fn version(&self) -> ZbusResult<String>;

    #[zbus(property)]
    fn actions_info(&self) -> ZbusResult<Vec<Action>>;

    #[zbus(property)]
    fn active_profile_holds(&self) -> ZbusResult<Vec<ActiveHold>>;

    #[zbus(property)]
    fn battery_aware(&self) -> ZbusResult<bool>;

    #[zbus(property)]
    fn set_battery_aware(&self, value: bool) -> ZbusResult<()>;
}

// Re-export error module
pub mod error;
pub use error::{PpdError, Result};
