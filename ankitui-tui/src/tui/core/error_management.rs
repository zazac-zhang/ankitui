//! 统一错误管理系统
//!
//! 整合所有错误处理功能，包括：
//! - 错误类型定义
//! - 错误处理和记录
//! - 错误恢复策略
//! - 统一错误处理接口
//!
//! 遵循"一个功能一个文件"原则，统一管理所有错误相关逻辑

use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, VecDeque};
use std::time::Duration;

/// 主要的TUI错误类型 - 整合所有错误定义
#[derive(Debug, Clone, PartialEq)]
pub enum TUIError {
    // UI相关错误
    RenderError {
        component: String,
        operation: String,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    InputError {
        input_type: String,
        expected_format: String,
        received_value: String,
        timestamp: DateTime<Utc>,
    },
    ComponentError {
        component_name: String,
        operation: String,
        reason: String,
        timestamp: DateTime<Utc>,
    },

    // 数据相关错误
    DataLoadError {
        resource_type: String,
        resource_id: Option<String>,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    DataSaveError {
        resource_type: String,
        resource_id: Option<String>,
        reason: String,
        timestamp: DateTime<Utc>,
    },

    // 操作错误
    OperationError {
        operation: String,
        context: String,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    ValidationError {
        field: String,
        value: String,
        validation_rule: String,
        timestamp: DateTime<Utc>,
    },

    // 系统错误
    SystemError {
        component: String,
        operation: String,
        reason: String,
        timestamp: DateTime<Utc>,
        error_code: Option<u32>,
    },
}

/// 错误严重性级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl std::fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorSeverity::Info => write!(f, "INFO"),
            ErrorSeverity::Warning => write!(f, "WARNING"),
            ErrorSeverity::Error => write!(f, "ERROR"),
            ErrorSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// 恢复动作
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryAction {
    Retry,
    Refresh,
    SaveAs,
    CheckConnection,
    CheckPermissions,
    CorrectInput(String),
    ResetField(String),
    ShowHelp(String),
    ShowExamples,
    ShowDetails,
    Cancel,
}

/// 恢复结果
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryResult {
    RetryRequested,
    RefreshRequested,
    SaveAsDialog,
    ConnectionCheck,
    PermissionCheck,
    InputCorrection(String),
    FieldReset(String),
    HelpDialog(String),
    ExamplesDialog,
    ErrorDetails(TUIError),
    CancelOperation,
    NoAction,
}

/// 用户消息
#[derive(Debug, Clone)]
pub struct UserMessage {
    pub level: UserMessageLevel,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub auto_dismiss: Option<Duration>,
}

/// 用户消息级别
#[derive(Debug, Clone, PartialEq)]
pub enum UserMessageLevel {
    Info,
    Success,
    Warning,
    Error,
}

/// 错误记录
#[derive(Debug, Clone)]
pub struct ErrorRecord {
    pub error: TUIError,
    pub context: String,
    pub recovery_attempted: bool,
    pub recovery_result: Option<RecoveryResult>,
}

/// 错误管理器 - 处理错误记录和消息管理
#[derive(Debug, Clone)]
pub struct ErrorManager {
    history: VecDeque<ErrorRecord>,
    active_messages: Vec<UserMessage>,
    statistics: ErrorStatistics,
    max_history_size: usize,
    max_active_messages: usize,
}

/// 错误统计
#[derive(Debug, Clone, Default)]
pub struct ErrorStatistics {
    pub total_count: usize,
    pub critical_count: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub most_recent_error: Option<DateTime<Utc>>,
    pub recoverable_count: usize,
    pub non_recoverable_count: usize,
}

/// 恢复管理器 - 处理错误恢复策略
#[derive(Debug, Clone)]
pub struct RecoveryManager {
    auto_recovery: bool,
    max_retries: u32,
    retry_delay: Duration,
    recovery_strategies: HashMap<String, RecoveryStrategy>,
}

/// 恢复策略
#[derive(Debug, Clone)]
pub struct RecoveryStrategy {
    pub can_auto_recover: bool,
    pub max_attempts: u32,
    pub actions: Vec<RecoveryAction>,
}

/// 统一错误管理系统 - 主要接口
#[derive(Debug, Clone)]
pub struct ErrorManagement {
    pub error_manager: ErrorManager,
    pub recovery_manager: RecoveryManager,
    config: ErrorConfig,
}

/// 错误处理配置
#[derive(Debug, Clone)]
pub struct ErrorConfig {
    pub max_history_size: usize,
    pub max_active_messages: usize,
    pub auto_recovery: bool,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
}

impl Default for ErrorConfig {
    fn default() -> Self {
        Self {
            max_history_size: 100,
            max_active_messages: 10,
            auto_recovery: true,
            max_retries: 3,
            retry_delay_ms: 1000,
        }
    }
}

impl Default for ErrorManager {
    fn default() -> Self {
        Self {
            history: VecDeque::new(),
            active_messages: Vec::new(),
            statistics: ErrorStatistics::default(),
            max_history_size: 100,
            max_active_messages: 10,
        }
    }
}

impl Default for RecoveryManager {
    fn default() -> Self {
        Self {
            auto_recovery: true,
            max_retries: 3,
            retry_delay: Duration::from_millis(1000),
            recovery_strategies: HashMap::new(),
        }
    }
}

impl Default for ErrorManagement {
    fn default() -> Self {
        Self {
            error_manager: ErrorManager::default(),
            recovery_manager: RecoveryManager::default(),
            config: ErrorConfig::default(),
        }
    }
}
