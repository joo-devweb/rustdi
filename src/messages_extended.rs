//! Definisi struktur data pesan WhatsApp berdasarkan protokol whatsmeow dan Baileys
//! 
//! File ini berisi definisi struktur data utama yang digunakan dalam protokol WhatsApp
//! Berdasarkan analisis dari library whatsmeow dan Baileys terbaru

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageKey {
    pub remote_jid: String,
    pub from_me: bool,
    pub id: String,
    pub participant: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextInfo {
    pub stanza_id: Option<String>,
    pub participant: Option<String>,
    pub quoted_message: Option<Box<Message>>,
    pub remote_jid: Option<String>,
    pub mentioned_jid: Vec<String>,
    pub conversion_source: Option<String>,
    pub conversion_data: Option<Vec<u8>>,
    pub conversion_delay_seconds: Option<u32>,
    pub forwarding_score: Option<u32>,
    pub is_forwarded: Option<bool>,
    pub ephemeral_start_timestamp: Option<u64>,
    pub ephemeral_duration: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMessage {
    pub url: String,
    pub mimetype: Option<String>,
    pub caption: Option<String>,
    pub file_sha256: Vec<u8>,
    pub file_length: u64,
    pub height: u32,
    pub width: u32,
    pub media_key: Vec<u8>,
    pub file_enc_sha256: Vec<u8>,
    pub direct_path: String,
    pub media_key_timestamp: i64,
    pub jpeg_thumbnail: Option<Vec<u8>>,
    pub context_info: Option<ContextInfo>,
    pub streaming_sidecar: Option<Vec<u8>>,
    pub view_once: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactMessage {
    pub display_name: String,
    pub vcard: Option<String>,
    pub context_info: Option<ContextInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationMessage {
    pub degrees_latitude: f64,
    pub degrees_longitude: f64,
    pub name: Option<String>,
    pub address: Option<String>,
    pub url: Option<String>,
    pub is_live: Option<bool>,
    pub accuracy_in_meters: Option<u32>,
    pub speed_in_mps: Option<f32>,
    pub degrees_clockwise_from_magnetic_north: Option<u32>,
    pub comment: Option<String>,
    pub jpeg_thumbnail: Option<Vec<u8>>,
    pub context_info: Option<ContextInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedTextMessage {
    pub text: String,
    pub matched_text: Option<String>,
    pub canonical_url: Option<String>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub text_argb: Option<u32>,
    pub background_argb: Option<u32>,
    pub font: Option<u32>, // enum
    pub preview_type: Option<u32>, // enum
    pub jpeg_thumbnail: Option<Vec<u8>>,
    pub context_info: Option<ContextInfo>,
    pub do_not_play_inline: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMessage {
    pub url: String,
    pub mimetype: String,
    pub title: String,
    pub file_sha256: Vec<u8>,
    pub file_length: u64,
    pub page_count: Option<u32>,
    pub media_key: Vec<u8>,
    pub file_name: String,
    pub file_enc_sha256: Vec<u8>,
    pub direct_path: String,
    pub media_key_timestamp: i64,
    pub jpeg_thumbnail: Option<Vec<u8>>,
    pub context_info: Option<ContextInfo>,
    pub thumbnail_direct_path: Option<String>,
    pub thumbnail_sha256: Option<Vec<u8>>,
    pub thumbnail_enc_sha256: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioMessage {
    pub url: String,
    pub mimetype: String,
    pub file_sha256: Vec<u8>,
    pub file_length: u64,
    pub seconds: u32,
    pub ptt: bool,
    pub media_key: Vec<u8>,
    pub file_enc_sha256: Vec<u8>,
    pub direct_path: String,
    pub media_key_timestamp: i64,
    pub context_info: Option<ContextInfo>,
    pub streaming_sidecar: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMessage {
    pub url: String,
    pub mimetype: String,
    pub file_sha256: Vec<u8>,
    pub file_length: u64,
    pub seconds: u32,
    pub media_key: Vec<u8>,
    pub caption: Option<String>,
    pub gif_playback: Option<bool>,
    pub height: u32,
    pub width: u32,
    pub file_enc_sha256: Vec<u8>,
    pub direct_path: String,
    pub media_key_timestamp: i64,
    pub jpeg_thumbnail: Option<Vec<u8>>,
    pub context_info: Option<ContextInfo>,
    pub streaming_sidecar: Option<Vec<u8>>,
    pub gif_attribution: Option<u32>, // enum
    pub view_once: Option<bool>,
    pub thumbnail_direct_path: Option<String>,
    pub thumbnail_sha256: Option<Vec<u8>>,
    pub thumbnail_enc_sha256: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactArrayMessage {
    pub display_name: String,
    pub contacts: Vec<ContactMessage>,
    pub context_info: Option<ContextInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMessage {
    pub context_info: Option<ContextInfo>,
    pub hydrated_template: Option<HydratedFourRowTemplate>,
    pub four_row_template: Option<FourRowTemplate>,
    pub hydrated_four_row_template: Option<HydratedFourRowTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FourRowTemplate {
    pub content: Option<HighlyStructuredMessage>,
    pub footer: Option<HighlyStructuredMessage>,
    pub buttons: Vec<TemplateButton>,
    pub title_document_message: Option<DocumentMessage>,
    pub title_highly_structured_message: Option<HighlyStructuredMessage>,
    pub title_image_message: Option<ImageMessage>,
    pub title_video_message: Option<VideoMessage>,
    pub title_location_message: Option<LocationMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydratedFourRowTemplate {
    pub hydrated_content_text: Option<String>,
    pub hydrated_footer_text: Option<String>,
    pub hydrated_buttons: Vec<HydratedTemplateButton>,
    pub template_id: Option<String>,
    pub title_document_message: Option<DocumentMessage>,
    pub hydrated_title_text: Option<String>,
    pub title_image_message: Option<ImageMessage>,
    pub title_video_message: Option<VideoMessage>,
    pub title_location_message: Option<LocationMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateButton {
    pub index: u32,
    pub quick_reply_button: Option<QuickReplyButton>,
    pub url_button: Option<URLButton>,
    pub call_button: Option<CallButton>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydratedTemplateButton {
    pub index: u32,
    pub quick_reply_button: Option<HydratedQuickReplyButton>,
    pub url_button: Option<HydratedURLButton>,
    pub call_button: Option<HydratedCallButton>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickReplyButton {
    pub display_text: Option<HighlyStructuredMessage>,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydratedQuickReplyButton {
    pub display_text: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct URLButton {
    pub display_text: Option<HighlyStructuredMessage>,
    pub url: Option<HighlyStructuredMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydratedURLButton {
    pub display_text: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallButton {
    pub display_text: Option<HighlyStructuredMessage>,
    pub phone_number: Option<HighlyStructuredMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydratedCallButton {
    pub display_text: String,
    pub phone_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlyStructuredMessage {
    pub namespace: Option<String>,
    pub element_name: String,
    pub params: Vec<String>,
    pub fallback_lg: Option<String>,
    pub fallback_lc: Option<String>,
    pub localizable_params: Vec<HSMLocalizableParameter>,
    pub deterministic_lg: Option<String>,
    pub deterministic_lc: Option<String>,
    pub hydrated_hsm: Option<TemplateMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HSMLocalizableParameter {
    pub default: String,
    pub currency: Option<HSMCurrency>,
    pub date_time: Option<HSMDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HSMCurrency {
    pub currency_code: String,
    pub amount_1000: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HSMDateTimeType {
    Component(HSMDateTimeComponent),
    UnixEpoch(HSMDateTimeUnixEpoch),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HSMDateTimeComponent {
    pub day_of_week: Option<u32>, // enum
    pub year: u32,
    pub month: u32,
    pub day_of_month: u32,
    pub hour: u32,
    pub minute: u32,
    pub calendar: Option<u32>, // enum
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HSMDateTimeUnixEpoch {
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInviteMessage {
    pub group_jid: String,
    pub invite_code: String,
    pub invite_expiration: i64,
    pub group_name: String,
    pub jpeg_thumbnail: Option<Vec<u8>>,
    pub caption: Option<String>,
    pub context_info: Option<ContextInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductMessage {
    pub product: ProductSnapshot,
    pub business_owner_jid: String,
    pub catalog: Option<CatalogSnapshot>,
    pub context_info: Option<ContextInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSnapshot {
    pub product_image: ImageMessage,
    pub product_id: String,
    pub title: String,
    pub description: String,
    pub currency_code: String,
    pub price_amount_1000: i64,
    pub retailer_id: String,
    pub url: Option<String>,
    pub product_image_count: u32,
    pub first_image_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogSnapshot {
    pub catalog_image: ImageMessage,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMessage {
    pub title: String,
    pub description: String,
    pub button_text: String,
    pub list_type: Option<u32>, // enum
    pub sections: Vec<Section>,
    pub context_info: Option<ContextInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub title: Option<String>,
    pub rows: Vec<Row>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
    pub title: String,
    pub description: Option<String>,
    pub row_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResponseMessage {
    pub title: Option<String>,
    pub list_type: Option<u32>, // enum
    pub single_select_reply: Option<SingleSelectReply>,
    pub context_info: Option<ContextInfo>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleSelectReply {
    pub selected_row_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonsMessage {
    pub content_text: Option<String>,
    pub footer_text: Option<String>,
    pub context_info: Option<ContextInfo>,
    pub buttons: Vec<Button>,
    pub header_type: Option<u32>, // enum
    pub text: Option<String>,
    pub document_message: Option<DocumentMessage>,
    pub image_message: Option<ImageMessage>,
    pub video_message: Option<VideoMessage>,
    pub location_message: Option<LocationMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    pub button_id: String,
    pub button_text: ButtonText,
    pub r#type: Option<u32>, // enum
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonText {
    pub display_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonsResponseMessage {
    pub selected_button_id: String,
    pub context_info: Option<ContextInfo>,
    pub r#type: Option<u32>, // enum
    pub selected_display_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickerMessage {
    pub url: String,
    pub file_sha256: Vec<u8>,
    pub file_enc_sha256: Vec<u8>,
    pub media_key: Vec<u8>,
    pub mimetype: String,
    pub height: u32,
    pub width: u32,
    pub direct_path: String,
    pub file_length: u64,
    pub media_key_timestamp: i64,
    pub first_frame_length: Option<u32>,
    pub first_frame_sidecar: Option<Vec<u8>>,
    pub is_animated: Option<bool>,
    pub png_thumbnail: Option<Vec<u8>>,
    pub context_info: Option<ContextInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveLocationMessage {
    pub degrees_latitude: f64,
    pub degrees_longitude: f64,
    pub accuracy_in_meters: Option<u32>,
    pub speed_in_mps: Option<f32>,
    pub degrees_clockwise_from_magnetic_north: Option<u32>,
    pub caption: Option<String>,
    pub sequence_number: Option<i64>,
    pub time_offset: Option<u32>,
    pub jpeg_thumbnail: Option<Vec<u8>>,
    pub context_info: Option<ContextInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolMessage {
    pub key: MessageKey,
    pub r#type: u32, // enum
    pub ephemeral_expiration: Option<u32>,
    pub ephemeral_setting_timestamp: Option<i64>,
    pub history_sync_notification: Option<HistorySyncNotification>,
    pub app_state_sync_key_share: Option<AppStateSyncKeyShare>,
    pub app_state_sync_key_request: Option<AppStateSyncKeyRequest>,
    pub initial_security_notification_setting_sync: Option<InitialSecurityNotificationSettingSync>,
    pub app_state_fatal_exception_notification: Option<AppStateFatalExceptionNotification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistorySyncNotification {
    pub file_sha256: Option<Vec<u8>>,
    pub file_length: Option<u64>,
    pub media_key: Option<Vec<u8>>,
    pub file_enc_sha256: Option<Vec<u8>>,
    pub direct_path: Option<String>,
    pub sync_type: Option<u32>, // enum
    pub chunk_order: Option<u32>,
    pub original_message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStateSyncKey {
    pub key_id: Option<AppStateSyncKeyId>,
    pub key_data: Option<AppStateSyncKeyData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStateSyncKeyId {
    pub key_id: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStateSyncKeyData {
    pub key_data: Vec<u8>,
    pub fingerprint: Option<AppStateSyncKeyFingerprint>,
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStateSyncKeyFingerprint {
    pub raw_id: u32,
    pub current_index: u32,
    pub device_indexes: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStateSyncKeyShare {
    pub keys: Vec<AppStateSyncKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStateSyncKeyRequest {
    pub key_ids: Vec<AppStateSyncKeyId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStateFatalExceptionNotification {
    pub collection_names: Vec<String>,
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitialSecurityNotificationSettingSync {
    pub security_notification_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateButtonReplyMessage {
    pub selected_id: String,
    pub selected_display_text: String,
    pub context_info: Option<ContextInfo>,
    pub selected_index: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSentMessage {
    pub destination_jid: Option<String>,
    pub message: Option<Box<Message>>,
    pub phash: Option<String>,
    pub broadcast_ephemeral_settings: Vec<EphemeralSetting>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EphemeralSetting {
    pub chat_jid: String,
    pub ephemeral_expiration: Option<u32>,
    pub ephemeral_setting_timestamp: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub conversation: Option<String>,
    pub image_message: Option<ImageMessage>,
    pub contact_message: Option<ContactMessage>,
    pub location_message: Option<LocationMessage>,
    pub extended_text_message: Option<ExtendedTextMessage>,
    pub document_message: Option<DocumentMessage>,
    pub audio_message: Option<AudioMessage>,
    pub video_message: Option<VideoMessage>,
    pub contacts_array_message: Option<ContactArrayMessage>,
    pub template_message: Option<TemplateMessage>,
    pub group_invite_message: Option<GroupInviteMessage>,
    pub product_message: Option<ProductMessage>,
    pub list_message: Option<ListMessage>,
    pub list_response_message: Option<ListResponseMessage>,
    pub buttons_message: Option<ButtonsMessage>,
    pub buttons_response_message: Option<ButtonsResponseMessage>,
    pub sticker_message: Option<StickerMessage>,
    pub live_location_message: Option<LiveLocationMessage>,
    pub protocol_message: Option<ProtocolMessage>,
    pub template_button_reply_message: Option<TemplateButtonReplyMessage>,
    pub device_sent_message: Option<DeviceSentMessage>,
    // Tambahkan field lain sesuai kebutuhan
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebMessageInfo {
    #[serde(rename = "key")]
    pub key: MessageKey,
    #[serde(rename = "message")]
    pub message: Option<Box<Message>>,
    #[serde(rename = "messageTimestamp")]
    pub message_timestamp: Option<u64>,
    #[serde(rename = "status")]
    pub status: Option<u32>, // enum
    #[serde(rename = "participant")]
    pub participant: Option<String>,
    #[serde(rename = "ignore")]
    pub ignore: Option<bool>,
    #[serde(rename = "starred")]
    pub starred: Option<bool>,
    #[serde(rename = "broadcast")]
    pub broadcast: Option<bool>,
    #[serde(rename = "pushName")]
    pub push_name: Option<String>,
    #[serde(rename = "mediaCiphertextSha256")]
    pub media_ciphertext_sha256: Option<Vec<u8>>,
    #[serde(rename = "multicast")]
    pub multicast: Option<bool>,
    #[serde(rename = "urlText")]
    pub url_text: Option<bool>,
    #[serde(rename = "urlNumber")]
    pub url_number: Option<bool>,
    #[serde(rename = "messageStubType")]
    pub message_stub_type: Option<u32>, // enum
    #[serde(rename = "clearMedia")]
    pub clear_media: Option<bool>,
    #[serde(rename = "messageStubParameters")]
    pub message_stub_parameters: Vec<String>,
    #[serde(rename = "duration")]
    pub duration: Option<u32>,
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
    #[serde(rename = "paymentInfo")]
    pub payment_info: Option<PaymentInfo>,
    #[serde(rename = "finalLiveLocation")]
    pub final_live_location: Option<LiveLocationMessage>,
    #[serde(rename = "quotedPaymentInfo")]
    pub quoted_payment_info: Option<PaymentInfo>,
    #[serde(rename = "ephemeralStartTimestamp")]
    pub ephemeral_start_timestamp: Option<u64>,
    #[serde(rename = "ephemeralDuration")]
    pub ephemeral_duration: Option<u32>,
    #[serde(rename = "ephemeralOffToOn")]
    pub ephemeral_off_to_on: Option<bool>,
    #[serde(rename = "ephemeralOutOfSync")]
    pub ephemeral_out_of_sync: Option<bool>,
    #[serde(rename = "bizPrivacyStatus")]
    pub biz_privacy_status: Option<u32>, // enum
    #[serde(rename = "verifiedBizName")]
    pub verified_biz_name: Option<String>,
}

impl Default for WebMessageInfo {
    fn default() -> Self {
        WebMessageInfo {
            key: MessageKey::default(),
            message: None,
            message_timestamp: None,
            status: Some(0), // ERROR status by default
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
            message_stub_parameters: Vec::new(),
            duration: None,
            labels: Vec::new(),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInfo {
    #[serde(rename = "currencyDeprecated")]
    pub currency_deprecated: Option<u32>, // enum
    #[serde(rename = "amount1000")]
    pub amount_1000: Option<u64>,
    #[serde(rename = "receiverJid")]
    pub receiver_jid: Option<String>,
    #[serde(rename = "status")]
    pub status: Option<u32>, // enum
    #[serde(rename = "transactionTimestamp")]
    pub transaction_timestamp: Option<u64>,
    #[serde(rename = "requestMessageKey")]
    pub request_message_key: Option<MessageKey>,
    #[serde(rename = "expiryTimestamp")]
    pub expiry_timestamp: Option<u64>,
    #[serde(rename = "futureproofed")]
    pub futureproofed: Option<bool>,
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    #[serde(rename = "txnStatus")]
    pub txn_status: Option<u32>, // enum
    #[serde(rename = "useNoviFiatFormat")]
    pub use_novi_fiat_format: Option<bool>,
    #[serde(rename = "primaryAmount")]
    pub primary_amount: Option<PaymentMoney>,
    #[serde(rename = "exchangeAmount")]
    pub exchange_amount: Option<PaymentMoney>,
}

impl Default for PaymentInfo {
    fn default() -> Self {
        PaymentInfo {
            currency_deprecated: None,
            amount_1000: None,
            receiver_jid: None,
            status: None,
            transaction_timestamp: None,
            request_message_key: None,
            expiry_timestamp: None,
            futureproofed: None,
            currency: None,
            txn_status: None,
            use_novi_fiat_format: None,
            primary_amount: None,
            exchange_amount: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMoney {
    #[serde(rename = "value")]
    pub value: Option<i64>,
    #[serde(rename = "offset")]
    pub offset: Option<u32>,
    #[serde(rename = "currencyCode")]
    pub currency_code: Option<String>,
}

impl Default for PaymentMoney {
    fn default() -> Self {
        PaymentMoney {
            value: None,
            offset: None,
            currency_code: None,
        }
    }
}

impl Default for MessageKey {
    fn default() -> Self {
        MessageKey {
            remote_jid: String::new(),
            from_me: false,
            id: String::new(),
            participant: None,
        }
    }
}