use std::collections::HashMap;
use zbus::zvariant::{DeserializeDict, OwnedValue, SerializeDict, Type, Value};

use zbus::{Connection, Result, proxy};
fn main() {
    futures::executor::block_on(print_info()).unwrap();
}

#[derive(SerializeDict, DeserializeDict, Debug, Type, OwnedValue, Value)]
#[zvariant(signature = "dict", rename_all = "PascalCase")]
struct Profile {
    profile: String,
    driver: String,
    platform_driver: String,
    cpu_driver: String,
}

#[derive(SerializeDict, DeserializeDict, Debug, Type, OwnedValue, Value)]
#[zvariant(signature = "dict", rename_all = "PascalCase")]
struct Action {
    name: String,
    description: String,
    enabled: bool,
}

#[proxy(
    interface = "org.freedesktop.UPower.PowerProfiles",
    default_service = "org.freedesktop.UPower.PowerProfiles",
    default_path = "/org/freedesktop/UPower/PowerProfiles"
)]
trait Ppd {
    fn hold_profile(&self, profile: String, reason: String, application_id: String) -> Result<u32>;

    fn release_profile(&self, cookie: u32) -> Result<()>;

    fn set_action_enabled(&self, action: String, enabled: bool) -> Result<()>;

    #[zbus(signal)]
    fn profile_released(&self) -> Result<u32>;

    #[zbus(property)]
    fn active_profile(&self) -> Result<String>;

    #[zbus(property)]
    fn set_active_profile(&self, string: String) -> Result<()>;

    #[zbus(property)]
    fn performance_inhibited(&self) -> Result<String>;

    #[zbus(property)]
    fn performance_degraded(&self) -> Result<String>;

    #[zbus(property)]
    fn profiles(&self) -> Result<Vec<Profile>>;

    #[zbus(property)]
    fn actions(&self) -> Result<Vec<String>>;

    #[zbus(property)]
    fn version(&self) -> Result<String>;

    #[zbus(property)]
    fn actions_info(&self) -> Result<Vec<Action>>;

    #[zbus(property)]
    fn active_profile_holds(&self) -> Result<Vec<HashMap<String, zbus::zvariant::Value>>>;

    #[zbus(property)]
    fn battery_aware(&self) -> Result<bool>;

    #[zbus(property)]
    fn set_battery_aware(&self, value: bool) -> Result<()>;
}

async fn print_info() -> Result<()> {
    let connection = Connection::system().await?;

    let proxy = PpdProxy::new(&connection).await?;

    let reply = proxy.active_profile().await?;
    println!("Current profile is {reply}");

    let reply = proxy.performance_inhibited().await?;
    println!("Perf Inhibited is {reply}");

    let reply = proxy.performance_degraded().await?;
    println!("Perf Degraded is {reply}");

    let reply = proxy.actions().await?;
    println!("Actions is {reply:?}");

    let reply = proxy.version().await?;
    println!("Version is {reply}");

    let reply = proxy.actions_info().await?;
    println!("Actions Info is {reply:?}");

    let reply = proxy.active_profile_holds().await?;
    println!("Active Profile Holds is {reply:?}");

    let reply = proxy.battery_aware().await?;
    println!("Battery aware is {reply}");

    let reply = proxy.profiles().await?;
    println!("Available profiles are {reply:?}");

    Ok(())
}
