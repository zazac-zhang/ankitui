//! Application State Management
//!
//! 专门管理应用级别的状态，包括服务状态、配置状态、运行时状态等
//! 遵循"一个state只管理一类型state"原则

use crate::tui::core::error_management::{ErrorManagement, ErrorSeverity, TUIError, UserMessage};
use ankitui_core::ConfigManager;
use ankitui_core::{DeckManager, SessionController, StatsEngine};
use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// 应用状态 - 专门管理应用级别的状态
#[derive(Debug, Clone)]
pub struct ApplicationState {
    /// UI状态管理
    pub ui_state: UIAppState,
    /// 服务状态
    pub service_state: ServiceState,
    /// 运行时状态
    pub runtime_state: RuntimeState,
    /// 错误处理状态
    pub error_state: ErrorState,
}

/// UI应用状态 - UI相关的应用级别状态
#[derive(Debug, Clone)]
pub struct UIAppState {
    /// 当前焦点区域
    pub focused_area: String,
    /// 主题设置
    pub theme: String,
    /// 界面语言
    pub language: String,
    /// 界面缩放
    pub scale_factor: f32,
    /// 是否显示帮助信息
    pub show_help: bool,
}

/// 服务状态 - 管理各种核心服务的状态
#[derive(Debug, Clone)]
pub struct ServiceState {
    /// 牌组管理器状态
    pub deck_manager_state: DeckManagerState,
    /// 会话控制器状态
    pub session_controller_state: Option<SessionControllerState>,
    /// 统计引擎状态
    pub stats_engine_state: StatsEngineState,
}

/// 牌组管理器状态
#[derive(Debug, Clone)]
pub struct DeckManagerState {
    /// 是否已初始化
    pub initialized: bool,
    /// 最后加载时间
    pub last_load_time: Option<DateTime<Utc>>,
    /// 当前加载的牌组数量
    pub deck_count: usize,
    /// 错误状态
    pub error: Option<String>,
}

/// 会话控制器状态
#[derive(Debug, Clone)]
pub struct SessionControllerState {
    /// 是否活跃
    pub active: bool,
    /// 会话开始时间
    pub start_time: DateTime<Utc>,
    /// 当前牌组ID
    pub current_deck_id: Option<String>,
    /// 学习进度
    pub cards_studied: usize,
    /// 错误状态
    pub error: Option<String>,
}

/// 统计引擎状态
#[derive(Debug, Clone)]
pub struct StatsEngineState {
    /// 是否已初始化
    pub initialized: bool,
    /// 最后计算时间
    pub last_calculation: Option<DateTime<Utc>>,
    /// 缓存的数据
    pub cached_data: Option<HashMap<String, String>>,
    /// 错误状态
    pub error: Option<String>,
}

/// 运行时状态 - 管理应用运行时的状态
#[derive(Debug, Clone)]
pub struct RuntimeState {
    /// 应用启动时间
    pub start_time: DateTime<Utc>,
    /// 当前用户操作
    pub current_action: Option<String>,
    /// 性能监控
    pub performance: PerformanceState,
    /// 用户会话信息
    pub session: UserSessionState,
}

/// 性能状态
#[derive(Debug, Clone)]
pub struct PerformanceState {
    /// 最后渲染时间
    pub last_render_time: Option<DateTime<Utc>>,
    /// 渲染次数
    pub render_count: usize,
    /// 平均渲染时间（毫秒）
    pub avg_render_time: f32,
    /// 内存使用情况
    pub memory_usage: Option<usize>,
}

/// 用户会话状态
#[derive(Debug, Clone)]
pub struct UserSessionState {
    /// 用户ID
    pub user_id: Option<String>,
    /// 会话ID
    pub session_id: String,
    /// 最后活动时间
    pub last_activity: DateTime<Utc>,
    /// 会话持续时间（秒）
    pub duration: u64,
}

