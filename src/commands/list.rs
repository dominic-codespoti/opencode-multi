use crate::config::Config;
use crate::profile::{Profile, ProfileStatus};
use crate::utils::info;
use anyhow::Result;
use colored::Colorize;

pub async fn execute() -> Result<()> {
    let config = Config::new()?;
    config.ensure_roots_exist()?;

    let profiles = config.list_profiles()?;

    if profiles.is_empty() {
        info("No profiles found. Create one with: opencode-multi create <name>");
        return Ok(());
    }

    println!("{}", "NAME".bold().underline());
    println!("{}", "      CONFIG   AUTH   STATUS".dimmed());

    for name in profiles {
        let profile = Profile::new(&name)?;
        let status = profile.status();

        let config_exists = if profile.config_dir.exists() {
            "yes".green()
        } else {
            "no".red()
        };

        let auth_path = config.profile_auth_path(&name);
        let auth_exists = if auth_path.exists() {
            "yes".green()
        } else {
            "no".red()
        };

        let status_str = match status {
            ProfileStatus::Healthy => "healthy".green(),
            ProfileStatus::NeedsAuth => "needs-auth".yellow(),
            ProfileStatus::Missing => "missing".red(),
        };

        println!(
            "{:<10} {:>6}   {:>4}   {}",
            name, config_exists, auth_exists, status_str
        );
    }

    Ok(())
}
