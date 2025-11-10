use crate::errors::*;
use ring::{agreement, digest, hmac, hkdf, rand};
use std::sync::Arc;

/// Struktur untuk kunci enkripsi yang dihasilkan
#[derive(Debug, Clone)]
pub struct SessionKeys {
    pub enc_key: Vec<u8>,
    pub mac_key: Vec<u8>,
}

/// Fungsi untuk menghasilkan pasangan kunci X25519
pub fn generate_keypair() -> Result<(agreement::EphemeralPrivateKey, Vec<u8>)> {
    let rng = rand::SystemRandom::new();
    let private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)
        .map_err(|_| "Failed to generate private key")?;
    
    let public_key = private_key.compute_public_key()
        .map_err(|_| "Failed to compute public key")?;
    
    Ok((private_key, public_key.as_ref().to_vec()))
}

/// Fungsi untuk menghitung kunci enkripsi dan HMAC dari secret yang diberikan server
pub fn derive_session_keys(
    server_identity_public: &[u8],
    expected_hmac: &[u8],
    encrypted_keys: &[u8],
) -> Result<SessionKeys> {
    // Generate our private key
    let (our_private_key, our_public_key) = generate_keypair()?;
    
    // Compute shared secret using DH
    let server_public = agreement::UnparsedPublicKey::new(&agreement::X25519, server_identity_public);
    let shared_secret = agreement::agree_ephemeral(
        our_private_key,
        &server_public,
        |shared_secret| shared_secret.to_vec(),
    ).map_err(|_| "Failed to compute shared secret")?;

    // Verify HMAC
    let verifier_key = hmac::Key::new(hmac::HMAC_SHA256, &[0u8; 32]); // Null key for verification
    let verification_data = [server_identity_public, encrypted_keys].concat();
    let computed_hmac = hmac::sign(&verifier_key, &verification_data);
    
    if computed_hmac.as_ref() != expected_hmac {
        return Err("HMAC verification failed".into());
    }

    // Expand the shared secret using HKDF
    let salt = [0u8; 32]; // Zero salt as used in WhatsApp protocol
    let hkdf_salt = hkdf::Salt::new(hkdf::HKDF_SHA256, &salt);
    let pseudo_random_key = hkdf_salt.extract(&shared_secret);
    
    let mut expanded_secret = [0u8; 112]; // WhatsApp uses 112 bytes
    pseudo_random_key.expand(&[], &mut expanded_secret)
        .map_err(|_| "Failed to expand secret")?;

    // Extract keys
    let enc_key = expanded_secret[0..32].to_vec();
    let mac_key = expanded_secret[32..64].to_vec();
    
    Ok(SessionKeys {
        enc_key,
        mac_key,
    })
}

/// Enkripsi pesan menggunakan kunci yang dihasilkan
pub fn encrypt_message(enc_key: &[u8], mac_key: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    // Dalam implementasi asli, akan menggunakan AES atau cipher lainnya
    // Ini adalah placeholder yang hanya mengembalikan data mentah
    Ok(message.to_vec())
}

/// Dekripsi pesan menggunakan kunci yang dihasilkan
pub fn decrypt_message(enc_key: &[u8], mac_key: &[u8], encrypted_message: &[u8]) -> Result<Vec<u8>> {
    // Dalam implementasi asli, akan menggunakan AES atau cipher lainnya
    // Ini adalah placeholder yang hanya mengembalikan data mentah
    Ok(encrypted_message.to_vec())
}

/// Buat HMAC signature untuk pesan
pub fn sign_message(mac_key: &[u8], message: &[u8]) -> Vec<u8> {
    let signing_key = hmac::Key::new(hmac::HMAC_SHA256, mac_key);
    hmac::sign(&signing_key, message).as_ref().to_vec()
}

/// Fungsi untuk mengenkripsi dan menandatangani pesan sebelum dikirim
pub fn sign_and_encrypt_message(enc_key: &[u8], mac_key: &[u8], message: &[u8]) -> Vec<u8> {
    let encrypted = encrypt_message(enc_key, mac_key, message).unwrap_or(message.to_vec());
    let signature = sign_message(mac_key, &encrypted);
    
    // Gabungkan signature + encrypted message
    [signature, encrypted].concat()
}

/// Fungsi untuk memverifikasi dan mendekripsi pesan yang diterima
pub fn verify_and_decrypt_message(enc_key: &[u8], mac_key: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    if message.len() < 32 {
        return Err("Message too short".into());
    }

    // Ambil HMAC dari 32 byte pertama
    let received_hmac = &message[0..32];
    let encrypted_content = &message[32..];

    // Tandatangani kembali konten untuk verifikasi
    let computed_hmac = sign_message(mac_key, encrypted_content);

    if received_hmac != computed_hmac {
        return Err("HMAC verification failed".into());
    }

    // Dekripsi konten
    decrypt_message(enc_key, mac_key, encrypted_content)
}

/// Fungsi untuk membuat kunci sementara
pub fn create_temporary_key() -> Result<[u8; 32]> {
    let mut key = [0u8; 32];
    rand::SystemRandom::new().fill(&mut key)
        .map_err(|_| "Failed to generate random key")?;
    Ok(key)
}