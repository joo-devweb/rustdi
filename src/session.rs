use crate::errors::*;
use ring::rand::SecureRandom;
use std::collections::HashMap;

/// Session data untuk koneksi WhatsApp
#[derive(Debug, Clone)]
pub struct Session {
    pub client_id: String,
    pub client_token: String,
    pub server_token: String,
    pub wid: String,
    pub enc_key: Vec<u8>,
    pub mac_key: Vec<u8>,
    pub platform: String,
    pub push_name: String,
    pub phone_info: Option<PhoneInfo>,
    pub is_logged_in: bool,
    pub registration_id: u32,
    pub identity_key_pair: KeyPair,
    pub signed_pre_key: SignedPreKey,
    pub one_time_keys: HashMap<u32, Key>,
    pub next_pre_key_id: u32,
}

#[derive(Debug, Clone)]
pub struct PhoneInfo {
    pub wa_version: String,
    pub mcc: String,
    pub mnc: String,
    pub os_version: String,
    pub device_manufacturer: String,
    pub device_model: String,
    pub os_build_number: String,
}

#[derive(Debug, Clone)]
pub struct KeyPair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct SignedPreKey {
    pub key_id: u32,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct Key {
    pub key_id: u32,
    pub public_key: Vec<u8>,
}

impl Session {
    /// Membuat session baru
    pub fn new() -> Self {
        let mut client_id_bytes = [0u8; 16];
        ring::rand::SystemRandom::new().fill(&mut client_id_bytes).unwrap();
        
        Self {
            client_id: base64::encode(&client_id_bytes),
            client_token: String::new(),
            server_token: String::new(),
            wid: String::new(),
            enc_key: vec![0; 32],
            mac_key: vec![0; 32],
            platform: "chrome".to_string(),
            push_name: String::new(),
            phone_info: None,
            is_logged_in: false,
            registration_id: generate_registration_id(),
            identity_key_pair: generate_identity_key_pair(),
            signed_pre_key: generate_signed_pre_key(),
            one_time_keys: HashMap::new(),
            next_pre_key_id: 1,
        }
    }

    /// Perbaharui kunci enkripsi
    pub fn update_encryption_keys(&mut self, enc_key: Vec<u8>, mac_key: Vec<u8>) {
        self.enc_key = enc_key;
        self.mac_key = mac_key;
    }

    /// Set token otentikasi
    pub fn set_auth_tokens(&mut self, client_token: String, server_token: String) {
        self.client_token = client_token;
        self.server_token = server_token;
    }

    /// Set identitas pengguna
    pub fn set_user_identity(&mut self, wid: String, push_name: String) {
        self.wid = wid;
        self.push_name = push_name;
    }

    /// Cek apakah session valid
    pub fn is_valid(&self) -> bool {
        !self.client_token.is_empty() && 
        !self.server_token.is_empty() &&
        self.enc_key.len() == 32 &&
        self.mac_key.len() == 32
    }
}

/// Fungsi bantu untuk menghasilkan ID registrasi acak
fn generate_registration_id() -> u32 {
    let mut id_bytes = [0u8; 2];
    ring::rand::SystemRandom::new().fill(&mut id_bytes).unwrap();
    u16::from_le_bytes([id_bytes[0], id_bytes[1]]) as u32
}

/// Fungsi bantu untuk menghasilkan pasangan kunci identitas
fn generate_identity_key_pair() -> KeyPair {
    // Dalam implementasi sebenarnya akan menggunakan fungsi kriptografi yang tepat
    let mut public_key = [0u8; 32];
    let mut private_key = [0u8; 32];
    ring::rand::SystemRandom::new().fill(&mut public_key).unwrap();
    ring::rand::SystemRandom::new().fill(&mut private_key).unwrap();
    
    KeyPair {
        public_key: public_key.to_vec(),
        private_key: private_key.to_vec(),
    }
}

/// Fungsi bantu untuk menghasilkan signed pre-key
fn generate_signed_pre_key() -> SignedPreKey {
    let mut public_key = [0u8; 32];
    let mut signature = [0u8; 64];
    ring::rand::SystemRandom::new().fill(&mut public_key).unwrap();
    ring::rand::SystemRandom::new().fill(&mut signature).unwrap();
    
    SignedPreKey {
        key_id: 1, // Default key ID
        public_key: public_key.to_vec(),
        signature: signature.to_vec(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    }
}

    /// Fungsi untuk menambahkan one-time key baru
    pub fn add_one_time_key(&mut self) -> Result<u32> {
        let key_id = self.next_pre_key_id;
        self.next_pre_key_id += 1;
        
        let mut public_key = [0u8; 32];
        ring::rand::SystemRandom::new().fill(&mut public_key).unwrap();
        
        self.one_time_keys.insert(key_id, Key {
            key_id,
            public_key: public_key.to_vec(),
        });
        
        Ok(key_id)
    }

    /// Hapus one-time key yang sudah digunakan
    pub fn remove_used_key(&mut self, key_id: u32) {
        self.one_time_keys.remove(&key_id);
    }

    /// Dapatkan one-time key yang tersedia
    pub fn get_available_one_time_keys(&self) -> Vec<&Key> {
        self.one_time_keys.values().collect()
    }
    
    /// Set token otentikasi
    pub fn set_auth_tokens(&mut self, client_token: String, server_token: String) {
        self.client_token = client_token;
        self.server_token = server_token;
    }

    /// Update kunci enkripsi
    pub fn update_encryption_keys(&mut self, enc_key: Vec<u8>, mac_key: Vec<u8>) {
        self.enc_key = enc_key;
        self.mac_key = mac_key;
        self.is_logged_in = true;
    }
}