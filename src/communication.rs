use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;

pub async fn send_message<T, M>(
    ws_stream: &mut WebSocketStream<T>,
    message: &M,
) -> Result<(), Box<dyn Error>>
where
    T: AsyncRead + AsyncWrite + Unpin,
    M: Serialize,
{
    let serialized = serde_json::to_string(message)?;
    ws_stream.send(Message::Text(serialized)).await?;
    Ok(())
}

pub async fn receive_message<T, M>(ws_stream: &mut WebSocketStream<T>) -> Result<M, Box<dyn Error>>
where
    T: AsyncRead + AsyncWrite + Unpin,
    M: for<'de> Deserialize<'de>,
{
    if let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if let Message::Text(text) = msg {
            let deserialized: M = serde_json::from_str(&text)?;
            return Ok(deserialized);
        }
    }
    Err("Failed to receive message".into())
}
