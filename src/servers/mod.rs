use std::sync::Arc;

use async_trait::async_trait;

pub(crate) mod encrypted;
pub(crate) mod state;
pub(crate) mod unencrypted;

/// An implementation of a imap server
#[async_trait]
pub trait Server {
    /// Start the server
    async fn run_imap(config: Arc<Config>) -> anyhow::Result<()>;
}

pub use encrypted::Encrypted;
pub use unencrypted::Unencrypted;

use crate::config::Config;
