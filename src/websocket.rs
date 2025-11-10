//! Implementasi WebSocket modern untuk Rustdi
//! 
//! Menggunakan pendekatan async dan protokol WhatsApp Web Multi-Device

use crate::errors::*;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};
use url::Url;
use tokio::time::timeout;
use std::time::Duration;

// Konstanta endpoint dan timeout
const WEBSOCKET_ENDPOINT: &str = "wss://web.whatsapp.com/ws/chat";
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const PING_INTERVAL: Duration = Duration::from_secs(25);
const READ_TIMEOUT: Duration = Duration::from_secs(30);

/// Struktur untuk koneksi WebSocket
pub struct WebSocketConnection {
    pub url: Url,
    pub connected: bool,
    pub session_token: Option<String>,
}

impl WebSocketConnection {
    /// Membuat koneksi WebSocket baru
    pub fn new(url: Option<String>) -> Result<Self> {
        let ws_url = match url {
            Some(u) => Url::parse(&u)?,
            None => Url::parse(WEBSOCKET_ENDPOINT)
                .map_err(|e| format!("Invalid WebSocket URL: {}", e))?,
        };
        
        Ok(WebSocketConnection {
            url: ws_url,
            connected: false,
            session_token: None,
        })
    }
    
    /// Membuka koneksi WebSocket
    pub async fn connect(&mut self) -> Result<tokio_tungstenite::WebSocketStream<tokio_tungstenite::connect_async::ConnectStream>> {
        let (ws_stream, _) = timeout(
            CONNECT_TIMEOUT,
            connect_async(self.url.clone())
        )
        .await
        .map_err(|_| "Connection timeout")?
        .map_err(|e| format!("WebSocket connection failed: {}", e))?;
        
        self.connected = true;
        
        Ok(ws_stream)
    }
    
    /// Mengirim pesan melalui WebSocket
    pub async fn send_message(
        ws_stream: &mut tokio_tungstenite::WebSocketStream<tokio_tungstenite::connect_async::ConnectStream>,
        message: &str
    ) -> Result<()> {
        ws_stream
            .send(Message::Text(message.to_string()))
            .await
            .map_err(|e| format!("Failed to send message: {}", e).into())
    }
    
    /// Menerima pesan dari WebSocket
    pub async fn receive_message(
        ws_stream: &mut tokio_tungstenite::WebSocketStream<tokio_tungstenite::connect_async::ConnectStream>
    ) -> Result<Option<String>> {
        match timeout(READ_TIMEOUT, ws_stream.next()).await {
            Ok(Some(Ok(Message::Text(text)))) => Ok(Some(text)),
            Ok(Some(Ok(Message::Binary(_)))) => Err("Binary message received".into()),
            Ok(Some(Err(e))) => Err(format!("WebSocket error: {}", e).into()),
            Ok(None) => Ok(None), // Koneksi ditutup
            Err(_) => Err("Read timeout".into()),
        }
    }
    
    /// Menutup koneksi WebSocket
    pub async fn disconnect(
        ws_stream: &mut tokio_tungstenite::WebSocketStream<tokio_tungstenite::connect_async::ConnectStream>
    ) -> Result<()> {
        ws_stream
            .close(None)
            .await
            .map_err(|e| format!("Failed to close connection: {}", e).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_websocket_connection_creation() {
        let conn = WebSocketConnection::new(None);
        assert!(conn.is_ok());
    }
}