//! Modul untuk menghubungkan struktur pesan lama dan baru
//! 
//! File ini berisi fungsi untuk mengkonversi antara struktur pesan yang digunakan dalam
//! sistem saat ini dan struktur pesan yang lebih lengkap sesuai dengan protokol WhatsApp yang baru

use crate::message::{ChatMessageContent as OldChatMessageContent, ChatMessage as OldChatMessage, MessageId, Direction, Peer};
use crate::messages_extended::{Message as NewMessage, ImageMessage as NewImageMessage, AudioMessage as NewAudioMessage, DocumentMessage as NewDocumentMessage, ContactMessage as NewContactMessage, LocationMessage as NewLocationMessage, ExtendedTextMessage as NewExtendedTextMessage};
use crate::Jid;
use crate::errors::*;
use chrono::NaiveDateTime;
use std::time::Duration;

impl From<OldChatMessageContent> for NewMessage {
    fn from(old_content: OldChatMessageContent) -> NewMessage {
        match old_content {
            OldChatMessageContent::Text(text) => NewMessage {
                conversation: Some(text),
                ..Default::default()
            },
            OldChatMessageContent::Image(info, size, thumbnail) => NewMessage {
                image_message: Some(NewImageMessage {
                    url: info.url,
                    mimetype: Some(info.mime),
                    caption: None,
                    file_sha256: info.sha256,
                    file_length: info.size as u64,
                    height: size.0,
                    width: size.1,
                    media_key: info.key,
                    file_enc_sha256: info.enc_sha256,
                    direct_path: String::new(), // akan diisi saat upload
                    media_key_timestamp: 0, // akan diisi saat upload
                    jpeg_thumbnail: Some(thumbnail),
                    context_info: None,
                    streaming_sidecar: None,
                    view_once: None,
                }),
                ..Default::default()
            },
            OldChatMessageContent::Audio(info, duration) => NewMessage {
                audio_message: Some(NewAudioMessage {
                    url: info.url,
                    mimetype: info.mime,
                    file_sha256: info.sha256,
                    file_length: info.size as u64,
                    seconds: duration.as_secs() as u32,
                    ptt: false, // bisa diatur berdasarkan konteks
                    media_key: info.key,
                    file_enc_sha256: info.enc_sha256,
                    direct_path: String::new(), // akan diisi saat upload
                    media_key_timestamp: 0, // akan diisi saat upload
                    context_info: None,
                    streaming_sidecar: None,
                }),
                ..Default::default()
            },
            OldChatMessageContent::Document(info, filename) => NewMessage {
                document_message: Some(NewDocumentMessage {
                    url: info.url,
                    mimetype: info.mime,
                    title: filename, // gunakan filename sebagai title
                    file_sha256: info.sha256,
                    file_length: info.size as u64,
                    page_count: None,
                    media_key: info.key,
                    file_name: filename,
                    file_enc_sha256: info.enc_sha256,
                    direct_path: String::new(), // akan diisi saat upload
                    media_key_timestamp: 0, // akan diisi saat upload
                    jpeg_thumbnail: None,
                    context_info: None,
                    thumbnail_direct_path: None,
                    thumbnail_sha256: None,
                    thumbnail_enc_sha256: None,
                }),
                ..Default::default()
            },
        }
    }
}

impl From<NewMessage> for OldChatMessageContent {
    fn from(new_message: NewMessage) -> OldChatMessageContent {
        if let Some(conversation) = new_message.conversation {
            OldChatMessageContent::Text(conversation)
        } else if let Some(image_msg) = new_message.image_message {
            OldChatMessageContent::Image(
                crate::message::FileInfo {
                    url: image_msg.url,
                    mime: image_msg.mimetype.unwrap_or_default(),
                    sha256: image_msg.file_sha256,
                    enc_sha256: image_msg.file_enc_sha256,
                    size: image_msg.file_length as usize,
                    key: image_msg.media_key,
                },
                (image_msg.height, image_msg.width),
                image_msg.jpeg_thumbnail.unwrap_or_default()
            )
        } else if let Some(audio_msg) = new_message.audio_message {
            OldChatMessageContent::Audio(
                crate::message::FileInfo {
                    url: audio_msg.url,
                    mime: audio_msg.mimetype,
                    sha256: audio_msg.file_sha256,
                    enc_sha256: audio_msg.file_enc_sha256,
                    size: audio_msg.file_length as usize,
                    key: audio_msg.media_key,
                },
                Duration::new(audio_msg.seconds as u64, 0)
            )
        } else if let Some(document_msg) = new_message.document_message {
            OldChatMessageContent::Document(
                crate::message::FileInfo {
                    url: document_msg.url,
                    mime: document_msg.mimetype,
                    sha256: document_msg.file_sha256,
                    enc_sha256: document_msg.file_enc_sha256,
                    size: document_msg.file_length as usize,
                    key: document_msg.media_key,
                },
                document_msg.file_name
            )
        } else {
            // Default ke pesan teks kosong jika tidak ada yang dikenali
            OldChatMessageContent::Text("Unsupported message type".to_string())
        }
    }
}

