// cas_helpers.rs
use crate::storage::ContentAddressableStorage;
use std::io::{self, Result};

/// Provides extension methods for storing and retrieving strings in a
/// content addressable storage.
pub trait StringStorage {
    fn store_string(&self, data: &str) -> Result<String>;
    fn retrieve_string(&self, id: &str) -> Result<String>;
}

impl<T: ContentAddressableStorage> StringStorage for T {
    fn store_string(&self, data: &str) -> Result<String> {
        self.store(data.as_bytes())
    }

    fn retrieve_string(&self, id: &str) -> Result<String> {
        self.retrieve(id).and_then(|bytes| {
            String::from_utf8(bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_system_cas::FileSystemCAS;
    use tempfile::TempDir;

    #[test]
    fn test_store_and_retrieve_string() {
        let temp_dir = TempDir::new().unwrap();
        let cas = FileSystemCAS::new(temp_dir.path().to_path_buf());

        let original_string = "Hello, World!";
        let id = cas.store_string(original_string).unwrap();
        let retrieved_string = cas.retrieve_string(&id).unwrap();

        assert_eq!(original_string, retrieved_string);
    }

    #[test]
    fn test_retrieve_nonexistent_string() {
        let temp_dir = TempDir::new().unwrap();
        let cas = FileSystemCAS::new(temp_dir.path().to_path_buf());

        let id = "nonexistent_id";
        assert!(cas.retrieve_string(id).is_err());
    }
}
