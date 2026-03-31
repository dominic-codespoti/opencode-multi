use crate::config::Config;
use crate::profile::{Profile, ProfileStatus};
use anyhow::Result;
use colored::Colorize;

pub async fn execute() -> Result<()> {
    println!("{}", "OpenCode-Multi Doctor".bold().underline());
    println!();

    // Check 1: opencode in PATH
    let opencode_check = which::which("opencode");
    match opencode_check {
        Ok(path) => println!("[{}] opencode found at {:?}", "ok".green(), path),
        Err(_) => println!(
            "[{}] opencode not found in PATH. Please install OpenCode first.",
            "error".red()
        ),
    }

    // Check 2: Config root
    let config = Config::new()?;
    if config.config_root.exists() {
        println!(
            "[{}] Config root exists: {:?}",
            "ok".green(),
            config.config_root
        );
    } else {
        println!(
            "[{}] Config root missing: {:?}",
            "warn".yellow(),
            config.config_root
        );
    }

    // Check 3: Data root
    if config.data_root.exists() {
        println!(
            "[{}] Data root exists: {:?}",
            "ok".green(),
            config.data_root
        );
    } else {
        println!(
            "[{}] Data root missing: {:?}",
            "warn".yellow(),
            config.data_root
        );
    }

    // Check 4: Profiles
    println!();
    println!("{}", "Profile Status:".bold());

    match config.list_profiles() {
        Ok(profiles) => {
            if profiles.is_empty() {
                println!("  No profiles found. Create one with: opencode-multi create <name>");
            } else {
                for name in profiles {
                    let profile = Profile::new(&name)?;
                    let status = profile.status();

                    match status {
                        ProfileStatus::Healthy => {
                            println!("  [{}] Profile '{}' healthy", "ok".green(), name)
                        }
                        ProfileStatus::NeedsAuth => println!(
                            "  [{}] Profile '{}' needs authentication",
                            "warn".yellow(),
                            name
                        ),
                        ProfileStatus::Missing => println!(
                            "  [{}] Profile '{}' has missing directories",
                            "error".red(),
                            name
                        ),
                    }
                }
            }
        }
        Err(e) => println!("  [{}] Failed to list profiles: {}", "error".red(), e),
    }

    Ok(())
}
