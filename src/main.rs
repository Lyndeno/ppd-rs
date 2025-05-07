//! Command-line utility for interacting with the Power Profiles Daemon
//!
//! This utility provides a convenient interface to view and control
//! power profiles on Linux systems that use the Power Profiles Daemon.
//!
//! Run without arguments to list available profiles, or use one of
//! the available subcommands to perform specific operations.

use std::collections::HashSet;

use clap::Parser;
use ppd::PpdProxyBlocking;

mod args;

use args::Args;
use ppd::error::{PpdError, Result};

use zbus::blocking::Connection;

/// Main entry point for the ppd utility
fn main() -> Result<()> {
    // Parse command-line arguments
    let cli = Args::parse();

    // Connect to the system D-Bus and create a proxy to the Power Profiles Daemon
    let connection = Connection::system()?;
    let proxy = PpdProxyBlocking::new(&connection)?;

    // Execute the appropriate command (or list if no command specified)
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
            args::Commands::Watch => watch(&proxy)?,
        },
        _ => list(&proxy)?,
    };
    Ok(())
}

/// Print the currently active power profile
///
/// # Arguments
///
/// * `proxy` - The PPD proxy object
fn print_profile(proxy: &PpdProxyBlocking) -> Result<()> {
    let reply = proxy.active_profile()?;
    println!("{reply}");
    Ok(())
}

/// List all available power profiles and their properties
///
/// This function displays all available profiles with their respective
/// drivers and settings. The currently active profile is marked with
/// an asterisk (*).
///
/// # Arguments
///
/// * `proxy` - The PPD proxy object
fn list(proxy: &PpdProxyBlocking) -> Result<()> {
    let current = proxy.active_profile()?;
    let profiles = proxy.profiles()?;
    let degraded = proxy
        .performance_degraded()?
        .as_ref()
        .unwrap_or(&String::from("no"))
        .to_string();

    let mut profiles_iter = profiles.into_iter().rev().peekable();

    while let Some(profile) = profiles_iter.next() {
        let degraded_string = if profile.profile == "performance" {
            Some(degraded.clone())
        } else {
            None
        };

        let current_marker = if current == profile.profile { "*" } else { " " };
        println!("{} {}:", current_marker, profile.profile);
        if let Some(s) = profile.cpu_driver.clone() {
            println!("    CpuDriver:\t{}", s);
        }
        if let Some(s) = profile.platform_driver.clone() {
            println!("    PlatformDriver:\t{}", s);
        }
        if let Some(s) = degraded_string {
            println!("    Degraded:  {}", s);
        }

        if profiles_iter.peek().is_some() {
            println!();
        }
    }
    Ok(())
}

fn watch(proxy: &PpdProxyBlocking) -> Result<()> {
    println!("{}", proxy.active_profile()?);
    let signal = proxy.receive_active_profile_changed();
    for p in signal {
        let name = p.get()?;
        println!("{name}");
    }
    Ok(())
}

/// Set the active power profile
///
/// # Arguments
///
/// * `proxy` - The PPD proxy object
/// * `profile` - The name of the profile to set as active
///
/// # Returns
///
/// An error if the requested profile does not exist
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

/// Query whether battery-aware behavior is enabled
///
/// # Arguments
///
/// * `proxy` - The PPD proxy object
fn query_battery_aware(proxy: &PpdProxyBlocking) -> Result<()> {
    let ba = proxy.battery_aware()?;
    println!("Dynamic changes from charger and battery events: {}", ba);
    Ok(())
}

/// List all available power-related actions
///
/// # Arguments
///
/// * `proxy` - The PPD proxy object
fn list_actions(proxy: &PpdProxyBlocking) -> Result<()> {
    for action in proxy.actions_info()? {
        println!("Name: {}", action.name);
        println!("Description: {}", action.description);
        println!("Enabled: {}", action.enabled);
    }
    Ok(())
}

/// Configure battery-aware behavior
///
/// When enabled, the system may automatically adjust the power profile
/// based on whether the system is on battery or connected to power.
///
/// # Arguments
///
/// * `proxy` - The PPD proxy object
/// * `enable` - Whether to enable battery-aware behavior
/// * `disable` - Whether to disable battery-aware behavior
///
/// # Returns
///
/// An error if both enable and disable are true, or if neither is true
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