/// 错误状态 - 管理错误相关的状态
#[derive(Debug, Clone)]
pub struct ErrorState {
    /// 错误管理器
    pub error_management: ErrorManagement,
    /// 最后错误时间
    pub last_error_time: Option<DateTime<Utc>>,
    /// 错误计数
    pub error_count: usize,
    /// 警告计数
    pub warning_count: usize,
}

/// 应用状态管理器 - 专门管理应用级别的状态
pub struct ApplicationStateManager {
    /// 应用状态
    pub state: ApplicationState,
    /// 配置管理器
    pub config_manager: ConfigManager,
    /// 核心服务
    pub core_services: CoreServices,
}

/// 核心服务容器
#[derive(Debug, Clone)]
pub struct CoreServices {
    pub deck_manager: DeckManager,
    pub session_controller: Option<SessionController>,
    pub stats_engine: StatsEngine,
}

impl ApplicationStateManager {
    /// 创建新的应用状态管理器
    pub async fn new(config_manager: ConfigManager) -> Result<Self> {
        let now = Utc::now();

        let state = ApplicationState {
            ui_state: UIAppState {
                focused_area: "main".to_string(),
                theme: "default".to_string(),
                language: "en".to_string(),
                scale_factor: 1.0,
                show_help: false,
            },
            service_state: ServiceState {
                deck_manager_state: DeckManagerState {
                    initialized: false,
                    last_load_time: None,
                    deck_count: 0,
                    error: None,
                },
                session_controller_state: None,
                stats_engine_state: StatsEngineState {
                    initialized: false,
                    last_calculation: None,
                    cached_data: None,
                    error: None,
                },
            },
            runtime_state: RuntimeState {
                start_time: now,
                current_action: None,
                performance: PerformanceState {
                    last_render_time: None,
                    render_count: 0,
                    avg_render_time: 0.0,
                    memory_usage: None,
                },
                session: UserSessionState {
                    user_id: None,
                    session_id: uuid::Uuid::new_v4().to_string(),
                    last_activity: now,
                    duration: 0,
                },
            },
            error_state: ErrorState {
                error_management: ErrorManagement::default(),
                last_error_time: None,
                error_count: 0,
                warning_count: 0,
            },
        };

        // 初始化核心服务
        let data_dir = config_manager.get_data_dir();
        let db_path = data_dir.join("ankitui.db");
        let deck_manager = DeckManager::new(data_dir, db_path).await?;
        let stats_engine = StatsEngine::new();

        let core_services = CoreServices {
            deck_manager,
            session_controller: None,
            stats_engine,
        };

        Ok(Self {
            state,
            config_manager,
            core_services,
        })
    }

    /// 初始化服务状态
    pub async fn initialize_services(&mut self) -> Result<()> {
        // 初始化牌组管理器
        self.state.service_state.deck_manager_state.initialized = true;
        self.state.service_state.deck_manager_state.last_load_time = Some(Utc::now());
        self.state.service_state.deck_manager_state.deck_count =
            self.core_services.deck_manager.get_deck_count().await?;

        // 初始化统计引擎
        self.state.service_state.stats_engine_state.initialized = true;
        self.state.service_state.stats_engine_state.last_calculation = Some(Utc::now());

        Ok(())
    }

    /// 开始学习会话
    pub async fn start_learning_session(&mut self, deck_id: String) -> Result<()> {
        let session_controller =
            SessionController::new(self.core_services.deck_manager.clone(), None).await?;

        let now = Utc::now();
        self.state.service_state.session_controller_state = Some(SessionControllerState {
            active: true,
            start_time: now,
            current_deck_id: Some(deck_id),
            cards_studied: 0,
            error: None,
        });

        self.core_services.session_controller = Some(session_controller);
        self.state.runtime_state.current_action = Some("start_learning_session".to_string());

        Ok(())
    }

