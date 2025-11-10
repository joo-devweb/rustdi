use crate::errors::*;
use std::collections::HashMap;

/// Representasi struktur WebMessageInfo (protobuf root)
#[derive(Debug, Clone)]
pub struct WebMessageInfo {
    pub key: MessageKey,
    pub message: Option<Message>,
    pub message_timestamp: Option<u64>,
    pub status: Option<u32>,
    pub participant: Option<String>,
    pub ignore: Option<bool>,
    pub starred: Option<bool>,
    pub broadcast: Option<bool>,
    pub push_name: Option<String>,
    pub media_ciphertext_sha256: Option<Vec<u8>>,
    pub multicast: Option<bool>,
    pub url_text: Option<bool>,
    pub url_number: Option<bool>,
    pub message_stub_type: Option<u32>,
    pub clear_media: Option<bool>,
    pub message_stub_parameters: Vec<String>,
    pub duration: Option<u32>,
    pub labels: Vec<String>,
    pub payment_info: Option<PaymentInfo>,
    pub final_live_location: Option<LiveLocationMessage>,
    pub quoted_payment_info: Option<PaymentInfo>,
    pub ephemeral_start_timestamp: Option<u64>,
    pub ephemeral_duration: Option<u32>,
    pub ephemeral_off_to_on: Option<bool>,
    pub ephemeral_out_of_sync: Option<bool>,
    pub biz_privacy_status: Option<u32>,
    pub verified_biz_name: Option<String>,
}

/// Kunci pesan
#[derive(Debug, Clone)]
pub struct MessageKey {
    pub remote_jid: String,
    pub from_me: bool,
    pub id: String,
    pub participant: Option<String>,
}

/// Struktur pesan utama
#[derive(Debug, Clone)]
pub struct Message {
    pub conversation: Option<String>,
    pub image_message: Option<ImageMessage>,
    pub contact_message: Option<ContactMessage>,
    pub location_message: Option<LocationMessage>,
    pub extended_text_message: Option<ExtendedTextMessage>,
    pub document_message: Option<DocumentMessage>,
    pub audio_message: Option<AudioMessage>,
    pub video_message: Option<VideoMessage>,
    pub call: Option<Call>,
    pub chat: Option<Chat>,
    pub protocol_message: Option<ProtocolMessage>,
    pub contacts_array_message: Option<ContactsArrayMessage>,
    pub highly_structured_message: Option<HighlyStructuredMessage>,
    pub fast_ratchet_key_sender_key_distribution_message: Option<SenderKeyDistributionMessage>,
    pub send_payment_message: Option<SendPaymentMessage>,
    pub live_location_message: Option<LiveLocationMessage>,
    pub request_payment_message: Option<RequestPaymentMessage>,
    pub decline_payment_message: Option<DeclinePaymentRequestMessage>,
    pub cancel_payment_message: Option<CancelPaymentRequestMessage>,
    pub template_message: Option<TemplateMessage>,
    pub sticker_message: Option<StickerMessage>,
    pub group_invite_message: Option<GroupInviteMessage>,
    pub template_button_reply_message: Option<TemplateButtonReplyMessage>,
    pub product_message: Option<ProductMessage>,
    pub device_sent_message: Option<DeviceSentMessage>,
    pub message_context_info: Option<MessageContextInfo>,
    pub list_message: Option<ListMessage>,
    pub list_response_message: Option<ListResponseMessage>,
    pub buttons_response_message: Option<ButtonsResponseMessage>,
    pub buttons_message: Option<ButtonsMessage>,
    pub payment_invitation_message: Option<PaymentInvitationMessage>,
    pub interactive_message: Option<InteractiveMessage>,
    pub reaction_message: Option<ReactionMessage>,
    pub sticker_sync_rmr_message: Option<StickerSyncRMRMessage>,
    pub interactive_response_message: Option<InteractiveResponseMessage>,
    pub poll_creation_message: Option<PollCreationMessage>,
    pub poll_update_message: Option<PollUpdateMessage>,
    pub keep_in_chat_message: Option<KeepInChatMessage>,
}

