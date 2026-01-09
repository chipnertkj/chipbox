mod payload;
mod task;

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

type Send = tokio::sync::mpsc::UnboundedSender<HmrEvent>;
type Recv = tokio::sync::mpsc::UnboundedReceiver<HmrEvent>;
type WsStream =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;
type WsMessage = tokio_tungstenite::tungstenite::Message;
type HmrEventSendError<T> = tokio::sync::mpsc::error::SendError<T>;
type WsError = tokio_tungstenite::tungstenite::error::Error;
type WsResult<T> = Result<T, WsError>;

pub struct HmrRecv {
    recv: Recv,
}

impl HmrRecv {
    /// Receive the next HMR event, waiting if necessary.
    pub async fn recv(&mut self) -> Option<HmrEvent> {
        self.recv.recv().await
    }

    /// Try to receive an HMR event without blocking.
    pub fn try_recv(&mut self) -> Option<HmrEvent> {
        self.recv.try_recv().ok()
    }
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum HmrConnectError {
    #[error("connection timed out")]
    Timeout(#[source] tokio::time::error::Elapsed),
    #[error("connection failed")]
    Connection(#[source] tokio_tungstenite::tungstenite::Error),
    #[error("unexpected status code {0}")]
    BadStatus(reqwest::StatusCode),
}

pub type HmrConnectResult<T> = Result<T, HmrConnectError>;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum HmrClientError {
    #[error("connect to hmr ws")]
    Connect(#[source] HmrConnectError),
}

pub type HmrClientResult<T> = Result<T, HmrClientError>;

pub struct HmrClient {
    /// WebSocket message stream.
    ws_stream: WsStream,
    /// Event transmitter for HMR events.
    send: Send,
}

impl HmrClient {
    fn ws_url(port: u16) -> url::Url {
        url::Url::parse(&format!("ws://localhost:{port}/@vite-hmr")).expect("valid hmr ws url")
    }

    pub async fn new(port: u16) -> HmrClientResult<(Self, HmrRecv)> {
        let ws_stream = Self::connect(Self::ws_url(port))
            .await
            .map_err(HmrClientError::Connect)?;
        let (send, recv) = tokio::sync::mpsc::unbounded_channel();
        let client = Self { ws_stream, send };
        Ok((client, HmrRecv { recv }))
    }

    pub async fn start(mut self) -> miette::Result<()> {
        task::handle_messages(&mut self.ws_stream, &self.send).await
    }

    async fn connect(url: url::Url) -> HmrConnectResult<WsStream> {
        let (ws_stream, response) = tokio::time::timeout(
            tokio::time::Duration::from_millis(500),
            tokio_tungstenite::connect_async(url.to_string()),
        )
        .await
        .map_err(HmrConnectError::Timeout)?
        .map_err(HmrConnectError::Connection)?;

        if response.status() != reqwest::StatusCode::SWITCHING_PROTOCOLS {
            return Err(HmrConnectError::BadStatus(response.status()));
        }
        tracing::info!("Connected to websocket.");

        Ok(ws_stream)
    }
}
