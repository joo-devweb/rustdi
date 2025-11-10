extern crate simple_logger;
#[macro_use]
extern crate log;
extern crate qrcode;
extern crate image;
extern crate bincode;
extern crate rustdi;
extern crate reqwest;
extern crate base64;

use std::fs::{File, OpenOptions, remove_file};
use std::io::Read;
use std::sync::Arc;

use image::Luma;

use rustdi::connection;
use rustdi::connection::{DisconnectReason, PersistentSession, RustdiHandler, RustdiConnection, UserData, State};
use rustdi::message::{ChatMessage, ChatMessageContent};
use rustdi::media;
use rustdi::{Jid, MediaType};


const SESSION_FILENAME: &str = "session.bin";

struct Handler {}

impl RustdiHandler for Handler {
    fn on_state_changed(&self, connection: &RustdiConnection<Handler>, state: State) {
        info!("new state: {:?}", state);
        if state == State::Connected {
            let mut file = Vec::new();
            File::open("path/to/image.jpg").unwrap().read_to_end(&mut file).unwrap();

            let connection0 = connection.clone();
            let (thumbnail, size) = media::generate_thumbnail_and_get_size(&file);
            let thumbnail = Arc::new(thumbnail);

            media::upload_file(&file, MediaType::Image, &connection, Box::new(move |file_info| {
                let jid = Jid::from_phone_number("+49123456789".to_string()).unwrap();

                connection0.send_message(ChatMessageContent::Image(file_info.unwrap(), size, thumbnail.to_vec()), jid);
            }));
        }
    }

    fn on_persistent_session_data_changed(&self, persistent_session: PersistentSession) {
        bincode::serialize_into(OpenOptions::new().create(true).write(true).open(SESSION_FILENAME).unwrap(), &persistent_session).unwrap();
    }
    fn on_user_data_changed(&self, _: &RustdiConnection<Handler>, _: UserData) {}
    fn on_disconnect(&self, reason: DisconnectReason) {
        info!("disconnected");

        match reason {
            DisconnectReason::Removed => {
                remove_file(SESSION_FILENAME).unwrap();
            }
            _ => {}
        }
    }
    fn on_message(&self, _: &RustdiConnection<Handler>, _: bool, _: Box<ChatMessage>) {}
}

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let handler = Handler {};

    if let Ok(file) = File::open(SESSION_FILENAME) {
        let (_, join_handle) = connection::with_persistent_session(bincode::deserialize_from(file).unwrap(), handler);
        join_handle.join().unwrap();
    } else {
        let (_, join_handle) = connection::new(|qr| { qr.render::<Luma<u8>>().module_dimensions(10, 10).build().save("login_qr.png").unwrap(); }, handler);
        join_handle.join().unwrap();
    }
}
