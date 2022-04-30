use crate::config::Config;
use crate::imap_commands::capability::get_capabilities;
use async_trait::async_trait;
use const_format::formatcp;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{path::Path, sync::Arc};
use tokio::sync::broadcast;

pub(crate) mod encrypted;
pub(crate) mod state;
pub(crate) mod unencrypted;

/// A const variant of the Capabilities we welcome clients with
pub const CAPABILITY_HELLO: &str =
    formatcp!("* OK [{}] IMAP4rev2 Service Ready", get_capabilities());

/// An implementation of a imap server
#[async_trait]
pub trait Server {
    /// Start the server
    async fn run(
        config: Arc<Config>,
        file_watcher: broadcast::Sender<Event>,
    ) -> color_eyre::eyre::Result<()>;
}

/// Starts the imap server
///
/// # Errors
///
/// Returns an error if the server startup fails
pub fn start(config: Arc<Config>) -> color_eyre::eyre::Result<()> {
    let (tx, _rx) = broadcast::channel(1);
    let tx_clone = tx.clone();
    let mut watcher = RecommendedWatcher::new(move |res: notify::Result<Event>| {
        if let Ok(event) = res {
            futures::executor::block_on(async {
                tx.send(event.clone())
                    .expect("failed to send filechange event");
            });
        }
    })?;

    std::fs::create_dir_all(&config.mail.maildir_folders)?;

    watcher.watch(
        Path::new(&config.mail.maildir_folders),
        RecursiveMode::Recursive,
    )?;

    let config_clone = Arc::clone(&config);
    let tx_clone2 = tx_clone.clone();
    tokio::spawn(async move {
        if let Err(e) = unencrypted::Unencrypted::run(Arc::clone(&config_clone), tx_clone).await {
            panic!("Unable to start server: {:?}", e);
        }
    });
    tokio::spawn(async move {
        if let Err(e) = encrypted::Encrypted::run(Arc::clone(&config), tx_clone2).await {
            panic!("Unable to start TLS server: {:?}", e);
        }
    });
    Ok(())
}