use std::collections::HashMap;

use futures::SinkExt as _;

use crate::hmr::{HmrEvent, HmrEventSendError, WsError, WsMessage, WsResult, WsStream};

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum HmrMessageError {
    #[error("hmr message channel closed")]
    ChannelClosed(#[from] HmrEventSendError<HmrEvent>),
    #[error("send ws message")]
    SendWsMessage(#[source] WsError),
}

pub type HmrMessageResult<T> = Result<T, HmrMessageError>;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum HmrMessage {
    Connected,
    Ping,
    Update {
        updates: Vec<Update>,
    },
    /// Signals that modules are dead.
    /// Remove them from the module graph.
    Prune {
        paths: Vec<String>,
    },
    /// Reload the entire application.
    FullReload {
        #[serde(default)]
        path: Option<String>,
        #[serde(default)]
        triggered_by: Option<String>,
    },
    Custom(CustomMessage),
    Error(#[serde(rename = "err")] HmrError),
}

#[derive(Debug, thiserror::Error, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[error("{message}")]
pub struct HmrError {
    pub message: String,
    pub stack: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub frame: Option<String>,
    #[serde(default)]
    pub plugin: Option<String>,
    #[serde(default)]
    pub plugin_code: Option<String>,
    #[serde(default)]
    pub loc: Option<HmrErrorLoc>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl miette::Diagnostic for HmrError {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.plugin_code
            .as_ref()
            .map(|code| Box::new(code.clone()) as Box<dyn std::fmt::Display + 'a>)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HmrErrorLoc {
    #[serde(default)]
    pub file: Option<String>,
    pub line: u32,
    pub column: u32,
}

#[derive(serde::Deserialize)]
#[serde(tag = "event", rename_all = "kebab-case")]
pub enum CustomMessage {}

#[derive(serde::Deserialize)]
#[serde(tag = "type")]
pub enum Update {
    #[serde(rename = "js-update", alias = "css-update")]
    #[serde(rename_all = "camelCase")]
    Update {
        path: String,
        accepted_path: String,
        #[serde(with = "chrono::serde::ts_milliseconds")]
        timestamp: chrono::DateTime<chrono::Utc>,
        #[serde(default)]
        explicit_import_required: Option<bool>,
        #[serde(default)]
        is_within_circular_import: Option<bool>,
        #[serde(default)]
        first_invalidated_by: Option<String>,
        #[serde(default)]
        invalidates: Vec<String>,
    },
}

impl HmrMessage {
    pub async fn handle(
        self,
        ws_stream: &mut WsStream,
        event_tx: &tokio::sync::mpsc::UnboundedSender<HmrEvent>,
    ) -> miette::Result<()> {
        match self {
            Self::Connected => Self::handle_connected(),
            Self::Ping => Self::handle_ping(ws_stream)
                .await
                .map_err(HmrMessageError::SendWsMessage)?,
            Self::Update { updates } => Self::handle_update(updates, event_tx)?,

            Self::Prune { paths } => Self::handle_prune(paths, event_tx)?,
            Self::FullReload { path, triggered_by } => {
                Self::handle_full_reload(path, triggered_by, event_tx)?;
            }
            Self::Custom(message) => Self::handle_custom(message),
            Self::Error(e) => Self::handle_error(e),
        }
        Ok(())
    }

    fn handle_connected() {
        tracing::info!("HMR connection established.");
    }

    async fn handle_ping(ws_stream: &mut WsStream) -> WsResult<()> {
        tracing::info!("Received ping.");
        ws_stream.send(WsMessage::Pong(vec![].into())).await?;
        Ok(())
    }

    fn handle_update(
        updates: Vec<Update>,
        event_tx: &tokio::sync::mpsc::UnboundedSender<HmrEvent>,
    ) -> HmrMessageResult<()> {
        let paths: Vec<String> = updates
            .into_iter()
            .map(|u| match u {
                Update::Update { accepted_path, .. } => {
                    tracing::info!("Update {accepted_path}");
                    accepted_path
                }
            })
            .collect();
        if !paths.is_empty() {
            event_tx.send(HmrEvent::Update { paths })?;
        }
        Ok(())
    }

    fn handle_prune(
        paths: Vec<String>,
        event_tx: &tokio::sync::mpsc::UnboundedSender<HmrEvent>,
    ) -> HmrMessageResult<()> {
        for path in &paths {
            tracing::info!("Prune {path}");
        }
        event_tx.send(HmrEvent::Prune { paths })?;
        Ok(())
    }

    fn handle_full_reload(
        path: Option<String>,
        triggered_by: Option<String>,
        event_tx: &tokio::sync::mpsc::UnboundedSender<HmrEvent>,
    ) -> HmrMessageResult<()> {
        if let Some(path) = path {
            tracing::info!("Full reload from {path}.");
        }
        if let Some(triggered_by) = triggered_by {
            tracing::info!("Full reload triggered by {triggered_by}.");
        } else {
            tracing::info!("Full reload.");
        }
        event_tx.send(HmrEvent::FullReload)?;
        Ok(())
    }

    #[allow(clippy::needless_pass_by_value, reason = "uninhabited enum")]
    const fn handle_custom(message: CustomMessage) {
        match message {}
    }

    fn handle_error(e: HmrError) {
        tracing::error!("HMR error.");
        let report = miette::Report::from_err(e);
        eprintln!("{report:?}");
    }
}
