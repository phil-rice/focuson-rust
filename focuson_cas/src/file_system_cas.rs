use std::fs;
use std::io::{self, Error, ErrorKind, Write};
use std::path::PathBuf;

use sha1::{Digest, Sha1};

use crate::storage::ContentAddressableStorage;


/// Represents a file system-based Content Addressable Storage system.
/// It allows storing and retrieving data based on content-derived identifiers.
pub struct FileSystemCAS {
    root_dir: PathBuf,
}

impl FileSystemCAS {
    pub fn new(root_dir: PathBuf) -> Self {
        Self { root_dir }
    }
    fn calculate_git_sha1(data: &[u8]) -> String {
        let mut hasher = Sha1::new();

        // Create a header similar to what Git would use: "blob <size>\0"
        let header = format!("blob {}\0", data.len());

        // Update the hasher with the header first
        hasher.update(header.as_bytes());

        // Then update the hasher with the actual data
        hasher.update(data);

        // Finalize the hashing process to get the result
        let result = hasher.finalize();  // This returns an instance of sha1::Output

        // Convert the hash result to a slice of bytes
        let result_bytes = result.as_slice(); // Now result_bytes is &[u8]

        // Format each byte as a hex string and concatenate them
        result_bytes
            .iter()
            .map(|byte| format!("{:02x}", byte)) // Format each byte as two hexadecimal characters
            .collect::<Vec<String>>() // Collect all the formatted strings into a Vec
            .join("") // Join all strings in the Vec into a single String without any separator
    }


    fn path_from_id(&self, id: &str) -> PathBuf {
        let (xx, rest) = id.split_at(2);
        let (yy, rest) = rest.split_at(2);
        self.root_dir.join(xx).join(yy).join(rest)
    }
}

impl ContentAddressableStorage for FileSystemCAS {
    fn store(&self, data: &[u8]) -> Result<String, Error> {
        let id = Self::calculate_git_sha1(data);
        let path = self.path_from_id(&id);

        fs::create_dir_all(path.parent().unwrap())?;
        let mut file = fs::File::create(path)?;
        file.write_all(data)?;
        Ok(id)
    }

    fn retrieve(&self, id: &str) -> Result<Vec<u8>, Error> {
        let path = self.path_from_id(id);
        let data = fs::read(&path)?;

        // Calculate the SHA1 hash of the retrieved data
        let data_hash = Self::calculate_git_sha1(&data);

        // Compare the calculated hash with the provided id
        if data_hash == id {
            Ok(data)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Hash mismatch"))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_sha1_calculation() {
        let data = b"hello world";
        let expected_sha1 = "95d09f2b10159347eece71399a7e2e907ea3df4f"; // Precomputed SHA-1 for "hello world"
        assert_eq!(FileSystemCAS::calculate_git_sha1(data), expected_sha1);
    }

    #[test]
    fn test_store_retrieve_round_trip() {
        let temp_dir = TempDir::new().unwrap();
        let cas = FileSystemCAS::new(temp_dir.path().to_path_buf());

        let data = b"Hello, world!";
        let id = cas.store(data).expect("Failed to store data");
        let retrieved_data = cas.retrieve(&id).expect("Failed to retrieve data");

        assert_eq!(data, &retrieved_data[..]);
    }

    #[test]
    fn test_path_from_id() {
        let temp_dir = TempDir::new().unwrap();
        let cas = FileSystemCAS::new(temp_dir.path().to_path_buf());

        let id = "e8d95a51f3af4a3b134bf6bb680a213a"; // Example SHA-1 ID
        let constructed_path = cas.path_from_id(&id);
        let expected_path = temp_dir
            .path()
            .join("e8")
            .join("d9")
            .join("5a51f3af4a3b134bf6bb680a213a");

        assert_eq!(constructed_path, expected_path);
    }

    #[test]
    fn test_retrieve_nonexistent_content() {
        let temp_dir = TempDir::new().unwrap();
        let cas = FileSystemCAS::new(temp_dir.path().to_path_buf());

        let non_existent_id = "0000000000000000000000000000000000000000"; // An unlikely SHA-1 hash
        match cas.retrieve(&non_existent_id) {
            Ok(_) => panic!("Should have failed to retrieve non-existent content"),
            Err(e) => assert_eq!(e.kind(), io::ErrorKind::NotFound),
        }
    }

    #[test]
    fn test_retrieve_mismatched_hash() {
        let temp_dir = TempDir::new().unwrap();
        let cas = FileSystemCAS::new(temp_dir.path().to_path_buf());

        // Arrange: Create a file at a location that does not match its content's hash
        let data = b"This is some test data.";
        let incorrect_hash = "0000000000000000000000000000000000000000"; // Intentionally incorrect hash
        let path = cas.path_from_id(&incorrect_hash);
        fs::create_dir_all(path.parent().unwrap()).unwrap(); // Ensure the directory exists
        let mut file = fs::File::create(&path).unwrap(); // Create the file
        file.write_all(data).unwrap(); // Write data

        // Act: Attempt to retrieve data using the incorrect hash
        let result = cas.retrieve(&incorrect_hash);

        // Assert: Check the retrieval result
        assert_eq!(
            result.unwrap_err().kind(),
            std::io::ErrorKind::InvalidData,
            "Expected an InvalidData error"
        );
    }
}
