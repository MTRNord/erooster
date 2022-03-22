use serde::{Deserialize, Serialize};

/// The config for the mailserver
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Configurations specific to the TLS part
    pub tls: Tls,
    /// Configurations specific to the mail concept itself
    pub mail: Mail,
}

/// Configurations specific to the TLS part
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tls {
    /// Path to the certificate files
    pub cert_path: String,
}

/// Configurations specific to the mail concept itself
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mail {
    /// Path where maildir style mailboxes are going to get created
    pub maildir_folders: String,
}

impl Config {
    /// Loads the config file to the struct
    ///
    /// # Errors
    ///
    /// Does return io errors if something goes wrong
    pub fn load<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
    ) -> color_eyre::eyre::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: Self = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}
