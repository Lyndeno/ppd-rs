use clap::{Parser, Subcommand};

/// ppdctl
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    List,
    ListHolds,
    ListActions,
    Get,
    Set {
        profile: String,
    },
    ConfigureAction {
        action: String,

        #[arg(long)]
        enable: bool,

        #[arg(long)]
        disable: bool,
    },
    ConfigureBatteryAware {
        #[arg(long)]
        enable: bool,

        #[arg(long)]
        disable: bool,
    },
    QueryBatteryAware,
    Launch {
        arguments: String,

        #[arg(short, long)]
        profile: Option<String>,

        #[arg(short, long)]
        reason: Option<String>,

        #[arg(short, long)]
        appid: Option<String>,
    },
}
