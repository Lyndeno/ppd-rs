# ppd-rs

A Rust interface to the Power Profiles Daemon for Linux systems.

## Overview

`ppd-rs` provides a complete Rust API for interacting with the Power Profiles Daemon (PPD) on Linux systems. 
The library allows applications to:

- Query and set power profiles (performance, balanced, power-saver)
- Hold specific profiles for application needs
- Configure power-related actions
- Monitor and control battery-aware behavior

The project includes both a library for integration into other Rust applications and a command-line utility (`ppd`) for direct interaction with power profiles.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ppd = "0.1.0"
```

## Command-Line Usage

The included `ppd` utility allows you to interact with power profiles from the command line:

```shell
# List available profiles
ppd list

# Get the current profile
ppd get

# Set a specific profile
ppd set performance

# Configure battery-aware behavior
ppd configure-battery-aware --enable

# List available actions
ppd list-actions
```

## Library Usage

Here's a simple example of using the library in your Rust application:

```rust
use ppd::{PpdProxyBlocking, Result};
use zbus::blocking::Connection;

fn main() -> Result<()> {
    // Connect to the system D-Bus
    let connection = Connection::system()?;
    
    // Create a proxy to the Power Profiles Daemon
    let proxy = PpdProxyBlocking::new(&connection)?;
    
    // Get the current profile
    let active_profile = proxy.active_profile()?;
    println!("Current profile: {}", active_profile);
    
    // List all available profiles
    let profiles = proxy.profiles()?;
    println!("Available profiles:");
    for profile in profiles {
        println!("- {} (CPU driver: {})", profile.profile, profile.cpu_driver);
    }
    
    // Set a new profile
    proxy.set_active_profile("performance".to_string())?;
    println!("Profile set to performance");
    
    Ok(())
}
```

## Development

To build the project:

```shell
cargo build
```

To run the command-line utility:

```shell
cargo run
```

## License

This project is licensed under the MIT License.
