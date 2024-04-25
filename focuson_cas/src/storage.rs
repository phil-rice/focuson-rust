// src/storage.rs

use std::io::Result;

/// Defines the interface for a content addressable storage mechanism.
pub trait ContentAddressableStorage {
    fn store(&self, data: &[u8]) -> Result<String>;
    fn retrieve(&self, id: &str) -> Result<Vec<u8>>;
}
