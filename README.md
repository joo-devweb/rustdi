# Rustdi - Modern WhatsApp Client Library for Rust

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/rustdi.svg)](https://crates.io/crates/rustdi)
[![Documentation](https://docs.rs/rustdi/badge.svg)](https://docs.rs/rustdi)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/crates/d/rustdi.svg)](https://crates.io/crates/rustdi)

Modern, efficient, and secure WhatsApp client library for Rust.

</div>

## Overview

Rustdi adalah pustaka Rust modern untuk berinteraksi dengan protokol WhatsApp Web. Dirancang untuk menyediakan antarmuka yang aman, efisien, dan mudah digunakan untuk mengembangkan aplikasi WhatsApp seperti bot, gateway, atau layanan otomasi.

**Masih Dalam Development,jika banyak error ya maklumin,jika ketemu error buka issue yak**

## Features

- **Modern Protocol**: Menggunakan protokol Noise X25519-XSalsa20-Poly1305 (WhatsApp V2)
- **Dual Authentication**: Mendukung QR Code dan Pairing Code untuk otentikasi
- **Multi-Device Support**: Arsitektur multi-device yang lengkap
- **End-to-End Encryption**: Implementasi enkripsi tingkat perangkat
- **Media Handling**: Pengiriman dan penerimaan pesan media
- **Group Management**: Manajemen grup dan pesan broadcast
- **High Security**: Implementasi enkripsi dan verifikasi yang ketat
- **Async Support**: Dukungan untuk operasi asynchronous
- **Event-Driven**: Sistem event yang fleksibel
- **Type-Safe**: Desain tipe yang aman untuk mencegah bug saat runtime

## Installation

Tambahkan ini ke `Cargo.toml` Anda:

```toml
[dependencies]
rustdi = "0.10.0"
```

## Quick Start

Contoh sederhana menggunakan library:

```rust
use rustdi::{WhatsAppClient, EventHandler, Event, Jid, PresenceStatus};

// Definisi event handler Anda
struct MyEventHandler;

impl EventHandler for MyEventHandler {
    fn handle_event(&self, event: Event) {
        match event {
            Event::Connected => {
                println!("✅ Berhasil terhubung ke WhatsApp!");
            }
            Event::MessageReceived(msg_info) => {
                println!("📦 Pesan baru diterima dari: {}", msg_info.key.remote_jid);
                // Proses pesan di sini
            }
            Event::QrCodeGenerated(qr_data) => {
                println!("📱 Scan QR Code ini: {}", qr_data);
                // Tampilkan QR Code ke pengguna
            }
            Event::Error(error) => {
                eprintln!("❌ Error: {}", error);
            }
            _ => {
                // Tangani event lainnya
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Buat event handler
    let handler = Box::new(MyEventHandler {});
    
    // Buat client WhatsApp
    let client = rustdi::WhatsAppClient::new(handler)?;
    
    // Hubungkan menggunakan QR Code
    client.connect(rustdi::AuthMethod::QRCode {
        callback: Box::new(|qr_code| {
            // Tampilkan QR code ke pengguna
            println!("Silakan scan QR code yang tampil");
        })
    })?;
    
    // Kirim pesan contoh
    let jid = Jid::from_string("6281234567890@s.whatsapp.net")?;
    let message_id = client.send_text_message(&jid, "Halo dari Rustdi!")?;
    println!("💬 Pesan terkirim dengan ID: {}", message_id);
    
    // Atur status kehadiran
    client.set_presence(PresenceStatus::Available)?;
    
    // Loop utama aplikasi
    loop {
        if let Some(event) = client.poll_event() {
            // EventHandler akan menangani event secara otomatis
        }
        
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    Ok(())
}
```

## Advanced Usage

### Mengirim Media

```rust
use rustdi::MediaType;

// Kirim gambar
let message_id = client.send_media_message(
    &recipient_jid, 
    MediaType::Image, 
    "https://example.com/image.jpg",
    Some("Deskripsi gambar")
)?;
```

### Mengelola Grup

```rust
// Buat grup
let participants = vec![
    Jid::from_string("6281234567891@s.whatsapp.net")?,
    Jid::from_string("6281234567892@s.whatsapp.net")?
];

// Fungsi untuk membuat grup tidak disediakan dalam contoh ini
// karena memerlukan implementasi lebih lanjut
```

### Otentikasi dengan Pairing Code

```rust
client.connect(rustdi::AuthMethod::PairingCode {
    phone_number: "+6281234567890".to_string(),
    callback: Box::new(|pairing_code| {
        println!("Masukkan kode ini di perangkat Anda: {}", pairing_code);
    })
})?;
```

## Architecture

Rustdi dilengkapi dengan komponen utama berikut:

- **Client**: Titik masuk utama untuk koneksi WhatsApp
- **Session**: Manajemen sesi dan kunci enkripsi
- **Crypto**: Implementasi algoritma enkripsi WhatsApp
- **Protocol**: Pengkodean dan dekoding protokol node
- **Messages**: Struktur data pesan lengkap
- **Handshake**: Implementasi protokol Noise untuk otentikasi

## Security

- Menggunakan enkripsi Noise X25519-XSalsa20-Poly1305 sesuai standar WhatsApp
- Kunci enkripsi disimpan secara aman dan hanya dapat diakses oleh session yang sah
- Semua pesan terenkripsi end-to-end
- Verifikasi sertifikat server dilakukan secara ketat

## Performance

- Menggunakan implementasi efisien dari algoritma kriptografi
- Arsitektur event-driven untuk penanganan pesan yang cepat
- Manajemen memori yang optimal
- Dukungan untuk multi-threading

## Contributing

Kontribusi sangat dihargai! Silakan fork proyek ini dan kirim pull request untuk:

- Bug fixes
- Feature enhancements
- Dokumentasi yang lebih baik
- Unit tests yang lebih lengkap

## License

Licensed under the MIT license.