use miette::{Context as _, IntoDiagnostic as _};
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite};
use url::Url;

type WsMessage = tungstenite::Message;
type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub type EventTx = mpsc::UnboundedSender<HmrEvent>;
pub type EventRx = mpsc::UnboundedReceiver<HmrEvent>;

mod message;
pub mod payload;

fn ws_url() -> Url {
    Url::parse("ws://localhost:24678/@vite-hmr").expect("valid hmr ws url")
}

/// HMR event sent to JS runtime.
#[derive(Debug, Clone)]
pub enum HmrEvent {
    /// Reimport updated modules.
    Update { paths: Vec<String> },
    /// Prune modules from cache.
    Prune { paths: Vec<String> },
    /// Full reload required. Restart the application.
    FullReload,
}

pub struct HmrClient {
    /// WebSocket message stream.
    ws_stream: WsStream,
    /// Event transmitter for HMR events.
    event_tx: EventTx,
}

impl HmrClient {
    pub async fn new() -> miette::Result<(Self, EventRx)> {
        let ws_stream = connect().await.wrap_err("connect to hmr ws")?;
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let client = Self {
            ws_stream,
            event_tx,
        };
        Ok((client, event_rx))
    }

    pub async fn drive(mut self) -> miette::Result<()> {
        message::handle_messages(&mut self.ws_stream, &self.event_tx)
            .await
            .wrap_err("handle messages")
    }
}

async fn connect() -> miette::Result<WsStream> {
    let (ws_stream, response) = tokio::time::timeout(
        tokio::time::Duration::from_millis(500),
        tokio_tungstenite::connect_async(ws_url().to_string()),
    )
    .await
    // Timeout error.
    .into_diagnostic()
    .wrap_err("connect to ws timeout")?
    // Connection error.
    .into_diagnostic()
    .wrap_err("connect to ws")?;

    if response.status() != reqwest::StatusCode::SWITCHING_PROTOCOLS {
        miette::bail!("HMR connection failed: {}", response.status());
    }
    tracing::info!("Connected to websocket.");

    Ok(ws_stream)
}
