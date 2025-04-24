use std::collections::HashSet;

use clap::Parser;
use zbus::zvariant::{DeserializeDict, Optional, OwnedValue, SerializeDict, Type, Value};

mod args;
use args::Args;

use zbus::blocking::Connection;
use zbus::{Result, proxy};
fn main() -> Result<()> {
    let cli = Args::parse();

    let connection = Connection::system()?;
    let proxy = PpdProxyBlocking::new(&connection)?;
    match cli.command {
        Some(c) => match c {
            args::Commands::Get => print_profile(&proxy)?,
            args::Commands::List => list(&proxy)?,
            args::Commands::ListHolds => todo!(),
            args::Commands::Set { profile } => set(&proxy, profile)?,
            args::Commands::ListActions => todo!(),
            args::Commands::Launch {
                arguments: _,
                profile: _,
                reason: _,
                appid: _,
            } => todo!(),
            args::Commands::QueryBatteryAware => todo!(),
            args::Commands::ConfigureAction {
                action: _,
                enable: _,
                disable: _,
            } => todo!(),
            args::Commands::ConfigureBatteryAware {
                enable: _,
                disable: _,
            } => todo!(),
        },
        _ => list(&proxy)?,
    };
    Ok(())
}

#[derive(SerializeDict, DeserializeDict, Debug, Type, OwnedValue, Value)]
#[zvariant(signature = "dict", rename_all = "PascalCase")]
struct Profile {
    profile: String,
    driver: String,
    platform_driver: Option<String>,
    cpu_driver: String,
}

#[derive(SerializeDict, DeserializeDict, Debug, Type, OwnedValue, Value)]
#[zvariant(signature = "dict", rename_all = "PascalCase")]
struct Action {
    name: String,
    description: String,
    enabled: bool,
}

#[derive(SerializeDict, DeserializeDict, Debug, Type, OwnedValue, Value)]
#[zvariant(signature = "dict", rename_all = "PascalCase")]
struct ActiveHold {
    reason: String,
    profile: String,
    application_id: String,
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
    fn performance_degraded(&self) -> Result<Optional<String>>;

    #[zbus(property)]
    fn profiles(&self) -> Result<Vec<Profile>>;

    #[zbus(property)]
    fn actions(&self) -> Result<Vec<String>>;

    #[zbus(property)]
    fn version(&self) -> Result<String>;

    #[zbus(property)]
    fn actions_info(&self) -> Result<Vec<Action>>;

    #[zbus(property)]
    fn active_profile_holds(&self) -> Result<Vec<ActiveHold>>;

    #[zbus(property)]
    fn battery_aware(&self) -> Result<bool>;

    #[zbus(property)]
    fn set_battery_aware(&self, value: bool) -> Result<()>;
}

fn print_profile(proxy: &PpdProxyBlocking) -> Result<()> {
    let reply = proxy.active_profile()?;
    println!("{reply}");
    Ok(())
}

fn list(proxy: &PpdProxyBlocking) -> Result<()> {
    let current = proxy.active_profile()?;
    let profiles = proxy.profiles()?;

    for profile in profiles {
        if current == profile.profile {
            println!("* {}:", profile.profile);
        } else {
            println!("  {}:", profile.profile);
        }
        println!("    CpuDriver:  {}", profile.cpu_driver);
        if let Some(s) = profile.platform_driver {
            println!("    PlatformDriver:  {}", s);
        }
        if profile.profile == "performance" {
            let degraded_string = proxy
                .performance_degraded()?
                .as_ref()
                .unwrap_or(&String::from("no"))
                .to_string();
            println!("    Degraded:  {}", degraded_string);
        }
    }
    Ok(())
}

fn set(proxy: &PpdProxyBlocking, profile: String) -> Result<()> {
    let profiles_names: HashSet<_> = proxy
        .profiles()?
        .iter()
        .map(|x| x.profile.clone())
        .collect();
    if profiles_names.contains(&profile) {
        proxy.set_active_profile(profile)?
    } else {
        println!("Invalid profile");
    }
    Ok(())
}
