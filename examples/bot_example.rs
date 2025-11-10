use rustdi::{WhatsAppClient, State, Event};

fn main() {
    println!("🚀 Memulai WhatsApp Bot dengan Rustdi");

    // Buat client baru
    let client = rustdi::create_client().expect("Gagal membuat client");
    
    println!("📱 Client berhasil dibuat");
    
    // Hubungkan ke WhatsApp dengan QR code
    println!("⏳ Menginisialisasi koneksi...");
    
    // Kita akan gunakan callback untuk menangani QR code
    let qr_callback = |qr_code: &qrcode::QrCode| {
        println!("📱 Silakan scan QR code berikut:");
        
        // Cetak QR code ke konsol
        let qr_string = qr_code
            .render::<qrcode::render::unicode::Dense1x2>()
            .dark_color(qrcode::render::unicode::Dense1x2::Light)
            .light_color(qrcode::render::unicode::Dense1x2::Dark)
            .build();
        
        println!("{}", qr_string);
        println!("Kode QR juga bisa diakses melalui event");
    };
    
    // Hubungkan ke WhatsApp
    if let Err(e) = rustdi::connect_with_qr(&client, qr_callback) {
        eprintln!("❌ Gagal menghubungkan: {}", e);
        return;
    }
    
    println!("✅ Koneksi dimulai, tunggu...");
    
    // Loop untuk membaca event
    loop {
        if let Some(event) = client.poll_event() {
            match event {
                Event::StateChanged(state) => {
                    println!("🔄 Status berubah: {:?}", state);
                    
                    if let State::Connected = state {
                        println!("✅ WhatsApp berhasil terhubung!");
                        
                        // Kirim pesan percobaan
                        match client.send_message("+6281234567890@s.whatsapp.net", "Halo! Ini adalah pesan dari bot Rustdi") {
                            Ok(msg_id) => println!("💬 Pesan terkirim dengan ID: {}", msg_id),
                            Err(e) => eprintln!("❌ Gagal kirim pesan: {}", e),
                        }
                        
                        // Set status kehadiran
                        if let Err(e) = client.set_presence(rustdi::PresenceStatus::Available) {
                            eprintln!("❌ Gagal set kehadiran: {}", e);
                        }
                    }
                },
                Event::MessageReceived(msg) => {
                    println!("📥 Pesan baru diterima!");
                    if let Some(ref key) = msg.key {
                        println!("  Dari: {}", key.remote_jid);
                        if let Some(ref message_content) = msg.message {
                            if let Some(ref text) = message_content.conversation {
                                println!("  Isi: {}", text);
                                
                                // Balas pesan
                                let jid = key.remote_jid.clone();
                                let reply = format!("Balas: {}", text);
                                if let Err(e) = client.send_message(&jid, &reply) {
                                    eprintln!("❌ Gagal balas pesan: {}", e);
                                }
                            }
                        }
                    }
                },
                Event::QrCodeGenerated(qr_data) => {
                    println!("📱 QR Code telah dibuat: {}", qr_data);
                    println!("Silakan scan QR code yang ditampilkan di atas");
                },
                Event::Error(error) => {
                    eprintln!("❌ Error: {}", error);
                },
                _ => {
                    println!("📦 Event diterima: {:?}", event);
                }
            }
        }
        
        // Sleep sebentar untuk mengurangi penggunaan CPU
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    // langsung disconnect setelah beberapa detik
    // client.disconnect().unwrap();
    // println!("🔌 Terputus dari WhatsApp");
}