use std::sync::Arc;
use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;
use std::sync::{Mutex, mpsc};
use std::time::{SystemTime, Duration};
use std::str::FromStr;

use ws;
use ws::{CloseCode, Handler, Request, Sender, Message};
use ring::agreement;
use ring::rand::{SystemRandom, SecureRandom};
use url::Url;
use qrcode::QrCode;
use base64;
use json::JsonValue;
use ws::util::{Token, Timeout};

use chrono::{NaiveDateTime, Utc};

use crate::session_new::{Session, MultiDeviceSession};
use crate::node_protocol_new::{Node, NodeEncoder, NodeDecoder};
use crate::{Jid, PresenceStatus, Contact, Chat, GroupMetadata, GroupParticipantsChange, ChatAction, MediaType};
use crate::{messages_extended, message_converter};
use crate::errors::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum State {
    Uninitialized = 0,
    Connected = 1,
    Disconnecting = 2,
    Reconnecting = 3,
    Pairing = 4,
}

pub enum DisconnectReason {
    Replaced,
    Removed,
    PairingFailed,
}

#[derive(Debug)]
pub enum Event {
    StateChange(State),
    UserData(UserData),
    MessageReceived(bool, Box<messages_extended::WebMessageInfo>),
    Disconnected(DisconnectReason),
}

#[derive(Debug)]
pub enum UserData {
    ContactsInitial(Vec<Contact>),
    ContactAddChange(Contact),
    ContactDelete(Jid),
    Chats(Vec<Chat>),
    ChatAction(Jid, ChatAction),
    UserJid(Jid),
    PresenceChange(Jid, PresenceStatus, Option<NaiveDateTime>),
    MessageAck(MessageAckExtended),
    GroupIntroduce { newly_created: bool, inducer: Jid, meta: GroupMetadata },
    GroupParticipantsChange { group: Jid, change: GroupParticipantsChange, inducer: Option<Jid>, participants: Vec<Jid> },
    Battery(u8),
    PairingCode(String),
}

#[derive(Debug)]
pub struct MessageAckExtended {
    pub level: u32,
    pub message_id: String,
    pub time: Option<i64>,
}

pub trait EventHandler: Send + Sync + 'static {
    fn handle_event(&self, event: Event);
}

// Struktur utama WhatsApp Client
pub struct WhatsAppClient {
    sender: Arc<Mutex<Option<Sender>>>,
    session: Arc<Mutex<Session>>,
    multi_device_session: Arc<Mutex<MultiDeviceSession>>,
    state: Arc<Mutex<State>>,
    event_handler: Arc<dyn EventHandler>,
    client_id: String,
    websocket_url: String,
}

impl WhatsAppClient {
    pub fn new(event_handler: Box<dyn EventHandler>) -> Result<Self> {
        let session = Session::new();
        
        Ok(WhatsAppClient {
            sender: Arc::new(Mutex::new(None)),
            session: Arc::new(Mutex::new(session)),
            multi_device_session: Arc::new(Mutex::new(MultiDeviceSession::default())),
            state: Arc::new(Mutex::new(State::Uninitialized)),
            event_handler: Arc::from(event_handler),
            client_id: generate_client_id(),
            websocket_url: "wss://web.whatsapp.com/ws".to_string(),
        })
    }

    pub fn connect_with_qr(&self, qr_callback: Box<dyn Fn(&str) + Send>) -> Result<()> {
        let mut session_guard = self.session.lock().unwrap();
        session_guard.client_id = self.client_id.clone();
        drop(session_guard);

        self.connect_with_callback(ConnectionType::QR(qr_callback))
    }