    /// 结束学习会话
    pub fn end_learning_session(&mut self) -> Result<()> {
        if let Some(ref mut session_state) = self.state.service_state.session_controller_state {
            session_state.active = false;
        }

        self.core_services.session_controller = None;
        self.state.runtime_state.current_action = Some("end_learning_session".to_string());

        Ok(())
    }

    /// 更新性能状态
    pub fn update_performance(&mut self, render_time_ms: f32) {
        let perf_state = &mut self.state.runtime_state.performance;
        perf_state.last_render_time = Some(Utc::now());
        perf_state.render_count += 1;

        // 计算平均渲染时间
        let total_time = perf_state.avg_render_time * (perf_state.render_count - 1) as f32;
        perf_state.avg_render_time = (total_time + render_time_ms) / perf_state.render_count as f32;
    }

    /// 更新用户活动
    pub fn update_user_activity(&mut self) {
        let now = Utc::now();
        self.state.runtime_state.session.last_activity = now;
        self.state.runtime_state.session.duration =
            (now - self.state.runtime_state.start_time).num_seconds() as u64;
    }

    /// 处理错误
    pub fn handle_error(&mut self, error: String, severity: ErrorSeverity) {
        // For now, just log the error and update counters
        // TODO: Implement proper error handling with ErrorManagement

        self.state.error_state.last_error_time = Some(Utc::now());

        match severity {
            ErrorSeverity::Error => self.state.error_state.error_count += 1,
            ErrorSeverity::Warning => self.state.error_state.warning_count += 1,
            _ => {}
        }

        // Handle errors without console output in TUI mode
        // Store error for UI display instead of logging to console
    }

    /// 获取应用状态快照
    pub fn get_state_snapshot(&self) -> ApplicationStateSnapshot {
        ApplicationStateSnapshot {
            ui_focused_area: self.state.ui_state.focused_area.clone(),
            deck_count: self.state.service_state.deck_manager_state.deck_count,
            session_active: self
                .state
                .service_state
                .session_controller_state
                .as_ref()
                .map(|s| s.active)
                .unwrap_or(false),
            render_count: self.state.runtime_state.performance.render_count,
            error_count: self.state.error_state.error_count,
            uptime_seconds: (Utc::now() - self.state.runtime_state.start_time).num_seconds(),
        }
    }

    /// 重置应用状态
    pub async fn reset(&mut self) -> Result<()> {
        let config_manager = self.config_manager.clone();
        *self = Self::new(config_manager).await?;
        Ok(())
    }
}

/// 应用状态快照 - 用于监控和调试
#[derive(Debug, Clone)]
pub struct ApplicationStateSnapshot {
    pub ui_focused_area: String,
    pub deck_count: usize,
    pub session_active: bool,
    pub render_count: usize,
    pub error_count: usize,
    pub uptime_seconds: i64,
}

impl Default for ApplicationState {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            ui_state: UIAppState {
                focused_area: "main".to_string(),
                theme: "default".to_string(),
                language: "en".to_string(),
                scale_factor: 1.0,
                show_help: false,
            },
            service_state: ServiceState {
                deck_manager_state: DeckManagerState {
                    initialized: false,
                    last_load_time: None,
                    deck_count: 0,
                    error: None,
                },
                session_controller_state: None,
                stats_engine_state: StatsEngineState {
                    initialized: false,
                    last_calculation: None,
                    cached_data: None,
                    error: None,
                },
            },
            runtime_state: RuntimeState {
                start_time: now,
                current_action: None,
                performance: PerformanceState {
                    last_render_time: None,
                    render_count: 0,
                    avg_render_time: 0.0,
                    memory_usage: None,
                },
                session: UserSessionState {
                    user_id: None,
                    session_id: uuid::Uuid::new_v4().to_string(),
                    last_activity: now,
                    duration: 0,
                },
            },
            error_state: ErrorState {
                error_management: ErrorManagement::default(),
                last_error_time: None,
                error_count: 0,
                warning_count: 0,
            },
        }
    }
}
