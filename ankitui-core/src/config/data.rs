//! Data Configuration Module
//!
//! Contains data storage, backup, sync, and database settings

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Data storage configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataConfig {
    /// Custom data directory path (None = default)
    pub data_dir: Option<PathBuf>,

    /// Enable automatic backups
    pub auto_backup: bool,

    /// Number of backups to keep
    pub backup_count: usize,

    /// Backup interval in hours
    pub backup_interval: u32,

    /// Enable data compression
    pub compress_data: bool,

    /// Database connection settings
    pub database: DatabaseConfig,

    /// Sync settings
    pub sync: SyncConfig,

    /// Backup settings
    pub backup: BackupConfig,

    /// Import/Export settings
    pub import_export: ImportExportConfig,

    /// Data integrity settings
    pub integrity: DataIntegrityConfig,

    /// Performance settings
    pub performance: DataPerformanceConfig,

    /// Security settings
    pub security: DataSecurityConfig,
}

/// Database configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Connection pool size
    pub pool_size: u32,

    /// Connection timeout in seconds
    pub timeout: u64,

    /// Enable foreign key constraints
    pub foreign_keys: bool,

    /// Enable WAL mode
    pub wal_mode: bool,

    /// Cache size in pages
    pub cache_size: u32,

    /// Page size in bytes
    pub page_size: u32,

    /// Synchronous mode
    pub synchronous: String,

    /// Journal mode
    pub journal_mode: String,

    /// Enable query logging
    pub query_logging: bool,

    /// Enable connection pooling
    pub connection_pooling: bool,

    /// Maximum number of connections
    pub max_connections: u32,

    /// Connection timeout in milliseconds
    pub connection_timeout: u64,

    /// Connection retry attempts
    pub connection_retry_attempts: u32,

    /// Connection retry delay in milliseconds
    pub connection_retry_delay: u64,

    /// Enable database vacuum
    pub enable_vacuum: bool,

    /// Vacuum interval in hours
    pub vacuum_interval: u32,

    /// Enable database analysis
    pub enable_analyze: bool,

    /// Analysis interval in hours
    pub analyze_interval: u32,

    /// Enable database optimization
    pub enable_optimization: bool,

    /// Optimization interval in hours
    pub optimization_interval: u32,
}

/// Sync configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Enable sync
    pub enabled: bool,

    /// Sync server URL
    pub server_url: Option<String>,

    /// Sync interval in minutes
    pub interval: u32,

    /// API key for sync
    pub api_key: Option<String>,

    /// Auto-sync on changes
    pub auto_sync: bool,

    /// Conflict resolution strategy
    pub conflict_resolution: String,

    /// Sync on startup
    pub sync_on_startup: bool,

    /// Sync on shutdown
    pub sync_on_shutdown: bool,

    /// Enable delta sync
    pub delta_sync: bool,

    /// Maximum sync retries
    pub max_retries: u32,

    /// Retry delay in seconds
    pub retry_delay: u32,

    /// Enable compression for sync
    pub compress_sync: bool,

    /// Enable encryption for sync
    pub encrypt_sync: bool,

    /// Sync timeout in seconds
    pub sync_timeout: u32,

    /// Batch size for sync
    pub batch_size: u32,

    /// Enable sync progress display
    pub show_progress: bool,

    /// Sync conflict settings
    pub conflict_settings: ConflictSettings,

    /// Sync authentication
    pub authentication: SyncAuthentication,
}

/// Conflict resolution settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConflictSettings {
    /// Default conflict resolution
    pub default_resolution: String,

    /// Prompt for conflicts
    pub prompt_conflicts: bool,

    /// Auto-resolve conflicts
    pub auto_resolve: bool,

    /// Keep local changes
    pub keep_local: bool,

    /// Keep remote changes
    pub keep_remote: bool,

    /// Merge changes when possible
    pub merge_changes: bool,

    /// Create conflict backup
    pub create_backup: bool,

    /// Conflict log path
    pub conflict_log_path: Option<String>,
}

