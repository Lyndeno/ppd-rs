//! Command-line interface definitions for the `ppd` utility
//!
//! This module defines the command-line arguments for the Power Profiles
//! Daemon CLI utility using the `clap` crate for argument parsing.

use clap::{Parser, Subcommand};

/// Command-line interface for interacting with the Power Profiles Daemon
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Command to execute (defaults to list if not specified)
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available commands for the ppd utility
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List all available power profiles
    List,

    /// List active profile holds
    ListHolds,

    /// List all available actions
    ListActions,

    /// Get the currently active profile
    Get,

    /// Set the active power profile
    Set {
        /// Name of the profile to set as active
        profile: String,
    },

    /// Configure a power-related action
    ConfigureAction {
        /// Name of the action to configure
        action: String,

        /// Enable the action
        #[arg(long)]
        enable: bool,

        /// Disable the action
        #[arg(long)]
        disable: bool,
    },

    /// Configure battery-aware behavior
    ConfigureBatteryAware {
        /// Enable battery-aware behavior
        #[arg(long)]
        enable: bool,

        /// Disable battery-aware behavior
        #[arg(long)]
        disable: bool,
    },

    /// Query whether battery-aware behavior is enabled
    QueryBatteryAware,

    /// Launch an application with a specific power profile
    Launch {
        /// Command and arguments to launch
        arguments: String,

        /// Profile to use for the application
        #[arg(short, long)]
        profile: Option<String>,

        /// Reason for the profile hold
        #[arg(short, long)]
        reason: Option<String>,

        /// Application ID for the profile hold
        #[arg(short, long)]
        appid: Option<String>,
    },
}
