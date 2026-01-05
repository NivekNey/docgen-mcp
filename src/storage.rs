//! Temporary file storage for generated PDFs
//!
//! This module provides secure, time-limited storage for generated PDF files
//! that are served via HTTP. Files are identified by UUIDs and automatically
//! expire after a configurable duration.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use uuid::Uuid;

/// Duration that files remain available (1 hour)
const FILE_EXPIRATION: Duration = Duration::from_secs(3600);

/// How often to run cleanup of expired files (every 5 minutes)
const CLEANUP_INTERVAL: Duration = Duration::from_secs(300);

/// A stored file with metadata
#[derive(Clone)]
pub struct StoredFile {
    /// The PDF file content
    pub data: Vec<u8>,
    /// When the file was created
    pub created_at: SystemTime,
    /// When the file expires
    pub expires_at: SystemTime,
    /// Original filename (for Content-Disposition header)
    pub filename: String,
}

impl StoredFile {
    /// Check if the file has expired
    pub fn is_expired(&self) -> bool {
        SystemTime::now() >= self.expires_at
    }
}

/// Thread-safe storage manager for temporary files
#[derive(Clone)]
pub struct FileStorage {
    files: Arc<RwLock<HashMap<Uuid, StoredFile>>>,
}

impl FileStorage {
    /// Create a new file storage instance
    pub fn new() -> Self {
        Self {
            files: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Store a file and return its unique ID
    ///
    /// # Arguments
    /// * `data` - The PDF file content
    /// * `filename` - The original filename (for download)
    ///
    /// # Returns
    /// A UUID that can be used to retrieve the file
    pub async fn store(&self, data: Vec<u8>, filename: String) -> Uuid {
        let id = Uuid::new_v4();
        let now = SystemTime::now();

        let stored_file = StoredFile {
            data,
            created_at: now,
            expires_at: now + FILE_EXPIRATION,
            filename,
        };

        let mut files = self.files.write().await;
        files.insert(id, stored_file);

        id
    }

    /// Retrieve a file by its ID
    ///
    /// Returns None if the file doesn't exist or has expired.
    /// Expired files are automatically removed.
    pub async fn retrieve(&self, id: &Uuid) -> Option<StoredFile> {
        let mut files = self.files.write().await;

        if let Some(file) = files.get(id) {
            if file.is_expired() {
                // Remove expired file
                files.remove(id);
                None
            } else {
                Some(file.clone())
            }
        } else {
            None
        }
    }

    /// Clean up all expired files
    ///
    /// This is called periodically by the cleanup task
    pub async fn cleanup_expired(&self) {
        let mut files = self.files.write().await;
        files.retain(|_, file| !file.is_expired());
    }

    /// Get the number of files currently stored
    pub async fn count(&self) -> usize {
        let files = self.files.read().await;
        files.len()
    }

    /// Start a background task that periodically cleans up expired files
    pub fn start_cleanup_task(self) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(CLEANUP_INTERVAL);
            loop {
                interval.tick().await;
                self.cleanup_expired().await;

                let count = self.count().await;
                tracing::debug!("Cleaned up expired files. Current count: {}", count);
            }
        });
    }
}

impl Default for FileStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_store_and_retrieve() {
        let storage = FileStorage::new();
        let data = vec![1, 2, 3, 4];
        let filename = "test.pdf".to_string();

        let id = storage.store(data.clone(), filename.clone()).await;
        let retrieved = storage.retrieve(&id).await;

        assert!(retrieved.is_some());
        let file = retrieved.unwrap();
        assert_eq!(file.data, data);
        assert_eq!(file.filename, filename);
    }

    #[tokio::test]
    async fn test_retrieve_nonexistent() {
        let storage = FileStorage::new();
        let id = Uuid::new_v4();

        let retrieved = storage.retrieve(&id).await;
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_cleanup_expired() {
        let storage = FileStorage::new();

        // Store a file
        let data = vec![1, 2, 3];
        let id = storage.store(data, "test.pdf".to_string()).await;

        // Manually expire it
        {
            let mut files = storage.files.write().await;
            if let Some(file) = files.get_mut(&id) {
                file.expires_at = SystemTime::now() - Duration::from_secs(1);
            }
        }

        // Cleanup should remove it
        storage.cleanup_expired().await;

        let count = storage.count().await;
        assert_eq!(count, 0);
    }
}