/// Sync authentication settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncAuthentication {
    /// Authentication method
    pub method: String,

    /// Username for basic auth
    pub username: Option<String>,

    /// Password for basic auth
    pub password: Option<String>,

    /// Token for token-based auth
    pub token: Option<String>,

    /// Certificate path for client cert auth
    pub certificate_path: Option<String>,

    /// Private key path for client cert auth
    pub private_key_path: Option<String>,

    /// CA certificate path
    pub ca_certificate_path: Option<String>,

    /// Verify SSL certificate
    pub verify_certificate: bool,

    /// Custom headers
    pub custom_headers: std::collections::HashMap<String, String>,
}

/// Backup configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Enable automatic backups
    pub auto_backup: bool,

    /// Backup directory
    pub backup_dir: Option<String>,

    /// Number of backups to keep
    pub backup_count: usize,

    /// Backup interval in hours
    pub backup_interval: u32,

    /// Backup format
    pub backup_format: String,

    /// Compress backups
    pub compress_backups: bool,

    /// Encrypt backups
    pub encrypt_backups: bool,

    /// Include media files
    pub include_media: bool,

    /// Include settings
    pub include_settings: bool,

    /// Include statistics
    pub include_statistics: bool,

    /// Backup on exit
    pub backup_on_exit: bool,

    /// Backup on startup
    pub backup_on_startup: bool,

    /// Create full backups
    pub full_backups: bool,

    /// Full backup interval in days
    pub full_backup_interval: u32,

    /// Incremental backup interval in hours
    pub incremental_backup_interval: u32,

    /// Backup retention policy
    pub retention_policy: RetentionPolicy,

    /// Backup verification
    pub verification: BackupVerification,
}

/// Backup retention policy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Keep daily backups for N days
    pub keep_daily: u32,

    /// Keep weekly backups for N weeks
    pub keep_weekly: u32,

    /// Keep monthly backups for N months
    pub keep_monthly: u32,

    /// Keep yearly backups for N years
    pub keep_yearly: u32,

    /// Maximum total backups
    pub max_total: usize,

    /// Minimum free space required (MB)
    pub min_free_space: u64,

    /// Delete old backups when space is low
    pub delete_when_low: bool,

    /// Alert when backup count exceeds threshold
    pub alert_threshold: usize,
}

/// Backup verification settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackupVerification {
    /// Enable backup verification
    pub enabled: bool,

    /// Verify backup integrity
    pub verify_integrity: bool,

    /// Verify backup can be restored
    pub verify_restore: bool,

    /// Verification schedule
    pub verification_schedule: String,

    /// Alert on verification failure
    pub alert_on_failure: bool,

    /// Automatic repair if possible
    pub auto_repair: bool,
}

/// Import/Export configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportExportConfig {
    /// Default export format
    pub default_export_format: String,

    /// Default import format
    pub default_import_format: String,

    /// Include media files in export
    pub include_media: bool,

    /// Include scheduling information in export
    pub include_scheduling: bool,

    /// Include statistics in export
    pub include_statistics: bool,

    /// Include tags in export
    pub include_tags: bool,

    /// Import duplicates handling
    pub import_duplicates: String,

    /// Import conflict resolution
    pub import_conflict_resolution: String,

    /// Validate imports
    pub validate_imports: bool,

    /// Show import preview
    pub show_import_preview: bool,

    /// Export compression
    pub export_compression: bool,

    /// Export encryption
    pub export_encryption: bool,

    /// Supported formats
    pub supported_formats: Vec<String>,

    /// Custom format settings
    pub custom_formats: std::collections::HashMap<String, CustomFormat>,
}

/// Custom format configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomFormat {
    /// Format name
    pub name: String,

    /// File extension
    pub extension: String,

    /// MIME type
    pub mime_type: String,

    /// Export command
    pub export_command: Option<String>,

    /// Import command
    pub import_command: Option<String>,

    /// Format options
    pub options: std::collections::HashMap<String, serde_json::Value>,
}

