use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "opencode-multi")]
#[command(about = "Multi-profile manager for OpenCode")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new profile with scaffolded structure
    Create { name: String },

    /// List all profiles with status
    List,

    /// Show detailed information about a profile
    Show { name: String },

    /// Run OpenCode with the specified profile
    Run {
        name: String,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        opencode_args: Vec<String>,
    },

    /// Clone an existing profile
    Clone { source: String, destination: String },

    /// Remove a profile and all its data
    Remove {
        name: String,
        #[arg(long, help = "Skip confirmation prompt")]
        yes: bool,
    },

    /// Check system health and profile status
    Doctor,
}