    pub fn connect_with_pairing_code(&self, phone_number: &str, is_temporary: bool) -> Result<String> {
        let pairing_code = generate_pairing_code();
        let mut session_guard = self.session.lock().unwrap();
        session_guard.client_id = self.client_id.clone();
        drop(session_guard);

        // Kirim event pairing code ke handler
        self.event_handler.handle_event(Event::UserData(
            UserData::PairingCode(pairing_code.clone())
        ));

        self.connect_with_callback(ConnectionType::PairingCode(
            phone_number.to_string(),
            is_temporary,
            Box::new(move |_| {})
        ))?;

        Ok(pairing_code)
    }

    fn connect_with_callback(&self, conn_type: ConnectionType) -> Result<()> {
        let url = self.websocket_url.clone();
        let sender_clone = Arc::clone(&self.sender);
        let session_clone = Arc::clone(&self.session);
        let state_clone = Arc::clone(&self.state);
        let handler_clone = Arc::clone(&self.event_handler);
        let client_id = self.client_id.clone();

        thread::spawn(move || {
            if let Err(e) = ws::connect(url, |out| {
                *sender_clone.lock().unwrap() = Some(out.clone());
                
                // Kirim inisialisasi koneksi
                let init_msg = json::object! {
                    "id": format!("init_{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()),
                    "type": "init",
                    "client_id": client_id,
                    "version": [2, 2142, 12],
                    "platform": "rustdi"
                };
                
                if let Err(_) = out.send(init_msg.dump()) {
                    return;
                }

                // Kirim status koneksi ke handler
                handler_clone.handle_event(Event::StateChange(State::Connected));
                *state_clone.lock().unwrap() = State::Connected;

                WsHandler {
                    out,
                    session: session_clone,
                    event_handler: handler_clone,
                    state: state_clone,
                }
            }) {
                eprintln!("WebSocket connection error: {}", e);
            }
        });

        Ok(())
    }

    pub fn disconnect(&self) -> Result<()> {
        let mut state_guard = self.state.lock().unwrap();
        *state_guard = State::Disconnecting;
        
        if let Ok(sender_guard) = self.sender.lock() {
            if let Some(ref sender) = *sender_guard {
                sender.close(CloseCode::Normal).ok();
            }
        }
        
        self.event_handler.handle_event(Event::Disconnected(DisconnectReason::Removed));
        Ok(())
    }

    pub fn send_message(&self, jid: Jid, content: messages_extended::Message) -> Result<String> {
        let message_id = format!("msg_{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos());
        
        // Kirim sebagai WebMessageInfo
        let web_message = messages_extended::WebMessageInfo {
            key: messages_extended::MessageKey {
                remote_jid: jid.to_string(),
                from_me: true,
                id: message_id.clone(),
                participant: None,
            },
            message: Some(Box::new(content)),
            message_timestamp: Some(Utc::now().timestamp() as u64),
            status: Some(1), // PENDING
            participant: None,
            ignore: None,
            starred: None,
            broadcast: None,
            push_name: None,
            media_ciphertext_sha256: None,
            multicast: None,
            url_text: None,
            url_number: None,
            message_stub_type: None,
            clear_media: None,
            message_stub_parameters: vec![],
            duration: None,
            labels: vec![],
            payment_info: None,
            final_live_location: None,
            quoted_payment_info: None,
            ephemeral_start_timestamp: None,
            ephemeral_duration: None,
            ephemeral_off_to_on: None,
            ephemeral_out_of_sync: None,
            biz_privacy_status: None,
            verified_biz_name: None,
        };

        // Konversi ke node dan kirim
        let node = self.create_message_node(web_message)?;
        self.send_node(node)?;

        Ok(message_id)
    }

    fn create_message_node(&self, web_message: messages_extended::WebMessageInfo) -> Result<Node> {
        let mut attrs = HashMap::new();
        attrs.insert("type".to_string(), "chat".to_string());
        
        let mut node_encoder = NodeEncoder::new();
        let message_bytes = serde_json::to_vec(&web_message)?;
        
        Ok(Node {
            tag: "message".to_string(),
            attrs,
            content: Some(crate::node_protocol_new::NodeContent::Binary(message_bytes)),
        })
    }

    fn send_node(&self, node: Node) -> Result<()> {
        if let Ok(sender_guard) = self.sender.lock() {
            if let Some(ref sender) = *sender_guard {
                let mut encoder = NodeEncoder::new();
                encoder.write_node(&node)?;
                let data = encoder.data;
                
                // Kirim dalam format binary node
                sender.send(&data).map_err(|e| Error::from(format!("Failed to send node: {}", e)))?;
                return Ok(());
            }
        }
        Err("WebSocket sender not available".into())
    }

    pub fn send_read_receipt(&self, chat_jid: &Jid, message_id: &str) -> Result<()> {
        let receipt_node = Node {
            tag: "receipt".to_string(),
            attrs: {
                let mut attrs = HashMap::new();
                attrs.insert("to".to_string(), chat_jid.to_string());
                attrs.insert("id".to_string(), message_id.to_string());
                attrs.insert("type".to_string(), "read".to_string());
                attrs
            },
            content: None,
        };
        
        self.send_node(receipt_node)
    }

    pub fn set_presence(&self, presence: PresenceStatus) -> Result<()> {
        let presence_str = match presence {
            PresenceStatus::Available => "available",
            PresenceStatus::Unavailable => "unavailable",
            PresenceStatus::Typing => "composing", // Actually means "typing"
            PresenceStatus::Recording => "recording", // Actually means "recording"
        };

        let presence_node = Node {
            tag: "presence".to_string(),
            attrs: {
                let mut attrs = HashMap::new();
                attrs.insert("type".to_string(), presence_str.to_string());
                attrs
            },
            content: None,
        };
        
        self.send_node(presence_node)
    }

    pub fn get_group_metadata(&self, jid: &Jid) -> Result<GroupMetadata> {
        // Ini hanya placeholder, implementasi sebenarnya akan mengirim permintaan ke server
        Err("Not implemented".into())
    }

    pub fn create_group(&self, subject: &str, participants: Vec<Jid>) -> Result<GroupMetadata> {
        // Ini hanya placeholder, implementasi sebenarnya akan mengirim permintaan ke server
        Err("Not implemented".into())
    }

    pub fn add_participants(&self, group_jid: &Jid, participants: Vec<Jid>) -> Result<()> {
        // Ini hanya placeholder, implementasi sebenarnya akan mengirim permintaan ke server
        Ok(())
    }

    pub fn remove_participants(&self, group_jid: &Jid, participants: Vec<Jid>) -> Result<()> {
        // Ini hanya placeholder, implementasi sebenarnya akan mengirim permintaan ke server
        Ok(())
    }
}

// Tipe koneksi yang digunakan
enum ConnectionType {
    QR(Box<dyn Fn(&str) + Send>),
    PairingCode(String, bool, Box<dyn Fn(&str) + Send>),
}

// Fungsi bantu untuk generate ID
fn generate_client_id() -> String {
    let mut bytes = [0u8; 16];
    SystemRandom::new().fill(&mut bytes).unwrap();
    base64::encode(&bytes)
}

fn generate_pairing_code() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let code: String = (0..9)
        .map(|_| {
            if rng.gen::<bool>() && rng.gen::<f32>() < 0.5 {
                '-'
            } else if rng.gen::<f32>() < 0.5 {
                '.'
            } else {
                char::from_digit(rng.gen::<u8>() % 10 as u8, 10).unwrap()
            }
        })
        .collect();
    
    // Format menjadi "XXX-XXX-XXX"
    format!("{}{}{}-{}{}{}-{}{}{}", 
        code.chars().nth(0).unwrap_or('0'),
        code.chars().nth(1).unwrap_or('0'),
        code.chars().nth(2).unwrap_or('0'),
        code.chars().nth(3).unwrap_or('0'),
        code.chars().nth(4).unwrap_or('0'),
        code.chars().nth(5).unwrap_or('0'),
        code.chars().nth(6).unwrap_or('0'),
        code.chars().nth(7).unwrap_or('0'),
        code.chars().nth(8).unwrap_or('0'))
}

// Handler WebSocket
struct WsHandler {
    out: Sender,
    session: Arc<Mutex<Session>>,
    event_handler: Arc<dyn EventHandler>,
    state: Arc<Mutex<State>>,
}

impl Handler for WsHandler {
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        // Tangani pesan dari server
        match msg {
            Message::Text(text) => {
                // Tangani pesan JSON
                if let Ok(json_value) = json::parse(&text) {
                    self.handle_json_message(json_value)?;
                }
            }
            Message::Binary(data) => {
                // Tangani pesan binari (node protocol)
                self.handle_binary_message(&data)?;
            }
        }
        
