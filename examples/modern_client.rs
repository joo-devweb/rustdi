use rustdi::{WhatsAppClientBuilder, EventHandler, Event, State, UserData};
use rustdi::{Jid, PresenceStatus};

// Handler untuk menangani event dari WhatsApp
struct MyEventHandler;

impl EventHandler for MyEventHandler {
    fn handle_event(&self, event: Event) {
        match event {
            Event::StateChange(state) => {
                println!("State changed: {:?}", state);
            }
            Event::UserData(user_data) => {
                println!("User data received: {:?}", user_data);
            }
            Event::MessageReceived(is_new, message) => {
                println!("Message received (new: {}): {:?}", is_new, message);
            }
            Event::Disconnected(reason) => {
                println!("Disconnected: {:?}", reason);
            }
        }
    }
}

fn main() {
    println!("Contoh Client WhatsApp");
    
    // Buat event handler
    let event_handler = Box::new(MyEventHandler);
    
    // Buat client
    let client = WhatsAppClientBuilder::new()
        .with_event_handler(event_handler)
        .build()
        .expect("Gagal membuat client");
    
    // Contoh: Konek dengan QR code
    // client.connect_with_qr(Box::new(|qr_data| {
    //     println!("Data QR Code: {}", qr_data);
    //     // Tampilkan QR code ke pengguna
    // })).expect("Gagal terhubung");
    
    // ATAU konek dengan pairing code
    match client.connect_with_pairing_code("+1234567890", false) {
        Ok(pairing_code) => {
            println!("Masukkan kode ini di ponsel Anda: {}", pairing_code);
        }
        Err(e) => {
            eprintln!("Gagal memulai pairing: {}", e);
        }
    }
    
    // Contoh: Atur kehadiran
    if let Err(e) = client.set_presence(PresenceStatus::Available) {
        eprintln!("Gagal mengatur kehadiran: {}", e);
    }
    
    // Biarkan program berjalan
    std::thread::sleep(std::time::Duration::from_secs(30));
    
    // Putuskan koneksi
    if let Err(e) = client.disconnect() {
        eprintln!("Gagal memutus koneksi: {}", e);
    }
    
    println!("Contoh selesai");
}