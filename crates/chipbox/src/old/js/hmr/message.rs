//! HMR message handling.

use std::ops::ControlFlow;

use futures::{SinkExt as _, StreamExt as _};
use miette::{Context as _, IntoDiagnostic as _};
use tokio_tungstenite::tungstenite;

use super::{EventTx, WsMessage, WsStream, payload::HmrMessage};

/// Await and handle every message from the HMR websocket stream until the connection is closed.
pub async fn handle_messages(ws_stream: &mut WsStream, event_tx: &EventTx) -> miette::Result<()> {
    while let Some(msg) = ws_stream.next().await {
        let msg = msg.into_diagnostic().wrap_err("read message")?;
        let control_flow = handle_message(msg, ws_stream, event_tx).await?;
        match control_flow {
            ControlFlow::Continue(()) => (),
            ControlFlow::Break(()) => break,
        }
    }
    Ok(())
}

/// Handle a message from the HMR server.
async fn handle_message(
    msg: WsMessage,
    ws_stream: &mut WsStream,
    event_tx: &EventTx,
) -> miette::Result<ControlFlow<()>> {
    match msg {
        WsMessage::Text(utf8_bytes) => handle_text_message(ws_stream, event_tx, utf8_bytes).await?,
        WsMessage::Close(frame) => {
            handle_close_frame(frame);
            return Ok(ControlFlow::Break(()));
        }
        WsMessage::Ping(data) => handle_ping(ws_stream, data).await?,
        WsMessage::Pong(data) => tracing::info!("Received pong ({} bytes).", data.len()),
        WsMessage::Binary(data) => tracing::info!("Received binary data ({} bytes).", data.len()),
        WsMessage::Frame(_) => miette::bail!("Received raw frame while reading WS message!"),
    }
    Ok(ControlFlow::Continue(()))
}

/// Handle a ping message from the HMR server.
///
/// Sends a pong back to the HMR server with the same data that was received.
async fn handle_ping(
    ws_stream: &mut WsStream,
    data: tungstenite::Bytes,
) -> Result<(), miette::Error> {
    tracing::info!("Received ping ({} bytes).", data.len());
    ws_stream
        .send(WsMessage::Pong(data))
        .await
        .into_diagnostic()
        .wrap_err("send pong")?;
    Ok(())
}

/// Handle a close frame from the HMR server.
fn handle_close_frame(frame: Option<tungstenite::protocol::CloseFrame>) {
    use tungstenite::protocol::CloseFrame;
    let info = frame.map(|CloseFrame { code, reason }| format!("[{code}]: {reason}"));
    let info_str = info.as_deref().map_or("no close frame.", str::as_ref);
    tracing::info!("Connection closed by server: {info_str}");
}

/// Handle a text message from the HMR server.
///
/// If the message cannot be parsed, an error is logged. The function still returns `Ok(())` in this case.
async fn handle_text_message(
    ws_stream: &mut WsStream,
    event_tx: &EventTx,
    utf8_bytes: tungstenite::Utf8Bytes,
) -> miette::Result<()> {
    tracing::trace!("Received text message: {utf8_bytes:?}");
    let parse_result = serde_json::from_str::<HmrMessage>(&utf8_bytes).into_diagnostic();
    match parse_result {
        Ok(hmr_message) => hmr_message
            .handle(ws_stream, event_tx)
            .await
            .wrap_err("handle hmr message"),
        Err(e) => {
            tracing::error!("Failed to parse HMR message.");
            eprintln!("{e:?}");
            Ok(())
        }
    }
}
