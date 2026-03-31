use crate::profile::Profile;
use crate::utils::{success, validate_profile_name};
use anyhow::Result;

pub async fn execute(name: String) -> Result<()> {
    validate_profile_name(&name)?;

    let profile = Profile::new(&name)?;

    if profile.exists() {
        anyhow::bail!("Profile '{}' already exists", name);
    }

    profile.create()?;

    success(&format!("Created profile '{}'", name));
    println!();
    println!("Config directory: {:?}", profile.config_dir);
    println!("Data directory: {:?}", profile.data_dir);
    println!();
    println!("To use this profile, run:");
    println!("  opencode-multi run {}", name);

    Ok(())
}
