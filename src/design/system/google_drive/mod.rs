/// [System Design] Google Drive (Distributed Storage)
/// Topic: Distributed Systems, Storage
/// Tags: BlockStorage, Chunking, Versioning
///
/// Link: https://bytebytego.com/courses/system-design-interview/design-google-drive

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct FileMetadata {
    pub name: String,
    pub chunk_ids: Vec<String>,
    pub version: u32,
}

pub struct GoogleDrive {
    /// Maps chunk_id to actual data (Mocking Block Storage)
    block_storage: HashMap<String, Vec<u8>>,
    /// Maps file_id to its metadata (Mocking Metadata DB)
    metadata_db: HashMap<String, FileMetadata>,
    chunk_size: usize,
}

impl GoogleDrive {
    pub fn new(chunk_size: usize) -> Self {
        Self {
            block_storage: HashMap::new(),
            metadata_db: HashMap::new(),
            chunk_size,
        }
    }

    /// Uploads a file by splitting it into chunks and storing them
    pub fn upload(&mut self, file_id: &str, file_name: String, content: &[u8]) {
        todo!("Split the content into chunks, store unique chunks, and update the metadata")
    }

    /// Downloads a file by retrieving its chunks and re-assembling them
    pub fn download(&self, file_id: &str) -> Option<Vec<u8>> {
        todo!("Retrieve the chunk list from metadata and re-assemble the file content")
    }

    /// Returns the current metadata for a file
    pub fn get_metadata(&self, file_id: &str) -> Option<FileMetadata> {
        self.metadata_db.get(file_id).cloned()
    }

    /// Helper function to generate a unique ID for a chunk (e.g., using SHA-256)
    fn hash_chunk(data: &[u8]) -> String {
        todo!("Implement a simple hash function (or mock it) to represent chunk_id")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_download() {
        let mut drive = GoogleDrive::new(4); // 4-byte chunks for testing
        let content = b"hello world!"; // 12 bytes -> 3 chunks
        
        drive.upload("file1", "test.txt".to_string(), content);
        
        let downloaded = drive.download("file1").unwrap();
        assert_eq!(downloaded, content);
    }

    #[test]
    fn test_versioning() {
        let mut drive = GoogleDrive::new(1024);
        
        drive.upload("file1", "v1.txt".to_string(), b"version 1");
        let v1 = drive.get_metadata("file1").unwrap().version;
        
        drive.upload("file1", "v2.txt".to_string(), b"version 2");
        let v2 = drive.get_metadata("file1").unwrap().version;
        
        assert!(v2 > v1);
    }

    #[test]
    fn test_deduplication() {
        let mut drive = GoogleDrive::new(4);
        let content = b"aaaa"; // 1 chunk
        
        drive.upload("file1", "f1.txt".to_string(), content);
        drive.upload("file2", "f2.txt".to_string(), content);
        
        // Both files should point to the same chunk
        let meta1 = drive.get_metadata("file1").unwrap();
        let meta2 = drive.get_metadata("file2").unwrap();
        
        assert_eq!(meta1.chunk_ids, meta2.chunk_ids);
    }
}
