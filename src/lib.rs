//! # Rustdi - Modern WhatsApp Client Library
//! 
//! Rustdi adalah library Rust modern untuk berinteraksi dengan protokol WhatsApp Web.
//! Library ini dirancang untuk menyediakan antarmuka yang aman, efisien, dan mudah digunakan
//! untuk mengembangkan aplikasi WhatsApp seperti bot, gateway, atau layanan otomasi.

use std::sync::Arc;
use std::collections::HashMap;
use std::thread;
use std::sync::{Mutex, mpsc};
use std::time::{SystemTime, Duration};

use ws::{CloseCode, Handler, Sender, Message};
use ring::{agreement, digest, hmac, hkdf, rand};
use url::Url;
use qrcode::QrCode;
use base64;
use json::JsonValue;

use chrono::{NaiveDateTime, Utc};

// Impor modul internal
pub mod crypto;
pub mod session;
pub mod handshake;
pub mod node_protocol;
pub mod messages;
pub mod errors;

pub use errors::*;

// Re-eksport struktur penting
pub use session::Session;
pub use crypto::{SessionKeys, generate_keypair, derive_session_keys};
pub use node_protocol::{Node, NodeEncoder, NodeDecoder};
pub use messages::*;

// ========================
// STRUKTUR DATA UTAMA
// ========================

/// Identitas pengguna atau grup di WhatsApp
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Jid {
    pub id: String,
    pub is_group: bool,
    pub is_lid: bool, // Logical ID untuk nomor internasional
}

impl Jid {
    pub fn new(id: String, is_group: bool, is_lid: bool) -> Self {
        Jid { id, is_group, is_lid }
    }

    pub fn to_string(&self) -> String {
        if self.is_lid {
            format!("{}@lid", self.id)
        } else if self.is_group {
            format!("{}@g.us", self.id)
        } else {
            format!("{}@s.whatsapp.net", self.id)
        }
    }

    pub fn from_string(jid_str: &str) -> Result<Self> {
        let parts: Vec<&str> = jid_str.split('@').collect();
        if parts.len() != 2 {
            return Err("Invalid JID format".into());
        }

        let id = parts[0].to_string();
        let suffix = parts[1];

        let (is_group, is_lid) = match suffix {
            "s.whatsapp.net" => (false, false),
            "g.us" => (true, false),
            "lid" => (false, true),
            _ => return Err("Unknown JID suffix".into()),
        };

        Ok(Jid { id, is_group, is_lid })
    }

    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() && self.id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }
}

/// Status kehadiran pengguna
#[derive(Debug, Copy, Clone)]
pub enum PresenceStatus {
    Unavailable,
    Available,
    Typing,
    Recording,
}

/// Jenis media yang didukung
#[derive(Debug, Copy, Clone)]
pub enum MediaType {
    Image,
    Video,
    Audio,
    Document,
}

/// Jenis perubahan participant grup
#[derive(Debug, Copy, Clone)]
pub enum GroupParticipantsChange {
    Add,
    Remove,
    Promote,
    Demote,
}

// ========================
// METODE OTENTIKASI
// ========================

/// Metode otentikasi yang tersedia
#[derive(Clone)]
pub enum AuthMethod {
    QRCode { callback: Box<dyn Fn(&QrCode) + Send> },
    PairingCode { phone_number: String, callback: Box<dyn Fn(&str) + Send> },
}

impl Clone for Box<dyn Fn(&QrCode) + Send> {
    fn clone(&self) -> Self {
        Box::new(|_| {})
    }
}

impl Clone for Box<dyn Fn(&str) + Send> {
    fn clone(&self) -> Self {
        Box::new(|_| {})
    }
}

impl Clone for AuthMethod {
    fn clone(&self) -> Self {
        match self {
            AuthMethod::QRCode { .. } => AuthMethod::QRCode {
                callback: Box::new(|_| {}),
            },
            AuthMethod::PairingCode { phone_number, .. } => AuthMethod::PairingCode {
                phone_number: phone_number.clone(),
                callback: Box::new(|_| {}),
            },
        }
    }
}

