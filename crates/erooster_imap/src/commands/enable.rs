use crate::{
    commands::{CommandData, Data},
    state::Capabilities,
};
use futures::{channel::mpsc::SendError, Sink, SinkExt};
use tracing::instrument;

pub struct Enable<'a> {
    pub data: &'a Data,
}

impl Enable<'_> {
    #[instrument(skip(self, lines, command_data))]
    pub async fn exec<S>(
        &self,
        lines: &mut S,
        command_data: &CommandData<'_>,
    ) -> color_eyre::eyre::Result<()>
    where
        S: Sink<String, Error = SendError> + std::marker::Unpin + std::marker::Send,
    {
        let mut write_lock = self.data.con_state.write().await;

        for arg in command_data.arguments.iter() {
            if arg == &"UTF8=ACCEPT" {
                write_lock.active_capabilities.push(Capabilities::UTF8);
                lines.feed(format!("* ENABLED {}", arg)).await?;
            } else {
                write_lock
                    .active_capabilities
                    .push(Capabilities::Other((*arg).to_string()));
            }
        }
        lines.feed(format!("{} OK", command_data.tag)).await?;
        lines.flush().await?;
        Ok(())
    }
}