/// Data integrity configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataIntegrityConfig {
    /// Enable data integrity checks
    pub enabled: bool,

    /// Check frequency in hours
    pub check_frequency: u32,

    /// Check on startup
    pub check_on_startup: bool,

    /// Check on shutdown
    pub check_on_shutdown: bool,

    /// Automatic repair
    pub auto_repair: bool,

    /// Create repair backups
    pub repair_backups: bool,

    /// Log integrity issues
    pub log_issues: bool,

    /// Alert on integrity issues
    pub alert_issues: bool,

    /// Verify checksums
    pub verify_checksums: bool,

    /// Deep scan mode
    pub deep_scan: bool,

    /// Check database schema
    pub check_schema: bool,

    /// Check data consistency
    pub check_consistency: bool,

    /// Check references
    pub check_references: bool,
}

/// Data performance configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataPerformanceConfig {
    /// Enable data caching
    pub enable_caching: bool,

    /// Cache size in MB
    pub cache_size: u64,

    /// Cache TTL in seconds
    pub cache_ttl: u32,

    /// Preload data
    pub preload_data: bool,

    /// Preload size in MB
    pub preload_size: u64,

    /// Enable lazy loading
    pub lazy_loading: bool,

    /// Batch size for operations
    pub batch_size: u32,

    /// Connection pooling
    pub connection_pooling: bool,

    /// Query optimization
    pub query_optimization: bool,

    /// Index optimization
    pub index_optimization: bool,

    /// Memory management
    pub memory_management: MemoryManagement,

    /// Disk I/O optimization
    pub disk_io: DiskIOOptimization,
}

/// Memory management settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryManagement {
    /// Maximum memory usage in MB
    pub max_memory: u64,

    /// Memory cleanup threshold in percentage
    pub cleanup_threshold: u8,

    /// Garbage collection frequency
    pub gc_frequency: u32,

    /// Enable memory profiling
    pub enable_profiling: bool,

    /// Memory leak detection
    pub leak_detection: bool,
}

/// Disk I/O optimization settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiskIOOptimization {
    /// Enable read buffering
    pub read_buffering: bool,

    /// Buffer size in KB
    pub buffer_size: u32,

    /// Enable write buffering
    pub write_buffering: bool,

    /// Sync mode
    pub sync_mode: String,

    /// Enable async I/O
    pub async_io: bool,

    /// I/O priority
    pub io_priority: u8,
}

/// Data security configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSecurityConfig {
    /// Enable data encryption
    pub encryption_enabled: bool,

    /// Encryption algorithm
    pub encryption_algorithm: String,

    /// Key derivation function
    pub key_derivation: String,

    /// Enable data masking
    pub data_masking: bool,

    /// Access control
    pub access_control: AccessControl,

    /// Audit logging
    pub audit_logging: AuditLogging,

    /// Data sanitization
    pub data_sanitization: DataSanitization,
}

/// Access control settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessControl {
    /// Enable access control
    pub enabled: bool,

    /// Read-only mode
    pub read_only: bool,

    /// Allowed users
    pub allowed_users: Vec<String>,

    /// Allowed IP addresses
    pub allowed_ips: Vec<String>,

    /// Session timeout in minutes
    pub session_timeout: u32,

    /// Maximum login attempts
    pub max_login_attempts: u32,

    /// Lockout duration in minutes
    pub lockout_duration: u32,
}

/// Audit logging settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditLogging {
    /// Enable audit logging
    pub enabled: bool,

    /// Log file path
    pub log_file: Option<String>,

    /// Log level
    pub log_level: String,

    /// Log rotation
    pub log_rotation: bool,

    /// Maximum log size in MB
    pub max_log_size: u64,

    /// Keep logs for N days
    pub keep_logs_days: u32,

    /// Log all data access
    pub log_data_access: bool,

    /// Log configuration changes
    pub log_config_changes: bool,

    /// Log security events
    pub log_security_events: bool,
}

/// Data sanitization settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSanitization {
    /// Enable data sanitization
    pub enabled: bool,

    /// Sanitize on export
    pub sanitize_on_export: bool,

    /// Sanitize on backup
    pub sanitize_on_backup: bool,

    /// Fields to sanitize
    pub fields_to_sanitize: Vec<String>,

    /// Sanitization method
    pub sanitization_method: String,

    /// Preserve structure
    pub preserve_structure: bool,
}

