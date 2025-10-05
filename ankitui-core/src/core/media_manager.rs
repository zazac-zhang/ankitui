//! Media Management Module
//!
//! Handles media file operations, validation, and metadata extraction

use crate::data::models::{EnhancedMediaRef, MediaMetadata, MediaRef, MediaStatus, MediaType};
use anyhow::{anyhow, Result};
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Media manager for handling card media files
pub struct MediaManager {
    /// Base directory for media storage
    media_dir: PathBuf,
    /// Maximum file size in bytes (default: 50MB)
    max_file_size: u64,
}

impl MediaManager {
    /// Create a new media manager with the given media directory
    pub fn new<P: AsRef<Path>>(media_dir: P) -> Self {
        Self {
            media_dir: PathBuf::from(media_dir.as_ref()),
            max_file_size: 50 * 1024 * 1024, // 50MB
        }
    }

    /// Set maximum file size for media files
    pub fn set_max_file_size(&mut self, size: u64) {
        self.max_file_size = size;
    }

    /// Ensure media directory exists
    pub fn ensure_media_dir(&self) -> Result<()> {
        if !self.media_dir.exists() {
            fs::create_dir_all(&self.media_dir)?;
        }
        Ok(())
    }

    /// Add a media file to the media collection
    pub async fn add_media_file<P: AsRef<Path>>(
        &self,
        source_path: P,
        media_type: MediaType,
    ) -> Result<EnhancedMediaRef> {
        self.ensure_media_dir()?;

        let source = source_path.as_ref();
        if !source.exists() {
            return Err(anyhow!(
                "Source media file does not exist: {}",
                source.display()
            ));
        }

        // Validate file size
        let file_size = fs::metadata(source)?.len();
        if file_size > self.max_file_size {
            return Err(anyhow!(
                "Media file too large: {} bytes (max: {})",
                file_size,
                self.max_file_size
            ));
        }

        // Generate unique filename
        let file_extension = source
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("bin");

        let filename = format!("{}.{}", Uuid::new_v4(), file_extension);
        let dest_path = self.media_dir.join(&filename);

        // Copy file to media directory
        fs::copy(source, &dest_path)?;

        // Extract metadata
        let metadata = self
            .extract_metadata(&dest_path, &filename, file_size)
            .await?;

        Ok(EnhancedMediaRef {
            id: Uuid::new_v4(),
            path: filename,
            media_type,
            metadata,
            status: MediaStatus::Available,
            local_cache_path: Some(dest_path.to_string_lossy().to_string()),
            remote_url: None,
            alt_text: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Add media from URL (downloads and caches locally)
    pub async fn add_media_from_url(
        &self,
        url: &str,
        media_type: MediaType,
    ) -> Result<EnhancedMediaRef> {
        self.ensure_media_dir()?;

        // TODO: Implement URL download functionality
        // For now, create a reference without local caching
        let filename = format!(
            "{}.{}",
            Uuid::new_v4(),
            self.get_extension_for_type(&media_type)
        );

        Ok(EnhancedMediaRef {
            id: Uuid::new_v4(),
            path: filename,
            media_type,
            metadata: MediaMetadata::default(),
            status: MediaStatus::Processing,
            local_cache_path: None,
            remote_url: Some(url.to_string()),
            alt_text: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Get media file path
    pub fn get_media_path(&self, media_ref: &MediaRef) -> PathBuf {
        self.media_dir.join(&media_ref.path)
    }

    /// Get enhanced media file path
    pub fn get_enhanced_media_path(&self, media_ref: &EnhancedMediaRef) -> Option<PathBuf> {
        if let Some(cache_path) = &media_ref.local_cache_path {
            Some(PathBuf::from(cache_path))
        } else {
            Some(self.media_dir.join(&media_ref.path))
        }
    }

    /// Check if media file exists
    pub fn media_exists(&self, media_ref: &MediaRef) -> bool {
        self.get_media_path(media_ref).exists()
    }

    /// Check enhanced media status
    pub async fn check_media_status(&self, media_ref: &mut EnhancedMediaRef) -> Result<()> {
        if let Some(cache_path) = &media_ref.local_cache_path {
            let path = PathBuf::from(cache_path);
            if path.exists() {
                media_ref.status = MediaStatus::Available;
            } else {
                media_ref.status = MediaStatus::Missing;
            }
        } else if media_ref.remote_url.is_some() {
            // Would check remote availability here
            media_ref.status = MediaStatus::Processing;
        } else {
            let path = self.media_dir.join(&media_ref.path);
            if path.exists() {
                media_ref.status = MediaStatus::Available;
            } else {
                media_ref.status = MediaStatus::Missing;
            }
        }
        Ok(())
    }

    /// Delete media file
    pub fn delete_media(&self, media_ref: &MediaRef) -> Result<()> {
        let path = self.get_media_path(media_ref);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    /// Clean up orphaned media files (not referenced by any cards)
    pub async fn cleanup_orphaned_media(&self, referenced_files: &[String]) -> Result<usize> {
        self.ensure_media_dir()?;

        let mut cleaned_count = 0;

        if let Ok(entries) = fs::read_dir(&self.media_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    if !referenced_files.contains(&filename.to_string()) {
                        if fs::remove_file(&path).is_ok() {
                            cleaned_count += 1;
                        }
                    }
                }
            }
        }

        Ok(cleaned_count)
    }

    /// Extract metadata from media file
    async fn extract_metadata(
        &self,
        path: &Path,
        filename: &str,
        file_size: u64,
    ) -> Result<MediaMetadata> {
        let mime_type = self.guess_mime_type(path)?;

        // Basic metadata - in a real implementation, you'd use libraries like:
        // - `image` crate for image dimensions
        // - `symphonia` or `rodio` for audio duration
        // - `gstreamer` or `ffmpeg-next` for video processing

        let metadata = MediaMetadata {
            filename: Some(filename.to_string()),
            file_size,
            mime_type: Some(mime_type),
            duration_seconds: None, // TODO: Extract for audio/video
            dimensions: None,       // TODO: Extract for images/video
            checksum: Some(self.calculate_checksum(path)?),
            duration: None,
            created_at: Some(Utc::now()),
            tags: Some(Vec::new()),
        };

        Ok(metadata)
    }

    /// Guess MIME type from file extension
    fn guess_mime_type(&self, path: &Path) -> Result<String> {
        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

        let mime_type = match extension.to_lowercase().as_str() {
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "webp" => "image/webp",
            "svg" => "image/svg+xml",
            "mp3" => "audio/mpeg",
            "wav" => "audio/wav",
            "ogg" => "audio/ogg",
            "mp4" => "video/mp4",
            "webm" => "video/webm",
            "avi" => "video/x-msvideo",
            _ => "application/octet-stream",
        };

        Ok(mime_type.to_string())
    }

    /// Get file extension for media type
    fn get_extension_for_type(&self, media_type: &MediaType) -> &'static str {
        match media_type {
            MediaType::Audio => "mp3",
            MediaType::Image => "png",
            MediaType::Video => "mp4",
        }
    }

    /// Calculate file checksum (simple implementation)
    fn calculate_checksum(&self, path: &Path) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let contents = fs::read(path)?;
        let mut hasher = DefaultHasher::new();
        contents.hash(&mut hasher);

        Ok(format!("{:x}", hasher.finish()))
    }

    /// Validate media file
    pub async fn validate_media(&self, media_ref: &MediaRef) -> Result<bool> {
        let path = self.get_media_path(media_ref);

        if !path.exists() {
            return Ok(false);
        }

        // Check file size
        let metadata = fs::metadata(&path)?;
        if metadata.len() > self.max_file_size {
            return Ok(false);
        }

        // TODO: Add more validation based on media type
        // - Check file headers/magic numbers
        // - Try to decode the file

        Ok(true)
    }

    /// Convert legacy MediaRef to EnhancedMediaRef
    pub async fn enhance_media_ref(&self, media_ref: &MediaRef) -> Result<EnhancedMediaRef> {
        let path = self.get_media_path(media_ref);
        let status = if path.exists() {
            MediaStatus::Available
        } else {
            MediaStatus::Missing
        };

        let metadata = if path.exists() {
            let file_size = fs::metadata(&path)?.len();
            let filename = Path::new(&media_ref.path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&media_ref.path);

            Some(self.extract_metadata(&path, filename, file_size).await?)
        } else {
            None
        };

        Ok(EnhancedMediaRef {
            id: Uuid::new_v4(),
            path: media_ref.path.clone(),
            media_type: media_ref.media_type,
            metadata: metadata.unwrap_or_default(),
            status,
            local_cache_path: Some(path.to_string_lossy().to_string()),
            remote_url: None,
            alt_text: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_media_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = MediaManager::new(temp_dir.path());
        assert_eq!(manager.media_dir, temp_dir.path());
        assert_eq!(manager.max_file_size, 50 * 1024 * 1024);
    }

    #[test]
    fn test_set_max_file_size() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = MediaManager::new(temp_dir.path());
        manager.set_max_file_size(1024);
        assert_eq!(manager.max_file_size, 1024);
    }

    #[test]
    fn test_ensure_media_dir() {
        let temp_dir = TempDir::new().unwrap();
        let media_dir = temp_dir.path().join("media");
        let manager = MediaManager::new(&media_dir);

        assert!(!media_dir.exists());
        manager.ensure_media_dir().unwrap();
        assert!(media_dir.exists());
    }

    #[test]
    fn test_guess_mime_type() {
        let temp_dir = TempDir::new().unwrap();
        let manager = MediaManager::new(temp_dir.path());

        assert_eq!(
            manager.guess_mime_type(Path::new("test.jpg")).unwrap(),
            "image/jpeg"
        );
        assert_eq!(
            manager.guess_mime_type(Path::new("test.png")).unwrap(),
            "image/png"
        );
        assert_eq!(
            manager.guess_mime_type(Path::new("test.mp3")).unwrap(),
            "audio/mpeg"
        );
        assert_eq!(
            manager.guess_mime_type(Path::new("test.mp4")).unwrap(),
            "video/mp4"
        );
        assert_eq!(
            manager.guess_mime_type(Path::new("test.unknown")).unwrap(),
            "application/octet-stream"
        );
    }

    #[test]
    fn test_get_extension_for_type() {
        let temp_dir = TempDir::new().unwrap();
        let manager = MediaManager::new(temp_dir.path());

        assert_eq!(manager.get_extension_for_type(&MediaType::Audio), "mp3");
        assert_eq!(manager.get_extension_for_type(&MediaType::Image), "png");
        assert_eq!(manager.get_extension_for_type(&MediaType::Video), "mp4");
    }

    #[tokio::test]
    async fn test_add_media_file() {
        let temp_dir = TempDir::new().unwrap();
        let manager = MediaManager::new(temp_dir.path());

        // Create a test image file
        let test_file = temp_dir.path().join("test.png");
        let mut file = File::create(&test_file).unwrap();
        file.write_all(b"fake image data").unwrap();

        let media_ref = manager
            .add_media_file(&test_file, MediaType::Image)
            .await
            .unwrap();
    }

    #[test]
    fn test_media_exists() {
        let temp_dir = TempDir::new().unwrap();
        let manager = MediaManager::new(temp_dir.path());

        let media_ref = MediaRef {
            path: "test.png".to_string(),
            media_type: MediaType::Image,
        };

        assert!(!manager.media_exists(&media_ref));

        // Create the file
        let test_file = manager.get_media_path(&media_ref);
        File::create(&test_file).unwrap();

        assert!(manager.media_exists(&media_ref));
    }
}
