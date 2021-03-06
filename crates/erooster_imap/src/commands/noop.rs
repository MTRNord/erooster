use crate::commands::CommandData;
use futures::{channel::mpsc::SendError, Sink, SinkExt};
use tracing::instrument;

pub struct Noop;

impl Noop {
    #[instrument(skip(self, lines, command_data))]
    pub async fn exec<S>(
        &self,
        lines: &mut S,
        command_data: &CommandData<'_>,
    ) -> color_eyre::eyre::Result<()>
    where
        S: Sink<String, Error = SendError> + std::marker::Unpin + std::marker::Send,
    {
        // TODO return status as suggested in https://www.rfc-editor.org/rfc/rfc9051.html#name-noop-command
        lines
            .send(format!("{} OK NOOP completed", command_data.tag))
            .await?;
        Ok(())
    }
}
