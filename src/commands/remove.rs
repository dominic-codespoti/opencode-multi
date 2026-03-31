use crate::profile::load_profile;
use crate::utils::{confirm, success};
use anyhow::Result;

pub async fn execute(name: String, yes: bool) -> Result<()> {
    let profile = load_profile(&name)?;

    if !yes {
        let confirmed = confirm(&format!(
            "Are you sure you want to remove profile '{}' and all its data?",
            name
        ))?;

        if !confirmed {
            println!("Removal cancelled.");
            return Ok(());
        }
    }

    profile.remove()?;

    success(&format!("Removed profile '{}'", name));

    Ok(())
}