// Implementasi default untuk NewMessage
impl Default for NewMessage {
    fn default() -> Self {
        NewMessage {
            conversation: None,
            image_message: None,
            contact_message: None,
            location_message: None,
            extended_text_message: None,
            document_message: None,
            audio_message: None,
            video_message: None,
            contacts_array_message: None,
            template_message: None,
            group_invite_message: None,
            product_message: None,
            list_message: None,
            list_response_message: None,
            buttons_message: None,
            buttons_response_message: None,
            sticker_message: None,
            live_location_message: None,
            protocol_message: None,
            template_button_reply_message: None,
            device_sent_message: None,
        }
    }
}

// Konversi untuk ChatMessage
impl From<OldChatMessage> for crate::messages_extended::WebMessageInfo {
    fn from(old_msg: OldChatMessage) -> crate::messages_extended::WebMessageInfo {
        crate::messages_extended::WebMessageInfo {
            key: crate::messages_extended::MessageKey {
                remote_jid: match old_msg.direction {
                    Direction::Sending(jid) => jid.to_string(),
                    Direction::Receiving(peer) => match peer {
                        Peer::Individual(jid) => jid.to_string(),
                        Peer::Group { group, .. } => group.to_string(),
                    }
                },
                from_me: matches!(old_msg.direction, Direction::Sending(_)),
                id: old_msg.id.0,
                participant: match old_msg.direction {
                    Direction::Sending(_) => None,
                    Direction::Receiving(Peer::Group { participant, .. }) => Some(participant.to_string()),
                    Direction::Receiving(Peer::Individual(_)) => None,
                },
            },
            message: Some(Box::new(old_msg.content.into())),
            message_timestamp: Some(old_msg.time.timestamp() as u64),
            status: Some(2), // SERVER_ACK
            participant: match old_msg.direction {
                Direction::Sending(_) => None,
                Direction::Receiving(Peer::Group { participant, .. }) => Some(participant.to_string()),
                Direction::Receiving(Peer::Individual(_)) => None,
            },
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
        }
    }
}

impl From<crate::messages_extended::WebMessageInfo> for Result<OldChatMessage> {
    fn from(new_msg_info: crate::messages_extended::WebMessageInfo) -> Result<OldChatMessage> {
        let key = new_msg_info.key;
        
        let jid = Jid::from_str(&key.remote_jid.replace("@c.us", "@s.whatsapp.net").replace("@g.us", "@s.whatsapp.net"))
            .chain_err(|| "Invalid JID in message key")?;
        
        let direction = if key.from_me {
            Direction::Sending(jid)
        } else {
            if let Some(participant_str) = key.participant {
                let participant = Jid::from_str(&participant_str)
                    .chain_err(|| "Invalid participant JID")?;
                Direction::Receiving(Peer::Group { group: jid, participant })
            } else {
                Direction::Receiving(Peer::Individual(jid))
            }
        };

        let content = if let Some(message) = new_msg_info.message {
            (*message).into()
        } else {
            OldChatMessageContent::Text(String::new())
        };

        Ok(OldChatMessage {
            direction,
            time: NaiveDateTime::from_timestamp_opt(
                new_msg_info.message_timestamp.unwrap_or(0) as i64, 0
            ).ok_or_else(|| "Invalid timestamp")?,
            id: MessageId(key.id),
            content,
        })
    }
}