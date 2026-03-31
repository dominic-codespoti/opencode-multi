use crate::config::Config;
use crate::profile::{load_profile, ProfileStatus};
use anyhow::Result;
use colored::Colorize;

pub async fn execute(name: String) -> Result<()> {
    let profile = load_profile(&name)?;
    let config = Config::new()?;

    let status = profile.status();
    let auth_path = config.profile_auth_path(&name);

    println!("{}", format!("Profile: {}", name).bold());
    println!();

    println!("{}", "Paths:".bold());
    println!("  Config:  {:?}", profile.config_dir);
    println!("  Data:    {:?}", profile.data_dir);
    println!("  Auth:    {:?}", auth_path);
    println!();

    println!("{}", "Status:".bold());
    println!(
        "  Config exists: {}",
        if profile.config_dir.exists() {
            "yes".green()
        } else {
            "no".red()
        }
    );
    println!(
        "  Data exists:   {}",
        if profile.data_dir.exists() {
            "yes".green()
        } else {
            "no".red()
        }
    );
    println!(
        "  Auth present:  {}",
        if auth_path.exists() {
            "yes".green()
        } else {
            "no".red()
        }
    );
    println!();

    println!(
        "  Overall: {}",
        match status {
            ProfileStatus::Healthy => "healthy ✓".green(),
            ProfileStatus::NeedsAuth => "needs-auth (run /connect in OpenCode)".yellow(),
            ProfileStatus::Missing => "missing ✗".red(),
        }
    );

    Ok(())
}
