use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use ring::rand::{SystemRandom, SecureRandom};

// Sesi koneksi WhatsApp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub client_id: String,
    pub server_token: String,
    pub client_token: String,
    pub enc_key: Vec<u8>,
    pub mac_key: Vec<u8>,
    pub wid: String,
    pub push_name: String,
    pub phone: Option<PhoneInfo>,
}

// Informasi perangkat pengguna
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneInfo {
    pubwa_version: String,
    pub mcc: String,
    pub mnc: String,
    pub os_version: String,
    pub device_manufacturer: String,
    pub device_model: String,
    pub os_build_number: String,
}

// Sesi multi-perangkat untuk WhatsApp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiDeviceSession {
    pub device_id: String,
    pub registration_id: u32,
    pub identity_id: Vec<u8>,
    pub identity_key_pair: KeyPair,
    pub signed_pre_key: SignedPreKey,
    pub one_time_keys: HashMap<u32, Key>,
    pub signed_device_keys: HashMap<String, Vec<u8>>,
    pub next_pre_key_id: u32,
    pub next_session_id: u32,
}

// Pasangan kunci kriptografi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

// Kunci pre-signed untuk pertukanan kunci
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedPreKey {
    pub key_id: u32,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
    pub timestamp: u64,
}

// Kunci satu kali pakai
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Key {
    pub key_id: u32,
    pub public_key: Vec<u8>,
}

// Informasi perangkat lain dalam sesi multi-perangkat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: u32,
    pub jid: String,
    pub registration_id: u32,
    pub noise_key: KeyPair,
    pub identity_key: KeyPair,
    pub signed_pre_key: SignedPreKey,
    pub last_seen: Option<u64>,
}

impl Default for MultiDeviceSession {
    fn default() -> Self {
        let mut registration_id = [0u8; 2];
        SystemRandom::new().fill(&mut registration_id).unwrap();
        let reg_id = u32::from_le_bytes(registration_id);

        let (identity_private, identity_public) = crate::crypto::generate_keypair().unwrap();

        MultiDeviceSession {
            device_id: uuid::Uuid::new_v4().to_string(),
            registration_id: reg_id,
            identity_id: vec![0u8; 32],
            identity_key_pair: KeyPair {
                public_key: identity_public,
                private_key: vec![], // Didapatkan dari generate_keypair
            },
            signed_pre_key: SignedPreKey {
                key_id: 1,
                public_key: vec![],
                signature: vec![],
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
            one_time_keys: HashMap::new(),
            signed_device_keys: HashMap::new(),
            next_pre_key_id: 1,
            next_session_id: 1,
        }
    }
}

impl Session {
    pub fn new() -> Self {
        let mut client_id = [0u8; 16];
        SystemRandom::new().fill(&mut client_id).unwrap();
        
        Session {
            client_id: base64::encode(&client_id),
            server_token: String::new(),
            client_token: String::new(),
            enc_key: vec![0; 32],
            mac_key: vec![0; 32],
            wid: String::new(),
            push_name: String::new(),
            phone: None,
        }
    }
}