#[derive(Debug, Clone)]
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
    pub context_info: Option<MessageContextInfo>,
    pub streaming_sidecar: Option<Vec<u8>>,
    pub view_once: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ContactMessage {
    pub display_name: String,
    pub vcard: String,
    pub context_info: Option<MessageContextInfo>,
}

#[derive(Debug, Clone)]
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
    pub context_info: Option<MessageContextInfo>,
}

#[derive(Debug, Clone)]
pub struct ExtendedTextMessage {
    pub text: String,
    pub matched_text: Option<String>,
    pub canonical_url: Option<String>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub text_argb: Option<u32>,
    pub background_argb: Option<u32>,
    pub font: Option<u32>,
    pub preview_type: Option<u32>,
    pub jpeg_thumbnail: Option<Vec<u8>>,
    pub context_info: Option<MessageContextInfo>,
    pub do_not_play_inline: Option<bool>,
}

#[derive(Debug, Clone)]
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
    pub context_info: Option<MessageContextInfo>,
    pub thumbnail_direct_path: Option<String>,
    pub thumbnail_sha256: Option<Vec<u8>>,
    pub thumbnail_enc_sha256: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
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
    pub context_info: Option<MessageContextInfo>,
    pub streaming_sidecar: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
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
    pub context_info: Option<MessageContextInfo>,
    pub streaming_sidecar: Option<Vec<u8>>,
    pub gif_attribution: Option<u32>,
    pub view_once: Option<bool>,
    pub thumbnail_direct_path: Option<String>,
    pub thumbnail_sha256: Option<Vec<u8>>,
    pub thumbnail_enc_sha256: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub call_key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Chat {
    pub display_name: String,
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct ProtocolMessage {
    pub key: MessageKey,
    pub r#type: Option<u32>,
    pub ephemeral_expiration: Option<u32>,
    pub ephemeral_setting_timestamp: Option<i64>,
    pub history_sync_notification: Option<HistorySyncNotification>,
    pub app_state_sync_key_share: Option<AppStateSyncKeyShare>,
    pub app_state_sync_key_request: Option<AppStateSyncKeyRequest>,
    pub initial_security_notification_setting_sync: Option<InitialSecurityNotificationSettingSync>,
    pub app_state_fatal_exception_notification: Option<AppStateFatalExceptionNotification>,
}

#[derive(Debug, Clone)]
pub struct HistorySyncNotification {
    pub file_sha256: Option<Vec<u8>>,
    pub file_length: Option<u64>,
    pub media_key: Option<Vec<u8>>,
    pub file_enc_sha256: Option<Vec<u8>>,
    pub direct_path: Option<String>,
    pub sync_type: Option<u32>,
    pub chunk_order: Option<u32>,
    pub original_message_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ContactsArrayMessage {
    pub display_name: String,
    pub contacts: Vec<ContactMessage>,
    pub context_info: Option<MessageContextInfo>,
}

#[derive(Debug, Clone)]
pub struct SenderKeyDistributionMessage {
    pub group_id: String,
    pub axolotl_sender_key_distribution_message: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct MessageContextInfo {
    pub device_list_metadata: Option<DeviceListMetadata>,
    pub mentioned_jid: Vec<String>,
    pub is_forwarded: Option<bool>,
    pub forwarded_source_from: Option<String>,
    pub participant: Option<String>,
    pub orphaned_device_sent_message_number: Option<u32>,
    pub orphaned_device_sent_message_epoch: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct DeviceListMetadata {
    pub sender_key_hash: Option<Vec<u8>>,
    pub sender_timestamp: Option<u64>,
    pub recipient_key_hash: Option<Vec<u8>>,
    pub recipient_timestamp: Option<u64>,
    pub sender_epoch: Option<u32>,
    pub recipient_epoch: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct PaymentInfo {
    pub currency: String,
    pub amount_1000: u64,
    pub receiver_jid: String,
    pub status: u32,
    pub transaction_timestamp: u64,
    pub request_message_key: Option<MessageKey>,
    pub expiry_timestamp: u64,
    pub futureproofed: bool,
    pub currency_code_iso4217: String,
}

#[derive(Debug, Clone)]
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
    pub context_info: Option<MessageContextInfo>,
}

#[derive(Debug, Clone)]
pub struct AppStateSyncKeyShare {
    pub keys: Vec<AppStateSyncKey>,
}

#[derive(Debug, Clone)]
pub struct AppStateSyncKey {
    pub key_id: Option<AppStateSyncKeyId>,
    pub key_data: Option<AppStateSyncKeyData>,
}

#[derive(Debug, Clone)]
pub struct AppStateSyncKeyId {
    pub key_id: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct AppStateSyncKeyData {
    pub key_data: Vec<u8>,
    pub fingerprint: Option<AppStateSyncKeyFingerprint>,
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct AppStateSyncKeyFingerprint {
    pub raw_id: u32,
    pub current_index: u32,
    pub device_indexes: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct GroupInviteMessage {
    pub group_jid: String,
    pub invite_code: String,
    pub invite_expiration: i64,
    pub group_name: String,
    pub jpeg_thumbnail: Option<Vec<u8>>,
    pub caption: Option<String>,
    pub context_info: Option<MessageContextInfo>,
}

#[derive(Debug, Clone)]
pub struct TemplateMessage {
    pub context_info: Option<MessageContextInfo>,
    pub hydrated_template: Option<HydratedFourRowTemplate>,
    pub template_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HydratedFourRowTemplate {
    pub hydrated_content_text: Option<String>,
    pub hydrated_footer_text: Option<String>,
    pub hydrated_buttons: Vec<HydratedTemplateButton>,
    pub template_id: Option<String>,
    pub hydrated_title_text: Option<String>,
    pub hydrated_subtitle_text: Option<String>,
    pub hydrated_image_caption: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HydratedTemplateButton {
    pub index: u32,
    pub quick_reply_button: Option<HydratedQuickReplyButton>,
    pub url_button: Option<HydratedURLButton>,
    pub call_button: Option<HydratedCallButton>,
    pub currency_button: Option<HydratedCurrencyButton>,
}

#[derive(Debug, Clone)]
pub struct HydratedQuickReplyButton {
    pub display_text: String,
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct HydratedURLButton {
    pub display_text: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct HydratedCallButton {
    pub display_text: String,
    pub phone_number: String,
}

#[derive(Debug, Clone)]
pub struct HydratedCurrencyButton {
    pub display_text: String,
    pub currency: HydratedCurrency,
}

#[derive(Debug, Clone)]
pub struct HydratedCurrency {
    pub currency_code: String,
    pub amount_1000: i64,
    pub offset: u32,
    pub total_amount: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ListMessage {
    pub title: String,
    pub description: String,
    pub button_text: String,
    pub list_type: u32,
    pub sections: Vec<ListSection>,
    pub context_info: Option<MessageContextInfo>,
    pub footer_text: Option<String>,
    pub carousel_selection_header: Option<CarouselMessageHeader>,
}

#[derive(Debug, Clone)]
pub struct ListSection {
    pub title: String,
    pub rows: Vec<ListRow>,
}

#[derive(Debug, Clone)]
pub struct ListRow {
    pub title: String,
    pub description: String,
    pub row_id: String,
}

#[derive(Debug, Clone)]
pub struct CarouselMessageHeader {
    pub image_message: Option<ImageMessage>,
    pub video_message: Option<VideoMessage>,
    pub document_message: Option<DocumentMessage>,
    pub location_message: Option<LocationMessage>,
}

#[derive(Debug, Clone)]
pub struct ButtonsMessage {
    pub content_text: String,
    pub footer_text: Option<String>,
    pub context_info: Option<MessageContextInfo>,
    pub buttons: Vec<Button>,
    pub header_type: u32,
    pub image_message: Option<ImageMessage>,
    pub video_message: Option<VideoMessage>,
    pub document_message: Option<DocumentMessage>,
    pub location_message: Option<LocationMessage>,
    pub contact_message: Option<ContactMessage>,
}

#[derive(Debug, Clone)]
pub struct Button {
    pub button_id: String,
    pub button_text: String,
    pub r#type: u32,
}

#[derive(Debug, Clone)]
pub struct ListResponseMessage {
    pub title: String,
    pub list_type: u32,
    pub single_select_reply: Option<SingleSelectReply>,
    pub context_info: Option<MessageContextInfo>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SingleSelectReply {
    pub selected_row_id: String,
}

#[derive(Debug, Clone)]
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
    pub context_info: Option<MessageContextInfo>,
}

#[derive(Debug, Clone)]
pub struct ReactionMessage {
    pub key: MessageKey,
    pub text: String,
    pub grouping_key: String,
    pub type_field: Option<u32>,
    pub sender_timestamp_ms: i64,
}

#[derive(Debug, Clone)]
pub struct StickerSyncRMRMessage {
    pub rmr_reason: u32,
    pub requesting_phone_number: String,
    pub sticker_message_id: Option<String>,
    pub total_requested: u32,
}

#[derive(Debug, Clone)]
pub struct PollCreationMessage {
    pub name: String,
    pub selectable_count: u32,
    pub options: Vec<PollOption>,
    pub context_info: Option<MessageContextInfo>,
}

#[derive(Debug, Clone)]
pub struct PollOption {
    pub option_name: String,
}

#[derive(Debug, Clone)]
pub struct PollUpdateMessage {
    pub poll_update: PollUpdate,
    pub message: Option<Message>,
    pub sender_timestamp_ms: i64,
}

#[derive(Debug, Clone)]
pub struct PollUpdate {
    pub vote: PollEncValue,
}

#[derive(Debug, Clone)]
pub struct PollEncValue {
    pub enc_iv: Vec<u8>,
    pub enc_payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct KeepInChatMessage {
    pub key: MessageKey,
    pub action: u32,
    pub timestamp_ms: i64,
    pub message: Option<Message>,
    pub sender_timestamp_ms: i64,
}

#[derive(Debug, Clone)]
pub struct InteractiveMessage {
    pub header: Option<InteractiveMessageHeader>,
    pub body: Option<InteractiveMessageBody>,
    pub footer: Option<InteractiveMessageFooter>,
    pub native_flow_message: Option<NativeFlowMessage>,
    pub context_info: Option<MessageContextInfo>,
}

#[derive(Debug, Clone)]
pub struct InteractiveMessageHeader {
    pub title: String,
    pub subtitle: Option<String>,
    pub has_media_attachment: Option<bool>,
    pub image_message: Option<ImageMessage>,
    pub video_message: Option<VideoMessage>,
    pub document_message: Option<DocumentMessage>,
}

#[derive(Debug, Clone)]
pub struct InteractiveMessageBody {
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct InteractiveMessageFooter {
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct NativeFlowMessage {
    pub buttons: Vec<NativeFlowButton>,
    pub message_params_json: String,
}

#[derive(Debug, Clone)]
pub struct NativeFlowButton {
    pub name: String,
    pub button_params_json: String,
}

#[derive(Debug, Clone)]
pub struct InteractiveResponseMessage {
    pub native_flow_response_message: Option<NativeFlowResponseMessage>,
}

#[derive(Debug, Clone)]
pub struct NativeFlowResponseMessage {
    pub name: String,
    pub params_json: String,
}

#[derive(Debug, Clone)]
pub struct HighlyStructuredMessage {
    pub namespace: String,
    pub element_name: String,
    pub params: Vec<String>,
    pub fallback_lg: String,
    pub fallback_lc: String,
    pub localizable_params: Vec<HSMLocalizableParameter>,
    pub deterministic_lg: Option<String>,
    pub deterministic_lc: Option<String>,
    pub hydrated_hsm: Option<TemplateMessage>,
}

#[derive(Debug, Clone)]
pub struct HSMLocalizableParameter {
    pub default: String,
    pub currency: Option<HSMCurrency>,
    pub date_time_component: Option<HSMDateTimeComponent>,
}

#[derive(Debug, Clone)]
pub struct HSMCurrency {
    pub currency_code: String,
    pub amount_1000: i64,
}

#[derive(Debug, Clone)]
pub struct HSMDateTimeComponent {
    pub day_of_week: u32,
    pub year: u32,
    pub month: u32,
    pub day_of_month: u32,
    pub hour: u32,
    pub minute: u32,
    pub calendar: u32,
}

#[derive(Debug, Clone)]
pub struct SendPaymentMessage {
    pub note_message: Option<Message>,
    pub request_message_key: Option<MessageKey>,
}

#[derive(Debug, Clone)]
pub struct RequestPaymentMessage {
    pub note_message: Option<Message>,
    pub currency_code_iso4217: String,
    pub amount_1000: u64,
    pub request_from: String,
    pub expiry_timestamp: i64,
    pub amount: Option<PaymentMoney>,
    pub request_status: Option<u32>,
    pub background_url: Option<String>,
    pub text_attribution: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct DeclinePaymentRequestMessage {
    pub key: MessageKey,
}

#[derive(Debug, Clone)]
pub struct CancelPaymentRequestMessage {
    pub key: MessageKey,
}

#[derive(Debug, Clone)]
pub struct PaymentMoney {
    pub value: i64,
    pub offset: u32,
    pub currency_code: String,
}

#[derive(Debug, Clone)]
pub struct ProductMessage {
    pub product_snapshot: ProductSnapshot,
    pub business_owner_jid: String,
    pub catalog_image_count: Option<u32>,
    pub context_info: Option<MessageContextInfo>,
}

#[derive(Debug, Clone)]
pub struct ProductSnapshot {
    pub product_image: ImageMessage,
    pub product_title: String,
    pub product_description: String,
    pub product_currency_code: String,
    pub product_price_amount_1000: i64,
    pub product_id: String,
    pub product_additional_image_count: u32,
    pub product_image_urls: Vec<String>,
    pub is_sold_out: Option<bool>,
    pub merchantable: Option<bool>,
    pub product_retailer_id: Option<String>,
    pub url: Option<String>,
    pub product_image_alt_text: String,
    pub secondary_sub_title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DeviceSentMessage {
    pub destination_jid: Option<String>,
    pub message: Option<Message>,
    pub phash: Option<String>,
    pub broadcast_ephemeral_settings: Option<BroadcastEphemeralSettings>,
}

#[derive(Debug, Clone)]
pub struct BroadcastEphemeralSettings {
    pub chat_jid: String,
    pub ephemeral_expiration: Option<u32>,
    pub ephemeral_setting_timestamp: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct TemplateButtonReplyMessage {
    pub selected_id: String,
    pub selected_display_text: String,
    pub context_info: Option<MessageContextInfo>,
    pub selected_index: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct PaymentInvitationMessage {
    pub currency: String,
    pub amount_1000: u64,
    pub receiver_jid: String,
    pub note_message: Option<Message>,
    pub expiry_timestamp: i64,
    pub amount: Option<PaymentMoney>,
    pub payment_terms: Option<String>,
    pub amount_text: Option<String>,
    pub currency_code: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InitialSecurityNotificationSettingSync {
    pub security_notification_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct AppStateFatalExceptionNotification {
    pub collection_names: Vec<String>,
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct AppStateSyncKeyRequest {
    pub key_ids: Vec<AppStateSyncKeyId>,
}

#[derive(Debug, Clone)]
pub struct DeviceListMetadataCollection {
    pub r#type: u32,
    pub user_devices: Vec<DeviceInfo>,
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub jid: String,
    pub device_id: Vec<u32>,
    pub identity_id: Vec<u8>,
    pub registration_id: u32,
    pub local_registration_id: Option<u32>,
    pub verified_name: Option<String>,
    pub business_name: Option<String>,
    pub r#type: Option<u32>,
    pub null_device: Option<NullDevice>,
    pub is_planned_account_migration_device: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct NullDevice {
    pub jid: String,
    pub device_id: Vec<u32>,
    pub identity_id: Vec<u8>,
    pub registration_id: u32,
    pub null_device_token: Vec<u8>,
    pub device_key: Vec<u8>,
}