impl Default for DataConfig {
    fn default() -> Self {
        Self {
            data_dir: None,
            auto_backup: true,
            backup_count: 10,
            backup_interval: 24,
            compress_data: false,
            database: DatabaseConfig::default(),
            sync: SyncConfig::default(),
            backup: BackupConfig::default(),
            import_export: ImportExportConfig::default(),
            integrity: DataIntegrityConfig::default(),
            performance: DataPerformanceConfig::default(),
            security: DataSecurityConfig::default(),
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            pool_size: 10,
            timeout: 30,
            foreign_keys: true,
            wal_mode: true,
            cache_size: 2000,
            page_size: 4096,
            synchronous: "NORMAL".to_string(),
            journal_mode: "WAL".to_string(),
            query_logging: false,
            connection_pooling: true,
            max_connections: 20,
            connection_timeout: 30000,
            connection_retry_attempts: 3,
            connection_retry_delay: 1000,
            enable_vacuum: true,
            vacuum_interval: 168, // 7 days
            enable_analyze: true,
            analyze_interval: 24,
            enable_optimization: true,
            optimization_interval: 168,
        }
    }
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            server_url: None,
            interval: 15,
            api_key: None,
            auto_sync: true,
            conflict_resolution: "prompt".to_string(),
            sync_on_startup: false,
            sync_on_shutdown: false,
            delta_sync: true,
            max_retries: 3,
            retry_delay: 30,
            compress_sync: true,
            encrypt_sync: false,
            sync_timeout: 300,
            batch_size: 100,
            show_progress: true,
            conflict_settings: ConflictSettings::default(),
            authentication: SyncAuthentication::default(),
        }
    }
}

impl Default for ConflictSettings {
    fn default() -> Self {
        Self {
            default_resolution: "prompt".to_string(),
            prompt_conflicts: true,
            auto_resolve: false,
            keep_local: false,
            keep_remote: false,
            merge_changes: true,
            create_backup: true,
            conflict_log_path: None,
        }
    }
}

impl Default for SyncAuthentication {
    fn default() -> Self {
        Self {
            method: "none".to_string(),
            username: None,
            password: None,
            token: None,
            certificate_path: None,
            private_key_path: None,
            ca_certificate_path: None,
            verify_certificate: true,
            custom_headers: std::collections::HashMap::new(),
        }
    }
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            auto_backup: true,
            backup_dir: None,
            backup_count: 10,
            backup_interval: 24,
            backup_format: "zip".to_string(),
            compress_backups: true,
            encrypt_backups: false,
            include_media: true,
            include_settings: true,
            include_statistics: false,
            backup_on_exit: false,
            backup_on_startup: false,
            full_backups: false,
            full_backup_interval: 7,
            incremental_backup_interval: 24,
            retention_policy: RetentionPolicy::default(),
            verification: BackupVerification::default(),
        }
    }
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            keep_daily: 7,
            keep_weekly: 4,
            keep_monthly: 12,
            keep_yearly: 5,
            max_total: 100,
            min_free_space: 1000, // 1GB
            delete_when_low: true,
            alert_threshold: 90,
        }
    }
}

impl Default for BackupVerification {
    fn default() -> Self {
        Self {
            enabled: true,
            verify_integrity: true,
            verify_restore: false,
            verification_schedule: "weekly".to_string(),
            alert_on_failure: true,
            auto_repair: false,
        }
    }
}

impl Default for ImportExportConfig {
    fn default() -> Self {
        Self {
            default_export_format: "anki".to_string(),
            default_import_format: "anki".to_string(),
            include_media: true,
            include_scheduling: true,
            include_statistics: false,
            include_tags: true,
            import_duplicates: "skip".to_string(),
            import_conflict_resolution: "skip".to_string(),
            validate_imports: true,
            show_import_preview: true,
            export_compression: true,
            export_encryption: false,
            supported_formats: vec![
                "anki".to_string(),
                "csv".to_string(),
                "tsv".to_string(),
                "json".to_string(),
                "toml".to_string(),
            ],
            custom_formats: std::collections::HashMap::new(),
        }
    }
}

impl Default for DataIntegrityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_frequency: 168, // 7 days
            check_on_startup: false,
            check_on_shutdown: false,
            auto_repair: false,
            repair_backups: true,
            log_issues: true,
            alert_issues: true,
            verify_checksums: true,
            deep_scan: false,
            check_schema: true,
            check_consistency: true,
            check_references: true,
        }
    }
}

