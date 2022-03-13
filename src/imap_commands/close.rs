use crate::{
    config::Config,
    imap_commands::{Command, Data, CommandData},
    servers::state::{Access, State},
};
use async_trait::async_trait;
use futures::{channel::mpsc::SendError, Sink, SinkExt};
use maildir::Maildir;
use std::{fs, path::Path, sync::Arc};
use tracing::debug;

pub struct Close<'a> {
    pub data: &'a Data,
}

#[async_trait]
impl<S> Command<S> for Close<'_>
where
    S: Sink<String, Error = SendError> + std::marker::Unpin + std::marker::Send,
{
    async fn exec(
        &mut self,
        lines: &mut S,
        config: Arc<Config>,
        command_data: &CommandData,
    ) -> color_eyre::eyre::Result<()> {
        let mut write_lock = self.data.con_state.write().await;

        if let State::Selected(folder, access) = &write_lock.state {
            if access == &Access::ReadOnly {
                lines
                    .send(format!("{} NO in read-only mode", command_data.tag))
                    .await?;
                return Ok(());
            }
            let mut folder = folder.replace('/', ".");
            folder.insert(0, '.');
            let mailbox_path = Path::new(&config.mail.maildir_folders)
                .join(self.data.con_state.read().await.username.clone().unwrap())
                .join(folder.clone());
            let maildir = Maildir::from(mailbox_path.clone());

            // We need to check all messages it seems?
            let mails = maildir.list_cur().chain(maildir.list_new()).flatten();
            for mail in mails {
                debug!("Checking mails");
                if mail.is_trashed() {
                    let path = mail.path();
                    fs::remove_file(path)?;
                }
            }

            {
                write_lock.state = State::Authenticated;
            };
            lines
                .send(format!("{} OK CLOSE completed", command_data.tag))
                .await?;
        } else {
            lines
                .send(format!("{} NO invalid state", command_data.tag))
                .await?;
        }

        Ok(())
    }
}