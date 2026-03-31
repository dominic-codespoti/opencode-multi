use crate::config::Config;
use crate::profile::Profile;
use crate::utils::{success, validate_profile_name};
use anyhow::{Context, Result};

pub async fn execute(name: String, init: bool) -> Result<()> {
    validate_profile_name(&name)?;

    let profile = Profile::new(&name)?;

    if profile.exists() {
        anyhow::bail!("Profile '{}' already exists", name);
    }

    if init {
        // Check if default OpenCode config exists
        if !Config::default_opencode_config_exists() {
            anyhow::bail!("No existing OpenCode configuration found at ~/.config/opencode/\nUse `opencode-multi create {}` without --init to create a blank profile.", name);
        }

        let default_config = Config::default_opencode_config_dir()?;
        let default_data = Config::default_opencode_data_dir().ok();
        
        // Create profile directories
        profile.create()?;
        
        // Copy existing config
        copy_dir_all(&default_config, &profile.config_dir)
            .with_context(|| format!("Failed to copy existing OpenCode configuration from {:?}", default_config))?;

        // Copy existing data (auth, etc.) if it exists
        if let Some(ref data_dir) = default_data {
            if data_dir.exists() {
                copy_dir_all(data_dir, &profile.data_dir)
                    .with_context(|| format!("Failed to copy existing OpenCode data from {:?}", data_dir))?;
            }
        }

        success(&format!("Created profile '{}' from existing OpenCode configuration", name));
    } else {
        profile.create()?;
        success(&format!("Created profile '{}'", name));
    }

    println!();
    println!("Config directory: {:?}", profile.config_dir);
    println!("Data directory: {:?}", profile.data_dir);
    println!();
    println!("To use this profile, run:");
    println!("  opencode-multi run {}", name);

    Ok(())
}

fn copy_dir_all(src: impl AsRef<std::path::Path>, dst: impl AsRef<std::path::Path>) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    
    for entry in walkdir::WalkDir::new(src) {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let relative_path = path.strip_prefix(src)?;
            let dest_path = dst.join(relative_path);
            
            if let Some(parent) = dest_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            
            std::fs::copy(path, dest_path)?;
        }
    }

    Ok(())
}
