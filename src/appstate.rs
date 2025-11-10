//! Manajemen AppState untuk protokol Multi-Device
//!
//! AppState digunakan untuk menyinkronkan status aplikasi antar perangkat

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::crypto_multi_device::{IdentityKeyPair, SignedPreKeyPair, OneTimeKeyPair};
use crate::errors::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AppStateType {
    Regular,
    CriticalBlock,
    CriticalUnblockLow,
    RegularHigh,
    RegularLow,
}

impl AppStateType {
    pub fn into_string(&self) -> &'static str {
        match self {
            AppStateType::Regular => "regular",
            AppStateType::CriticalBlock => "critical_block",
            AppStateType::CriticalUnblockLow => "critical_unblock_low",
            AppStateType::RegularHigh => "regular_high",
            AppStateType::RegularLow => "regular_low",
        }
    }

    pub fn from_string(s: &str) -> Result<Self> {
        match s {
            "regular" => Ok(AppStateType::Regular),
            "critical_block" => Ok(AppStateType::CriticalBlock),
            "critical_unblock_low" => Ok(AppStateType::CriticalUnblockLow),
            "regular_high" => Ok(AppStateType::RegularHigh),
            "regular_low" => Ok(AppStateType::RegularLow),
            _ => Err("Invalid app state type".into())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStateEntry {
    pub name: String,
    pub version: u64,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStateSyncKey {
    pub key_id: Vec<u8>,
    pub key_data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStateSyncKeyShare {
    pub keys: Vec<AppStateSyncKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStateSyncKeyRequest {
    pub key_ids: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStateSyncPatch {
    pub version: u64,
    pub snapshot_mac: Vec<u8>,
    pub patch_mac: Vec<u8>,
    pub patches: Vec<AppStateEntry>,
}

/// Manajer AppState untuk sinkronisasi status aplikasi
#[derive(Debug, Clone)]
pub struct AppStateManager {
    pub state: HashMap<String, AppStateEntry>,
    pub keys: HashMap<Vec<u8>, AppStateSyncKey>,
    pub version: u64,
    pub current_state: HashMap<String, u64>, // Menyimpan versi terakhir dari setiap state
    pub pending_patches: Vec<AppStateSyncPatch>,
}

impl Default for AppStateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AppStateManager {
    pub fn new() -> Self {
        AppStateManager {
            state: HashMap::new(),
            keys: HashMap::new(),
            version: 0,
            current_state: HashMap::new(),
            pending_patches: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: AppStateEntry) {
        self.state.insert(entry.name.clone(), entry);
        self.current_state.insert(entry.name.clone(), entry.version);
    }

    pub fn get_entry(&self, name: &str) -> Option<&AppStateEntry> {
        self.state.get(name)
    }

    pub fn update_version(&mut self) {
        self.version += 1;
    }

    pub fn get_current_version(&self, name: &str) -> Option<u64> {
        self.current_state.get(name).copied()
    }

    /// Meminta sinkronisasi dari server
    pub fn request_sync(&mut self, state_type: AppStateType) -> AppStateSyncKeyRequest {
        // Dalam implementasi sebenarnya, ini akan mengirim permintaan ke server
        AppStateSyncKeyRequest {
            key_ids: vec![], // Placeholder
        }
    }

    /// Menyimpan patch yang diterima dari server
    pub fn store_patch(&mut self, patch: AppStateSyncPatch) {
        self.pending_patches.push(patch);
    }

    /// Mengaplikasikan patch yang diterima
    pub fn apply_patches(&mut self) -> Result<()> {
        for patch in self.pending_patches.drain(..) {
            for entry in patch.patches {
                self.add_entry(entry);
            }
        }
        Ok(())
    }

    /// Membuat snapshot dari state saat ini
    pub fn create_snapshot(&self) -> Result<Vec<u8>> {
        // Dalam implementasi sebenarnya, ini akan membuat snapshot terenkripsi dari state
        let serialized = bincode::serialize(&self.state)
            .map_err(|e| format!("Failed to serialize state: {}", e))?;
        Ok(serialized)
    }

    /// Memverifikasi MAC dari patch yang diterima
    pub fn verify_patch_mac(&self, patch: &AppStateSyncPatch) -> bool {
        // Dalam implementasi sebenarnya, ini akan memverifikasi MAC menggunakan kunci AppState
        !patch.patch_mac.is_empty() // Placeholder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_manager() {
        let mut manager = AppStateManager::new();
        
        let entry = AppStateEntry {
            name: "test_state".to_string(),
            version: 1,
            data: vec![1, 2, 3],
            timestamp: 1234567890,
        };
        
        manager.add_entry(entry);
        
        assert_eq!(manager.get_current_version("test_state"), Some(1));
        assert_eq!(manager.state.len(), 1);
    }

    #[test]
    fn test_app_state_type_conversion() {
        let regular = AppStateType::Regular;
        assert_eq!(regular.into_string(), "regular");
        assert_eq!(AppStateType::from_string("regular").unwrap(), AppStateType::Regular);
    }
}