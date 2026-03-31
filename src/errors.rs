use std::fmt;

#[derive(Debug)]
pub enum OpencodeMultiError {
    ProfileNotFound(String),
    ProfileAlreadyExists(String),
    InvalidProfileName(String),
    ConfigError(String),
    IoError(String),
}

impl fmt::Display for OpencodeMultiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProfileNotFound(name) => write!(f, "Profile '{}' not found", name),
            Self::ProfileAlreadyExists(name) => write!(f, "Profile '{}' already exists", name),
            Self::InvalidProfileName(msg) => write!(f, "Invalid profile name: {}", msg),
            Self::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            Self::IoError(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

impl std::error::Error for OpencodeMultiError {}