impl Default for DataPerformanceConfig {
    fn default() -> Self {
        Self {
            enable_caching: true,
            cache_size: 100,
            cache_ttl: 3600,
            preload_data: false,
            preload_size: 50,
            lazy_loading: true,
            batch_size: 1000,
            connection_pooling: true,
            query_optimization: true,
            index_optimization: true,
            memory_management: MemoryManagement::default(),
            disk_io: DiskIOOptimization::default(),
        }
    }
}

impl Default for MemoryManagement {
    fn default() -> Self {
        Self {
            max_memory: 512, // 512MB
            cleanup_threshold: 80,
            gc_frequency: 60,
            enable_profiling: false,
            leak_detection: false,
        }
    }
}

impl Default for DiskIOOptimization {
    fn default() -> Self {
        Self {
            read_buffering: true,
            buffer_size: 64,
            write_buffering: true,
            sync_mode: "NORMAL".to_string(),
            async_io: true,
            io_priority: 5,
        }
    }
}

impl Default for DataSecurityConfig {
    fn default() -> Self {
        Self {
            encryption_enabled: false,
            encryption_algorithm: "AES-256-GCM".to_string(),
            key_derivation: "PBKDF2".to_string(),
            data_masking: false,
            access_control: AccessControl::default(),
            audit_logging: AuditLogging::default(),
            data_sanitization: DataSanitization::default(),
        }
    }
}

impl Default for AccessControl {
    fn default() -> Self {
        Self {
            enabled: false,
            read_only: false,
            allowed_users: Vec::new(),
            allowed_ips: Vec::new(),
            session_timeout: 60,
            max_login_attempts: 3,
            lockout_duration: 15,
        }
    }
}

impl Default for AuditLogging {
    fn default() -> Self {
        Self {
            enabled: false,
            log_file: None,
            log_level: "INFO".to_string(),
            log_rotation: true,
            max_log_size: 10, // 10MB
            keep_logs_days: 30,
            log_data_access: false,
            log_config_changes: true,
            log_security_events: true,
        }
    }
}

impl Default for DataSanitization {
    fn default() -> Self {
        Self {
            enabled: false,
            sanitize_on_export: false,
            sanitize_on_backup: false,
            fields_to_sanitize: Vec::new(),
            sanitization_method: "mask".to_string(),
            preserve_structure: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_config_default() {
        let config = DataConfig::default();
        assert!(config.auto_backup);
        assert_eq!(config.backup_count, 10);
        assert_eq!(config.backup_interval, 24);
        assert!(!config.compress_data);
    }

    #[test]
    fn test_database_config() {
        let db_config = DatabaseConfig::default();
        assert_eq!(db_config.pool_size, 10);
        assert_eq!(db_config.timeout, 30);
        assert!(db_config.foreign_keys);
        assert!(db_config.wal_mode);
        assert_eq!(db_config.cache_size, 2000);
    }

    #[test]
    fn test_sync_config() {
        let sync_config = SyncConfig::default();
        assert!(!sync_config.enabled);
        assert_eq!(sync_config.interval, 15);
        assert!(sync_config.auto_sync);
        assert_eq!(sync_config.conflict_resolution, "prompt");
    }

    #[test]
    fn test_backup_config() {
        let backup_config = BackupConfig::default();
        assert!(backup_config.auto_backup);
        assert_eq!(backup_config.backup_count, 10);
        assert_eq!(backup_config.backup_format, "zip");
        assert!(backup_config.compress_backups);
    }

    #[test]
    fn test_import_export_config() {
        let config = ImportExportConfig::default();
        assert_eq!(config.default_export_format, "anki");
        assert_eq!(config.default_import_format, "anki");
        assert!(config.include_media);
        assert!(config.include_scheduling);
        assert!(config.validate_imports);
    }

    #[test]
    fn test_data_integrity_config() {
        let config = DataIntegrityConfig::default();
        assert!(config.enabled);
        assert_eq!(config.check_frequency, 168);
        assert!(!config.auto_repair);
        assert!(config.repair_backups);
    }
}
