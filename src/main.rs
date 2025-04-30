use std::collections::HashSet;

use clap::Parser;
use ppd::PpdProxyBlocking;

mod args;

use args::Args;
use ppd::error::{PpdError, Result};

use zbus::blocking::Connection;

fn main() -> Result<()> {
    let cli = Args::parse();

    let connection = Connection::system()?;
    let proxy = PpdProxyBlocking::new(&connection)?;
    match cli.command {
        Some(c) => match c {
            args::Commands::Get => print_profile(&proxy)?,
            args::Commands::List => list(&proxy)?,
            args::Commands::ListHolds => {
                Err(PpdError::Unimplemented("ListHolds command".to_string()))?
            }
            args::Commands::Set { profile } => set(&proxy, profile)?,
            args::Commands::ListActions => list_actions(&proxy)?,
            args::Commands::Launch {
                arguments: _,
                profile: _,
                reason: _,
                appid: _,
            } => Err(PpdError::Unimplemented("Launch command".to_string()))?,
            args::Commands::QueryBatteryAware => query_battery_aware(&proxy)?,
            args::Commands::ConfigureAction {
                action: _,
                enable: _,
                disable: _,
            } => Err(PpdError::Unimplemented(
                "ConfigureAction command".to_string(),
            ))?,
            args::Commands::ConfigureBatteryAware { enable, disable } => {
                configure_battery_aware(&proxy, enable, disable)?
            }
        },
        _ => list(&proxy)?,
    };
    Ok(())
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
        proxy.set_active_profile(profile)?;
        Ok(())
    } else {
        Err(PpdError::InvalidProfile(profile))
    }
}

fn query_battery_aware(proxy: &PpdProxyBlocking) -> Result<()> {
    let ba = proxy.battery_aware()?;
    println!("Dynamic changes from charger and battery events: {}", ba);
    Ok(())
}

fn list_actions(proxy: &PpdProxyBlocking) -> Result<()> {
    for action in proxy.actions_info()? {
        println!("Name: {}", action.name);
        println!("Description: {}", action.description);
        println!("Enabled: {}", action.enabled);
    }
    Ok(())
}

fn configure_battery_aware(proxy: &PpdProxyBlocking, enable: bool, disable: bool) -> Result<()> {
    if enable && disable {
        Err(PpdError::InvalidConfig(
            "can't set both enable and disable".to_string(),
        ))
    } else if !(enable || disable) {
        Err(PpdError::InvalidConfig(
            "enable or disable is required".to_string(),
        ))
    } else {
        proxy.set_battery_aware(enable)?;
        Ok(())
    }
}
