use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write};

pub fn confirm(prompt: &str) -> Result<bool> {
    print!("{} [y/N] ", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_lowercase() == "y")
}

pub fn success(msg: &str) {
    println!("{} {}", "✓".green(), msg);
}

pub fn warning(msg: &str) {
    println!("{} {}", "⚠".yellow(), msg);
}

pub fn error(msg: &str) {
    eprintln!("{} {}", "✗".red(), msg);
}

pub fn info(msg: &str) {
    println!("{} {}", "ℹ".blue(), msg);
}

pub fn validate_profile_name(name: &str) -> Result<()> {
    if name.is_empty() {
        anyhow::bail!("Profile name cannot be empty");
    }

    if name.len() > 64 {
        anyhow::bail!("Profile name must be 64 characters or less");
    }

    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        anyhow::bail!(
            "Profile name can only contain alphanumeric characters, hyphens, and underscores"
        );
    }

    if name.starts_with('-') || name.starts_with('_') {
        anyhow::bail!("Profile name cannot start with a hyphen or underscore");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_profile_name_valid() {
        assert!(validate_profile_name("work").is_ok());
        assert!(validate_profile_name("my-profile").is_ok());
        assert!(validate_profile_name("profile_123").is_ok());
        assert!(validate_profile_name("a-b-c-d").is_ok());
    }

    #[test]
    fn test_validate_profile_name_invalid() {
        assert!(validate_profile_name("").is_err());
        assert!(validate_profile_name("-invalid").is_err());
        assert!(validate_profile_name("_invalid").is_err());
        assert!(validate_profile_name("invalid name").is_err());
        assert!(validate_profile_name("invalid@name").is_err());
    }
}
