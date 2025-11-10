//! Fungsi utilitas untuk membuat pesan-pesan WhatsApp sesuai dengan protokol terbaru
//! 
//! File ini berisi fungsi-fungsi untuk membuat berbagai tipe pesan sesuai dengan
//! struktur yang ditemukan dalam analisis Baileys dan whatsmeow terbaru

use crate::messages_extended::*;
use std::collections::HashMap;

impl NewMessage {
    /// Membuat pesan teks biasa
    pub fn text_message(text: String) -> Self {
        NewMessage {
            conversation: Some(text),
            ..Default::default()
        }
    }

    /// Membuat pesan gambar
    pub fn image_message(
        url: String,
        caption: Option<String>,
        file_sha256: Vec<u8>,
        file_length: u64,
        dimensions: (u32, u32),
        media_key: Vec<u8>,
        file_enc_sha256: Vec<u8>,
        direct_path: String,
        media_key_timestamp: i64,
        jpeg_thumbnail: Option<Vec<u8>>
    ) -> Self {
        NewMessage {
            image_message: Some(ImageMessage {
                url,
                mimetype: Some("image/jpeg".to_string()), // default, bisa diubah
                caption,
                file_sha256,
                file_length,
                height: dimensions.0,
                width: dimensions.1,
                media_key,
                file_enc_sha256,
                direct_path,
                media_key_timestamp,
                jpeg_thumbnail,
                context_info: None,
                streaming_sidecar: None,
                view_once: None,
            }),
            ..Default::default()
        }
    }

    /// Membuat pesan dokumen
    pub fn document_message(
        url: String,
        title: String,
        file_name: String,
        mime_type: String,
        file_sha256: Vec<u8>,
        file_length: u64,
        media_key: Vec<u8>,
        file_enc_sha256: Vec<u8>,
        direct_path: String,
        media_key_timestamp: i64
    ) -> Self {
        NewMessage {
            document_message: Some(DocumentMessage {
                url,
                mimetype: mime_type,
                title,
                file_sha256,
                file_length,
                page_count: None,
                media_key,
                file_name,
                file_enc_sha256,
                direct_path,
                media_key_timestamp,
                jpeg_thumbnail: None,
                context_info: None,
                thumbnail_direct_path: None,
                thumbnail_sha256: None,
                thumbnail_enc_sha256: None,
            }),
            ..Default::default()
        }
    }

    /// Membuat pesan kontak
    pub fn contact_message(display_name: String, vcard: String) -> Self {
        NewMessage {
            contact_message: Some(ContactMessage {
                display_name,
                vcard: Some(vcard),
                context_info: None,
            }),
            ..Default::default()
        }
    }

    /// Membuat pesan lokasi
    pub fn location_message(
        degrees_latitude: f64,
        degrees_longitude: f64,
        name: Option<String>,
        jpeg_thumbnail: Option<Vec<u8>>
    ) -> Self {
        NewMessage {
            location_message: Some(LocationMessage {
                degrees_latitude,
                degrees_longitude,
                name,
                address: None,
                url: None,
                is_live: None,
                accuracy_in_meters: None,
                speed_in_mps: None,
                degrees_clockwise_from_magnetic_north: None,
                comment: None,
                jpeg_thumbnail,
                context_info: None,
            }),
            ..Default::default()
        }
    }

    /// Membuat pesan teks terperluas (dengan thumbnail, URL, dll)
    pub fn extended_text_message(
        text: String,
        matched_text: Option<String>,
        canonical_url: Option<String>,
        description: Option<String>,
        title: Option<String>,
        jpeg_thumbnail: Option<Vec<u8>>
    ) -> Self {
        NewMessage {
            extended_text_message: Some(ExtendedTextMessage {
                text,
                matched_text,
                canonical_url,
                description,
                title,
                text_argb: None,
                background_argb: None,
                font: None,
                preview_type: None,
                jpeg_thumbnail,
                context_info: None,
                do_not_play_inline: None,
            }),
            ..Default::default()
        }
    }

    /// Membuat pesan template (untuk pesan interaktif)
    pub fn template_message_with_buttons(
        hydrated_content_text: String,
        hydrated_footer_text: Option<String>,
        buttons: Vec<HydratedTemplateButton>
    ) -> Self {
        NewMessage {
            template_message: Some(TemplateMessage {
                context_info: None,
                hydrated_template: Some(HydratedFourRowTemplate {
                    hydrated_content_text: Some(hydrated_content_text),
                    hydrated_footer_text,
                    hydrated_buttons: buttons,
                    template_id: None,
                    title_document_message: None,
                    hydrated_title_text: None,
                    title_image_message: None,
                    title_video_message: None,
                    title_location_message: None,
                }),
                four_row_template: None,
                hydrated_four_row_template: None,
            }),
            ..Default::default()
        }
    }

    /// Membuat pesan invite ke grup
    pub fn group_invite_message(
        group_jid: String,
        invite_code: String,
        invite_expiration: i64,
        group_name: String,
        jpeg_thumbnail: Option<Vec<u8>>
    ) -> Self {
        NewMessage {
            group_invite_message: Some(GroupInviteMessage {
                group_jid,
                invite_code,
                invite_expiration,
                group_name,
                jpeg_thumbnail,
                caption: None,
                context_info: None,
            }),
            ..Default::default()
        }
    }
}

impl WebMessageInfo {
    /// Membuat WebMessageInfo baru dari pesan dan key
    pub fn new(message: NewMessage, key: MessageKey) -> Self {
        WebMessageInfo {
            key,
            message: Some(Box::new(message)),
            message_timestamp: Some(chrono::Utc::now().timestamp() as u64),
            status: Some(2), // SERVER_ACK
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
        }
    }
}

// Tambahkan fungsi untuk membuat ContextInfo
impl ContextInfo {
    pub fn new_with_quoted_message(quoted_message: NewMessage, stanza_id: Option<String>) -> Self {
        ContextInfo {
            stanza_id,
            participant: None,
            quoted_message: Some(Box::new(quoted_message)),
            remote_jid: None,
            mentioned_jid: vec![],
            conversion_source: None,
            conversion_data: None,
            conversion_delay_seconds: None,
            forwarding_score: None,
            is_forwarded: None,
            ephemeral_start_timestamp: None,
            ephemeral_duration: None,
        }
    }
    
    pub fn new_with_mentions(mentioned_jids: Vec<String>) -> Self {
        ContextInfo {
            stanza_id: None,
            participant: None,
            quoted_message: None,
            remote_jid: None,
            mentioned_jid: mentioned_jids,
            conversion_source: None,
            conversion_data: None,
            conversion_delay_seconds: None,
            forwarding_score: None,
            is_forwarded: None,
            ephemeral_start_timestamp: None,
            ephemeral_duration: None,
        }
    }
}