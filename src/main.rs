use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
mod config;
mod errors;
mod profile;
mod utils;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { name } => commands::create::execute(name).await?,
        Commands::List => commands::list::execute().await?,
        Commands::Show { name } => commands::show::execute(name).await?,
        Commands::Run { name, opencode_args } => {
            commands::run::execute(name, opencode_args).await?
        }
        Commands::Clone {
            source,
            destination,
        } => commands::clone::execute(source, destination).await?,
        Commands::Remove { name, yes } => commands::remove::execute(name, yes).await?,
        Commands::Doctor => commands::doctor::execute().await?,
    }

    Ok(())
}
