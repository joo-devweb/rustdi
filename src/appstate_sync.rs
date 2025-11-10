//! Protokol Sinkronisasi AppState untuk WhatsApp Multi-Device
//!
//! Modul ini mengimplementasikan protokol sinkronisasi AppState yang digunakan
//! untuk menyinkronkan status aplikasi antar perangkat WhatsApp

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::errors::*;
use crate::appstate::{AppStateManager, AppStateType, AppStateSyncKey, AppStateSyncKeyRequest, AppStateSyncPatch};
use crate::session::MultiDeviceSession;
use crate::node_protocol::{AppMessage, AppEvent};
use crate::node_wire::{Node, NodeContent};

/// Jenis sinkronisasi AppState
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppStateSyncKind {
    Full,
    Patch,
    Initial,
}

/// Permintaan sinkronisasi AppState
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStateSyncRequest {
    pub r#type: AppStateSyncKind,
    pub app_state_types: Vec<AppStateType>,
    pub version: u64,
}

impl AppStateSyncRequest {
    /// Membuat permintaan sinkronisasi AppState
    pub fn new(kind: AppStateSyncKind, types: Vec<AppStateType>) -> Self {
        AppStateSyncRequest {
            r#type: kind,
            app_state_types: types,
            version: 0,
        }
    }

    /// Mengonversi permintaan ke format node untuk dikirim ke server
    pub fn to_node(&self) -> Node {
        let mut attributes = HashMap::new();
        
        match self.r#type {
            AppStateSyncKind::Full => {
                attributes.insert("type".into(), NodeContent::Token("full"));
            },
            AppStateSyncKind::Patch => {
                attributes.insert("type".into(), NodeContent::Token("patch"));
            },
            AppStateSyncKind::Initial => {
                attributes.insert("type".into(), NodeContent::Token("initial"));
            },
        }

        // Konversi AppStateType ke string
        let types_str: Vec<String> = self.app_state_types
            .iter()
            .map(|t| t.into_string().to_string())
            .collect();

        // Gunakan list untuk menyimpan tipe-tipe AppState
        let content = NodeContent::List(types_str.iter()
            .map(|s| {
                let mut attr = HashMap::new();
                attr.insert("name".into(), NodeContent::String(s.as_str().into()));
                Node::new("collection", attr, NodeContent::None)
            })
            .collect());

        Node::new("appstate", attributes, content)
    }
}

/// Respon dari sinkronisasi AppState
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStateSyncResponse {
    pub patches: Vec<AppStateSyncPatch>,
    pub snapshot: Option<Vec<u8>>,
    pub has_more: bool,
}

impl AppStateSyncResponse {
    /// Membaca respon dari node yang diterima dari server
    pub fn from_node(node: &Node) -> Result<Self> {
        let mut patches = Vec::new();
        let mut has_more = false;
        
        if let NodeContent::List(children) = &node.content {
            for child in children {
                if child.desc() == "patch" {
                    // Parsing patch dari node
                    if let Some(version_str) = child.get_attribute("version").ok() {
                        let version = version_str.as_str().parse::<u64>()
                            .map_err(|_| "Invalid version in patch")?;
                        
                        // Dalam implementasi sebenarnya, kita akan mengurai data patch di sini
                        patches.push(AppStateSyncPatch {
                            version,
                            snapshot_mac: vec![], // Placeholder
                            patch_mac: vec![], // Placeholder
                            patches: vec![], // Placeholder
                        });
                    }
                } else if child.desc() == "sync" {
                    has_more = true;
                }
            }
        }

        Ok(AppStateSyncResponse {
            patches,
            snapshot: None, // Dalam implementasi sebenarnya, snapshot akan disimpan di sini
            has_more,
        })
    }
}

/// Manajer untuk sinkronisasi AppState
pub struct AppStateSyncManager {
    /// Manajer AppState dasar
    pub app_state_manager: AppStateManager,
    /// Daftar permintaan sinkronisasi yang sedang menunggu
    pub pending_requests: HashMap<String, AppStateSyncRequest>,
    /// Session untuk mengakses kunci dan informasi identitas
    pub session: MultiDeviceSession,
}

impl AppStateSyncManager {
    /// Membuat manajer baru
    pub fn new(session: MultiDeviceSession) -> Self {
        AppStateSyncManager {
            app_state_manager: AppStateManager::new(),
            pending_requests: HashMap::new(),
            session,
        }
    }

    /// Meminta sinkronisasi untuk tipe AppState tertentu
    pub fn request_sync(&mut self, types: Vec<AppStateType>) -> Result<AppMessage> {
        let request = AppStateSyncRequest::new(AppStateSyncKind::Initial, types);
        
        // Buat ID unik untuk permintaan ini
        let request_id = uuid::Uuid::new_v4().to_string();
        self.pending_requests.insert(request_id.clone(), request);

        // Buat AppMessage untuk permintaan sinkronisasi
        let mut attributes = HashMap::new();
        attributes.insert("id".into(), NodeContent::String(request_id.into()));
        attributes.insert("type".into(), NodeContent::Token("critical_block"));
        
        let node = Node::new("query", attributes, NodeContent::List(vec![]));
        
        Ok(AppMessage::Query(crate::node_protocol::Query::MessagesBefore {
            jid: crate::Jid { id: "status@broadcast".to_string(), is_group: false }, // Placeholder
            id: request_id,
            count: 1,
        }))
    }

    /// Menangani respon sinkronisasi dari server
    pub fn handle_sync_response(&mut self, response: &AppStateSyncResponse) -> Result<()> {
        // Aplikasikan semua patch yang diterima
        for patch in &response.patches {
            if self.app_state_manager.verify_patch_mac(patch) {
                // Dalam implementasi sebenarnya, kita akan mengaplikasikan patch ke state
                // For now, let's just update the manager's state
                self.app_state_manager.update_version();
            } else {
                return Err("Invalid patch MAC".into());
            }
        }

        Ok(())
    }

    /// Mengambil kunci AppState yang diperlukan untuk dekripsi
    pub fn get_app_state_key(&self, key_id: &[u8]) -> Option<&AppStateSyncKey> {
        self.app_state_manager.keys.get(key_id)
    }

    /// Mendaftarkan kunci AppState baru
    pub fn register_app_state_key(&mut self, key: AppStateSyncKey) {
        self.app_state_manager.keys.insert(key.key_id.clone(), key);
    }

    /// Mengambil versi terakhir dari tipe AppState tertentu
    pub fn get_current_version(&self, app_state_type: &str) -> Option<u64> {
        self.app_state_manager.current_state.get(app_state_type).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::MultiDeviceSession;

    #[test]
    fn test_app_state_sync_request() {
        let types = vec![AppStateType::Regular, AppStateType::CriticalBlock];
        let request = AppStateSyncRequest::new(AppStateSyncKind::Initial, types);
        
        let node = request.to_node();
        assert_eq!(node.desc(), "appstate");
    }

    #[test]
    fn test_app_state_sync_manager() {
        let session = MultiDeviceSession::default();
        let mut manager = AppStateSyncManager::new(session);

        let types = vec![AppStateType::Regular];
        let result = manager.request_sync(types);
        assert!(result.is_ok());

        // Test response handling
        let response = AppStateSyncResponse {
            patches: vec![],
            snapshot: None,
            has_more: false,
        };
        assert!(manager.handle_sync_response(&response).is_ok());
    }
}