// ========================
// EVENT HANDLER
// ========================

/// Jenis event yang diterima oleh aplikasi
#[derive(Debug)]
pub enum Event {
    Connected,
    Disconnected,
    Authenticating,
    Authenticated,
    MessageReceived(messages::WebMessageInfo),
    MessageAck(messages::MessageAck),
    PresenceChanged(Jid, PresenceStatus, Option<NaiveDateTime>),
    GroupParticipantsChanged {
        group: Jid,
        change_type: GroupParticipantsChange,
        participants: Vec<Jid>,
    },
    Error(String),
    QrCodeGenerated(String),
    PairingCodeGenerated(String),
}

/// Handler untuk menangani event dari server WhatsApp
pub trait EventHandler: Send + Sync + 'static {
    fn handle_event(&self, event: Event);
}

// ========================
// CLIENT UTAMA
// ========================

/// Status koneksi
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Authenticating,
    Connected,
    Reconnecting,
}

/// Client utama untuk koneksi WhatsApp
pub struct WhatsAppClient {
    id: String,
    state: Arc<Mutex<ConnectionState>>,
    session: Arc<Mutex<Option<session::Session>>>,
    sender: Arc<Mutex<Option<Sender>>>,
    event_handler: Arc<dyn EventHandler>,
    event_tx: mpsc::Sender<Event>,
    event_rx: mpsc::Receiver<Event>,
}

impl WhatsAppClient {
    /// Membuat client baru
    pub fn new(event_handler: Box<dyn EventHandler>) -> Result<Self> {
        let (tx, rx) = mpsc::channel();
        
        let mut id_bytes = [0u8; 16];
        rand::SystemRandom::new().fill(&mut id_bytes).map_err(|_| "Failed to generate ID")?;
        let id = base64::encode_config(&id_bytes, base64::URL_SAFE);

        Ok(WhatsAppClient {
            id,
            state: Arc::new(Mutex::new(ConnectionState::Disconnected)),
            session: Arc::new(Mutex::new(None)),
            sender: Arc::new(Mutex::new(None)),
            event_handler: Arc::from(event_handler),
            event_tx: tx,
            event_rx: rx,
        })
    }

    /// Menghubungkan ke server WhatsApp
    pub fn connect(&self, auth_method: AuthMethod) -> Result<()> {
        let state_clone = Arc::clone(&self.state);
        let sender_clone = Arc::clone(&self.sender);
        let session_clone = Arc::clone(&self.session);
        let event_tx = self.event_tx.clone();
        let id = self.id.clone();

        thread::spawn(move || {
            *state_clone.lock().unwrap() = ConnectionState::Connecting;
            
            let url = Url::parse("wss://web.whatsapp.com/ws")
                .map_err(|e| format!("Invalid WebSocket URL: {}", e))
                .unwrap();

            if let Err(e) = ws::connect(url, |out| {
                *sender_clone.lock().unwrap() = Some(out.clone());
                *state_clone.lock().unwrap() = ConnectionState::Authenticating;
                
                // Kirim event bahwa kita sedang otentikasi
                event_tx.send(Event::Authenticating).ok();

                // Kirim permintaan inisialisasi
                let init_request = json::object! {
                    "id": format!("init_{}", base64::encode(&id.as_bytes())),
                    "type": "init",
                    "version": [2, 3000, 1015901307], // Versi terbaru WhatsApp Web
                    "platform": "chrome"
                };

                out.send(init_request.dump()).ok();

                WsHandler {
                    out,
                    state: state_clone,
                    session: session_clone,
                    event_tx,
                    auth_method: auth_method.clone(),
                    stage: ConnectionStage::Initialized,
                }
            }) {
                event_tx.send(Event::Error(format!("WebSocket connection failed: {}", e))).ok();
                *state_clone.lock().unwrap() = ConnectionState::Disconnected;
            }
        });

        Ok(())
    }

