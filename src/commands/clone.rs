use crate::profile::{load_profile, Profile};
use crate::utils::success;
use anyhow::Result;

pub async fn execute(source: String, destination: String) -> Result<()> {
    let source_profile = load_profile(&source)?;
    let dest_profile = Profile::new(&destination)?;

    if dest_profile.exists() {
        anyhow::bail!("Destination profile '{}' already exists", destination);
    }

    source_profile.clone_to(&dest_profile)?;

    success(&format!(
        "Cloned profile '{}' to '{}'",
        source, destination
    ));

    Ok(())
}