        Ok(())
    }
    
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        // Tangani penutupan koneksi
        println!("Connection closed: {} - {}", code, reason);
        *self.state.lock().unwrap() = State::Disconnected;
        
        // Kirim event disconnected
        self.event_handler.handle_event(Event::Disconnected(match code {
            CloseCode::Normal => DisconnectReason::Removed,
            CloseCode::Abnormal => DisconnectReason::Removed,
            _ => DisconnectReason::Removed,
        }));
    }
}

impl WsHandler {
    fn handle_json_message(&mut self, json: JsonValue) -> ws::Result<()> {
        // Tangani pesan JSON dari server
        if let Some(status) = json["status"].as_u32() {
            if status == 200 {
                // Koneksi berhasil
                self.event_handler.handle_event(Event::StateChange(State::Connected));
            }
        }
        
        Ok(())
    }
    
    fn handle_binary_message(&mut self, data: &[u8]) -> ws::Result<()> {
        let mut decoder = NodeDecoder::new(data);
        
        match decoder.read_node() {
            Ok(node) => {
                // Proses node
                match node.tag.as_str() {
                    "message" => {
                        // Tangani pesan
                        if let Some(crate::node_protocol_new::NodeContent::Binary(content)) = node.content {
                            if let Ok(web_message) = serde_json::from_slice::<messages_extended::WebMessageInfo>(&content) {
                                self.event_handler.handle_event(Event::MessageReceived(true, Box::new(web_message)));
                            }
                        }
                    }
                    "presence" => {
                        // Tangani perubahan kehadiran
                        if let Some(from) = node.attrs.get("from") {
                            if let Ok(jid) = Jid::from_str(from) {
                                let presence_type = node.attrs.get("type").cloned().unwrap_or_default();
                                let status = match presence_type.as_str() {
                                    "available" => PresenceStatus::Available,
                                    "unavailable" => PresenceStatus::Unavailable,
                                    "composing" => PresenceStatus::Typing,
                                    "recording" => PresenceStatus::Recording,
                                    _ => PresenceStatus::Unavailable,
                                };
                                
                                self.event_handler.handle_event(Event::UserData(
                                    UserData::PresenceChange(jid, status, None)
                                ));
                            }
                        }
                    }
                    _ => {
                        // Tangani node lainnya
                    }
                }
            }
            Err(e) => {
                eprintln!("Error decoding node: {}", e);
            }
        }
        
        Ok(())
    }
}

// Clone implementation untuk WhatsAppClient
impl Clone for WhatsAppClient {
    fn clone(&self) -> Self {
        WhatsAppClient {
            sender: self.sender.clone(),
            session: self.session.clone(),
            multi_device_session: self.multi_device_session.clone(),
            state: self.state.clone(),
            event_handler: Arc::clone(&self.event_handler),
            client_id: self.client_id.clone(),
            websocket_url: self.websocket_url.clone(),
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

// Fungsi bantuan untuk operasi umum
pub fn format_phone_number(phone: &str) -> String {
    phone.replace(|c: char| !c.is_digit(10), "")
}

pub fn is_valid_jid(jid: &str) -> bool {
    jid.contains('@')
}