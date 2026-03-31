use crate::profile::load_profile;
use anyhow::{Context, Result};
use std::process::Stdio;

pub async fn execute(name: String, opencode_args: Vec<String>) -> Result<()> {
    let profile = load_profile(&name)?;

    // Get the data parent directory (XDG_DATA_HOME)
    // OpenCode will append /opencode/ to this path
    let data_parent = profile
        .data_dir
        .parent()
        .expect("Profile data dir should have a parent");

    let mut cmd = tokio::process::Command::new("opencode");
    cmd.env("OPENCODE_CONFIG_DIR", &profile.config_dir)
        .env("XDG_DATA_HOME", data_parent)
        .env("OPENCODE_PROFILE", &name)
        .args(&opencode_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let mut child = cmd
        .spawn()
        .with_context(|| "Failed to spawn opencode. Is it installed and in PATH?")?;

    let status = child
        .wait()
        .await
        .with_context(|| "Failed to wait for opencode process")?;

    if !status.success() {
        let code = status.code().unwrap_or(1);
        std::process::exit(code);
    }

    Ok(())
}
