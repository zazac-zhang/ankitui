//! Media Management Module
//!
//! Handles media file operations, validation, and metadata extraction

use crate::data::models::{EnhancedMediaRef, MediaMetadata, MediaRef, MediaStatus, MediaType};
use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Media dimensions for images and video
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaDimensions {
    pub width: u32,
    pub height: u32,
}

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

        // Generate unique filename
        let filename = format!(
            "{}.{}",
            Uuid::new_v4(),
            self.get_extension_for_type(&media_type)
        );

        let local_path = self.media_dir.join(&filename);

        // Download the file
        let response = reqwest::get(url).await
            .context("Failed to fetch URL")?;

        if !response.status().is_success() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let bytes = response.bytes().await
            .context("Failed to read response body")?;

        // Write to local file
        tokio::fs::write(&local_path, bytes).await
            .context("Failed to write media file")?;

        log::info!("Downloaded media from {} to {:?}", url, local_path);

        Ok(EnhancedMediaRef {
            id: Uuid::new_v4(),
            path: filename,
            media_type,
            metadata: MediaMetadata::default(),
            status: MediaStatus::Available,
            local_cache_path: Some(local_path.to_string_lossy().to_string()),
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

        // Extract dimensions for images
        let dimensions = if mime_type.starts_with("image/") {
            self.extract_image_dimensions(path).await.ok()
        } else {
            None
        };

        // Extract duration for audio/video
        // Note: This would require additional libraries like symphonia or gstreamer
        let duration_seconds = if mime_type.starts_with("audio/") || mime_type.starts_with("video/") {
            None // TODO: Implement audio/video duration extraction
        } else {
            None
        };

        let metadata = MediaMetadata {
            filename: Some(filename.to_string()),
            file_size,
            mime_type: Some(mime_type),
            duration_seconds,
            dimensions: dimensions.map(|d| (d.width, d.height)),
            checksum: Some(self.calculate_checksum(path)?),
            duration: None,
            created_at: Some(Utc::now()),
            tags: Some(Vec::new()),
        };

        Ok(metadata)
    }

    /// Extract image dimensions using the image crate
    async fn extract_image_dimensions(&self, path: &Path) -> Result<MediaDimensions> {
        // Use tokio::task::spawn_blocking to run blocking I/O off the async runtime
        let path_clone = path.to_path_buf();
        let dimensions = tokio::task::spawn_blocking(move || {
            let img = image::open(&path_clone)?;
            Ok::<_, anyhow::Error>((img.width(), img.height()))
        }).await
        .context("Failed to join blocking task")?
        .context("Failed to open image")?;

        Ok(MediaDimensions {
            width: dimensions.0,
            height: dimensions.1,
        })
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