    /// Mengirim pesan teks
    pub fn send_text_message(&self, to: &Jid, text: &str) -> Result<String> {
        let message_id = utils::generate_message_id();

        let message = messages::Message {
            conversation: Some(text.to_string()),
            ..Default::default()
        };

        let web_message = messages::WebMessageInfo {
            key: messages::MessageKey {
                remote_jid: to.to_string(),
                from_me: true,
                id: message_id.clone(),
                participant: None,
            },
            message: Some(message),
            message_timestamp: Some(Utc::now().timestamp() as u64),
            status: Some(1), // PENDING
            ..Default::default()
        };

        self.send_web_message(web_message)?;

        Ok(message_id)
    }

    /// Mengirim pesan media
    pub fn send_media_message(&self, to: &Jid, media_type: MediaType, url: &str, caption: Option<&str>) -> Result<String> {
        let message_id = utils::generate_message_id();

        let message = match media_type {
            MediaType::Image => messages::Message {
                image_message: Some(messages::ImageMessage {
                    url: url.to_string(),
                    caption: caption.map(|s| s.to_string()),
                    mimetype: Some("image/jpeg".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            MediaType::Video => messages::Message {
                video_message: Some(messages::VideoMessage {
                    url: url.to_string(),
                    caption: caption.map(|s| s.to_string()),
                    mimetype: Some("video/mp4".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            MediaType::Audio => messages::Message {
                audio_message: Some(messages::AudioMessage {
                    url: url.to_string(),
                    mimetype: "audio/ogg; codecs=opus".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            },
            MediaType::Document => messages::Message {
                document_message: Some(messages::DocumentMessage {
                    url: url.to_string(),
                    file_name: "file".to_string(), // Harus disediakan dalam implementasi sebenarnya
                    mimetype: Some("application/pdf".to_string()), // Harus sesuai dengan jenis dokumen
                    ..Default::default()
                }),
                ..Default::default()
            },
        };

        let web_message = messages::WebMessageInfo {
            key: messages::MessageKey {
                remote_jid: to.to_string(),
                from_me: true,
                id: message_id.clone(),
                participant: None,
            },
            message: Some(message),
            message_timestamp: Some(Utc::now().timestamp() as u64),
            status: Some(1), // PENDING
            ..Default::default()
        };

        self.send_web_message(web_message)?;

        Ok(message_id)
    }

    /// Mengirim pesan WebMessageInfo
    fn send_web_message(&self, web_message: messages::WebMessageInfo) -> Result<()> {
        let sender_guard = self.sender.lock().unwrap();
        
        if let Some(ref sender) = *sender_guard {
            // Serialisasi WebMessageInfo menjadi protobuf
            let serialized = serde_json::to_string(&web_message).map_err(|e| format!("Serialization error: {}", e))?;
            
            // Encode sebagai node protocol
            use node_protocol::{NodeEncoder, NodeContent};
            let mut encoder = NodeEncoder::new();
            let node = node_protocol::Node {
                tag: "action".to_string(),
                attrs: {
                    let mut attrs = HashMap::new();
                    attrs.insert("type".to_string(), "relay".to_string());
                    attrs.insert("epoch".to_string(), "1".to_string());
                    attrs
                },
                content: Some(NodeContent::Binary(serialized.as_bytes().to_vec())),
            };
            
            encoder.write_node(&node)?;
            sender.send(&encoder.data).map_err(|e| format!("Send error: {}", e).into())?;
        } else {
            return Err("No active connection".into());
        }

        Ok(())
    }

    /// Mengatur status kehadiran
    pub fn set_presence(&self, status: PresenceStatus) -> Result<()> {
        let sender_guard = self.sender.lock().unwrap();
        
        if let Some(ref sender) = *sender_guard {
            let presence_type = match status {
                PresenceStatus::Available => "available",
                PresenceStatus::Unavailable => "unavailable",
                _ => "unavailable", // Default untuk typing/recording
            };

            let presence_msg = json::object! {
                "type": "presence",
                "action": presence_type
            };

            sender.send(presence_msg.dump()).map_err(|e| format!("Failed to send presence: {}", e).into())?;
        } else {
            return Err("No active connection".into());
        }

        Ok(())
    }

    /// Menutup koneksi
    pub fn disconnect(&self) -> Result<()> {
        let mut sender_guard = self.sender.lock().unwrap();
        
        if let Some(ref sender) = *sender_guard {
            sender.close(CloseCode::Normal).ok();
        }
        
        *sender_guard = None;
        *self.state.lock().unwrap() = ConnectionState::Disconnected;

        Ok(())
    }

    /// Menerima event dari server
    pub fn poll_event(&self) -> Option<Event> {
        self.event_rx.try_recv().ok()
    }

    /// Mendapatkan status koneksi
    pub fn get_state(&self) -> ConnectionState {
        *self.state.lock().unwrap()
    }

    /// Mendapatkan ID unik client
    pub fn get_id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionStage {
    Initialized,
    Handshaking,
    Authenticating,
    Connected,
}

/// Handler untuk WebSocket
pub struct WsHandler {
    out: Sender,
    state: Arc<Mutex<ConnectionState>>,
    session: Arc<Mutex<Option<session::Session>>>,
    event_tx: mpsc::Sender<Event>,
    auth_method: AuthMethod,
    stage: ConnectionStage,
}

impl Handler for WsHandler {
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        match msg {
            Message::Text(json_str) => {
                if let Ok(json) = json::parse(&json_str) {
                    self.handle_json_message(json)?;
                }
            }
            Message::Binary(data) => {
                self.handle_binary_message(&data)?;
            }
        }
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closed: {} - {}", code, reason);
        *self.state.lock().unwrap() = ConnectionState::Disconnected;
        
        self.event_tx.send(Event::Disconnected).ok();
    }

    fn on_error(&mut self, err: ws::Error) {
        self.event_tx.send(Event::Error(format!("WebSocket error: {}", err))).ok();
    }
}

impl WsHandler {
    fn handle_json_message(&mut self, json: JsonValue) -> ws::Result<()> {
        if let Some(ref_type) = json["type"].as_str() {
            match ref_type {
                "Conn" => {
                    // Koneksi berhasil, ambil informasi otentikasi
                    if let Some(client_token) = json["clientToken"].as_str() {
                        if let Some(server_token) = json["serverToken"].as_str() {
                            let mut session_guard = self.session.lock().unwrap();
                            
                            if session_guard.is_none() {
                                *session_guard = Some(session::Session::new());
                            }
                            
                            if let Some(ref mut session) = *session_guard {
                                session.set_auth_tokens(client_token.to_string(), server_token.to_string());
                                
                                // Jika ada secret, proses handshake
                                if let Some(secret) = json["secret"].as_str() {
                                    // Proses secret untuk menghasilkan kunci enkripsi
                                    self.process_secret(secret)?;
                                }
                            }
                            
                            // Kirim event otentikasi
                            self.event_tx.send(Event::Authenticated).ok();
                            *self.state.lock().unwrap() = ConnectionState::Connected;
                        }
                    }
                }
                "ref" => {
                    // Ini adalah QR code reference
                    if let Some(ref_val) = json["ref"].as_str() {
                        match &self.auth_method {
                            AuthMethod::QRCode { callback } => {
                                // Bangun QR code
                                let qr_data = format!("{},{}", ref_val, "PLACEHOLDER_PUBLIC_KEY");
                                if let Ok(qr_code) = QrCode::new(qr_data.as_bytes()) {
                                    callback(&qr_code);
                                    self.event_tx.send(Event::QrCodeGenerated(qr_data)).ok();
                                }
                            }
                            _ => {
                                // Tidak menggunakan QR code
                            }
                        }
                    }
                }
                _ => {
                    // Tangani pesan lainnya
                }
            }
        }
        
        Ok(())
    }

    fn handle_binary_message(&mut self, data: &[u8]) -> ws::Result<()> {
        use node_protocol::NodeDecoder;
        
        let mut decoder = NodeDecoder::new(data);
        if let Ok(node) = decoder.read_node() {
            // Dalam implementasi asli, ini akan meng-parse node sebagai WebMessageInfo
            // Untuk sekarang kita kirim event kosong
            if node.tag == "message" {
                // Coba parse sebagai WebMessageInfo jika konten binari
                if let Some(node_protocol::NodeContent::Binary(bytes)) = node.content {
                    if let Ok(web_message) = serde_json::from_slice::<messages::WebMessageInfo>(&bytes) {
                        self.event_tx.send(Event::MessageReceived(web_message)).ok();
                    }
                }
            }
        }
        
        Ok(())
    }

    fn process_secret(&mut self, secret_base64: &str) -> Result<()> {
        // Proses secret dari server untuk menyelesaikan handshake Noise
        let secret = base64::decode(secret_base64).map_err(|e| format!("Failed to decode secret: {}", e))?;
        
        if secret.len() != 144 {
            return Err("Invalid secret length".into());
        }

        // Extract components
        let server_identity_public = &secret[0..32];
        let expected_hmac = &secret[32..64];
        let encrypted_keys = &secret[64..];

        // Generate session keys
        let session_keys = crypto::derive_session_keys(
            server_identity_public,
            expected_hmac,
            encrypted_keys
        )?;

        // Simpan kunci ke session
        let mut session_guard = self.session.lock().unwrap();
        if let Some(ref mut session) = *session_guard {
            session.update_encryption_keys(session_keys.enc_key, session_keys.mac_key);
        }

        Ok(())
    }
}

// ========================
// FUNGSI UTILITAS
// ========================

/// Fungsi bantuan untuk developer
pub mod utils {
    use super::*;

    /// Memformat nomor telepon ke format WhatsApp
    pub fn format_phone_number(phone: &str) -> String {
        phone.chars().filter(|c| c.is_digit(10)).collect::<String>().trim_start_matches('0').to_string()
    }

    /// Memvalidasi apakah sebuah JID valid
    pub fn is_valid_jid(jid: &str) -> bool {
        jid.contains('@') && (jid.ends_with("@s.whatsapp.net") || jid.ends_with("@g.us") || jid.ends_with("@lid"))
    }

    /// Mengenerate ID pesan unik
    pub fn generate_message_id() -> String {
        format!("msg_{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos())
    }
}

// Implementasi untuk Clone
impl Clone for WhatsAppClient {
    fn clone(&self) -> Self {
        WhatsAppClient {
            id: self.id.clone(),
            state: Arc::clone(&self.state),
            session: Arc::clone(&self.session),
            sender: Arc::clone(&self.sender),
            event_handler: Arc::clone(&self.event_handler),
            event_tx: self.event_tx.clone(),
            event_rx: self.event_rx.try_clone().unwrap(),
        }
    }
}

// Builder untuk WhatsAppClient
pub struct WhatsAppClientBuilder {
    event_handler: Option<Box<dyn EventHandler>>,
}

impl WhatsAppClientBuilder {
    pub fn new() -> Self {
        WhatsAppClientBuilder {
            event_handler: None,
        }
    }

    pub fn with_event_handler(mut self, handler: Box<dyn EventHandler>) -> Self {
        self.event_handler = Some(handler);
        self
    }

    pub fn build(self) -> Result<WhatsAppClient> {
        match self.event_handler {
            Some(handler) => WhatsAppClient::new(handler),
            None => Err("Event handler is required".into()),
        }
    }
}