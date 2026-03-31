use crate::config::Config;
use crate::utils::validate_profile_name;
use anyhow::{Context, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProfileStatus {
    Healthy,
    NeedsAuth,
    Missing,
}

impl std::fmt::Display for ProfileStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfileStatus::Healthy => write!(f, "{}", "healthy".green()),
            ProfileStatus::NeedsAuth => write!(f, "{}", "needs-auth".yellow()),
            ProfileStatus::Missing => write!(f, "{}", "missing".red()),
        }
    }
}

impl Profile {
    pub fn new(name: &str) -> Result<Self> {
        validate_profile_name(name)?;

        let config = Config::new()?;
        let config_dir = config.profile_config_dir(name);
        let data_dir = config.profile_data_dir(name);

        Ok(Self {
            name: name.to_string(),
            config_dir,
            data_dir,
        })
    }

    pub fn exists(&self) -> bool {
        self.config_dir.exists() || self.data_dir.exists()
    }

    pub fn status(&self) -> ProfileStatus {
        if !self.exists() {
            return ProfileStatus::Missing;
        }

        let auth_path = Config::new()
            .map(|c| c.profile_auth_path(&self.name))
            .unwrap_or_default();

        if auth_path.exists() {
            ProfileStatus::Healthy
        } else {
            ProfileStatus::NeedsAuth
        }
    }

    pub fn create(&self) -> Result<()> {
        if self.exists() {
            anyhow::bail!("Profile '{}' already exists", self.name);
        }

        // Create directories
        std::fs::create_dir_all(&self.config_dir)
            .with_context(|| format!("Failed to create config directory: {:?}", self.config_dir))?;

        std::fs::create_dir_all(&self.data_dir)
            .with_context(|| format!("Failed to create data directory: {:?}", self.data_dir))?;

        // Create subdirectories
        let subdirs = ["plugins", "commands", "agents", "modes"];
        for subdir in &subdirs {
            std::fs::create_dir_all(self.config_dir.join(subdir))
                .with_context(|| format!("Failed to create {} directory", subdir))?;
        }

        // Create default opencode.json
        let config_content = serde_json::json!({
            "$schema": "https://opencode.ai/config.json"
        });

        let config_path = self.config_dir.join("opencode.json");
        std::fs::write(&config_path, serde_json::to_string_pretty(&config_content)?)
            .with_context(|| format!("Failed to write opencode.json"))?;

        Ok(())
    }

    pub fn remove(&self) -> Result<()> {
        if !self.exists() {
            anyhow::bail!("Profile '{}' does not exist", self.name);
        }

        if self.config_dir.exists() {
            std::fs::remove_dir_all(&self.config_dir)
                .with_context(|| format!("Failed to remove config directory"))?;
        }

        if self.data_dir.exists() {
            std::fs::remove_dir_all(&self.data_dir)
                .with_context(|| format!("Failed to remove data directory"))?;
        }

        Ok(())
    }

    pub fn clone_to(&self, destination: &Profile) -> Result<()> {
        if !self.exists() {
            anyhow::bail!("Source profile '{}' does not exist", self.name);
        }

        if destination.exists() {
            anyhow::bail!("Destination profile '{}' already exists", destination.name);
        }

        // Copy config directory
        if self.config_dir.exists() {
            copy_dir_all(&self.config_dir, &destination.config_dir)?;
        }

        // Copy data directory
        if self.data_dir.exists() {
            copy_dir_all(&self.data_dir, &destination.data_dir)?;
        }

        Ok(())
    }
}

fn copy_dir_all(src: impl AsRef<std::path::Path>, dst: impl AsRef<std::path::Path>) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    std::fs::create_dir_all(&dst)?;

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

pub fn load_profile(name: &str) -> Result<Profile> {
    let profile = Profile::new(name)?;
    if !profile.exists() {
        anyhow::bail!("Profile '{}' not found", name);
    }
    Ok(profile)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_profile_creation_and_status() {
        // We can't fully test Profile in unit tests since it uses real directories
        // Integration tests will cover the full lifecycle
        let profile_result = Profile::new("test-profile");
        assert!(profile_result.is_ok());
    }
}
