use anyhow::{Context, Result};
use dirs;
use std::path::PathBuf;

pub struct Config {
    pub config_root: PathBuf,
    pub data_root: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self> {
        let config_root = Self::get_config_root()?;
        let data_root = Self::get_data_root()?;

        Ok(Self {
            config_root,
            data_root,
        })
    }

    fn get_config_root() -> Result<PathBuf> {
        let base = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        Ok(base.join("opencode-multi").join("profiles"))
    }

    fn get_data_root() -> Result<PathBuf> {
        let base = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?;
        Ok(base.join("opencode-multi").join("profiles"))
    }

    pub fn profile_config_dir(&self, name: &str) -> PathBuf {
        self.config_root.join(name)
    }

    pub fn profile_data_dir(&self, name: &str) -> PathBuf {
        self.data_root.join(name)
    }

    pub fn profile_auth_path(&self, name: &str) -> PathBuf {
        // OpenCode stores auth in data_dir/opencode/auth.json
        self.profile_data_dir(name)
            .join("opencode")
            .join("auth.json")
    }

    /// Get the default OpenCode config directory (~/.config/opencode/)
    pub fn default_opencode_config_dir() -> Result<PathBuf> {
        let base = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        let default_dir = base.join("opencode");
        Ok(default_dir)
    }

    /// Check if default OpenCode config exists
    pub fn default_opencode_config_exists() -> bool {
        Self::default_opencode_config_dir()
            .map(|dir| dir.exists())
            .unwrap_or(false)
    }

    /// Get the default OpenCode data directory (~/.local/share/opencode/)
    pub fn default_opencode_data_dir() -> Result<PathBuf> {
        let base = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?;
        let default_dir = base.join("opencode");
        Ok(default_dir)
    }

    /// Check if default OpenCode data exists
    pub fn default_opencode_data_exists() -> bool {
        Self::default_opencode_data_dir()
            .map(|dir| dir.exists())
            .unwrap_or(false)
    }

    pub fn ensure_roots_exist(&self) -> Result<()> {
        std::fs::create_dir_all(&self.config_root)
            .with_context(|| format!("Failed to create config root: {:?}", self.config_root))?;
        std::fs::create_dir_all(&self.data_root)
            .with_context(|| format!("Failed to create data root: {:?}", self.data_root))?;
        Ok(())
    }

    pub fn list_profiles(&self) -> Result<Vec<String>> {
        self.ensure_roots_exist()?;

        let mut profiles = Vec::new();

        if let Ok(entries) = std::fs::read_dir(&self.config_root) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        if let Some(name) = entry.file_name().to_str() {
                            profiles.push(name.to_string());
                        }
                    }
                }
            }
        }

        profiles.sort();
        Ok(profiles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_profile_paths() {
        let config = Config::new().unwrap();
        let config_dir = config.profile_config_dir("test");
        let data_dir = config.profile_data_dir("test");
        let auth_path = config.profile_auth_path("test");

        assert!(config_dir.to_string_lossy().contains("test"));
        assert!(data_dir.to_string_lossy().contains("test"));
        assert!(auth_path.to_string_lossy().contains("auth.json"));
    }
}
