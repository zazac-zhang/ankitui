//! UI State Management
//!
//! 专门管理UI相关的状态，包括菜单状态、选择状态、界面交互状态等
//! 遵循"一个state只管理一类型state"原则

use ankitui_core::Deck;
use ankitui_core::{DeckStatistics, SessionProgress};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

/// UI状态枚举 - 仅管理UI相关的状态
#[derive(Debug, Clone)]
pub enum UIState {
    MainMenu(MainMenuState),
    DeckSelection(DeckSelectionState),
    DeckManagement(DeckManagementState),
    CardReview(CardReviewState),
    Statistics(StatisticsState),
    Settings(SettingsState),
    Help(HelpState),
    ConfirmExit(ConfirmExitState),
}

/// 主菜单状态
#[derive(Debug, Clone, PartialEq)]
pub struct MainMenuState {
    pub selected_item: usize,
    pub menu_items: Vec<MenuItem>,
}

/// 菜单项
#[derive(Debug, Clone, PartialEq)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub enabled: bool,
}

/// 牌组选择状态
#[derive(Debug, Clone)]
pub struct DeckSelectionState {
    pub available_decks: Vec<Deck>,
    pub selected_deck_index: usize,
    pub deck_filter: Option<String>,
    pub sort_by: DeckSortBy,
}

/// 牌组排序方式
#[derive(Debug, Clone, PartialEq)]
pub enum DeckSortBy {
    Name,
    CreatedDate,
    CardCount,
    DueCount,
}

/// 牌组管理状态
#[derive(Debug, Clone)]
pub struct DeckManagementState {
    pub available_decks: Vec<Deck>,
    pub selected_deck_index: usize,
    pub selected_action: DeckManagementAction,
    pub deck_details: Option<(Deck, DeckStatistics)>,
    pub modal_state: Option<DeckModalState>,
    pub search_query: String,
    pub filtered_decks: Vec<usize>,
}

/// 牌组管理操作
#[derive(Debug, Clone, PartialEq)]
pub enum DeckManagementAction {
    Create,
    Delete,
    Rename,
    Info,
    Import,
    Export,
}

/// 牌组模态框状态
#[derive(Debug, Clone, PartialEq)]
pub enum DeckModalState {
    CreateDeck(CreateDeckState),
    RenameDeck(RenameDeckState),
    DeleteConfirm(DeleteConfirmState),
    Import(ImportState),
    Export(ExportState),
}

/// 创建牌组状态
#[derive(Debug, Clone, PartialEq)]
pub struct CreateDeckState {
    pub deck_name: String,
    pub description: String,
    pub cursor_position: usize,
    pub validation_error: Option<String>,
}

/// 重命名牌组状态
#[derive(Debug, Clone, PartialEq)]
pub struct RenameDeckState {
    pub deck_id: Uuid,
    pub old_name: String,
    pub new_name: String,
    pub cursor_position: usize,
    pub validation_error: Option<String>,
}

/// 删除确认状态
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteConfirmState {
    pub deck_id: Uuid,
    pub deck_name: String,
    pub card_count: usize,
}

/// 导入状态
#[derive(Debug, Clone, PartialEq)]
pub struct ImportState {
    pub file_path: String,
    pub target_deck_name: String,
    pub import_format: ImportFormat,
    pub progress: Option<f32>,
}

/// 导出状态
#[derive(Debug, Clone, PartialEq)]
pub struct ExportState {
    pub deck_id: Uuid,
    pub file_path: String,
    pub export_format: ExportFormat,
    pub progress: Option<f32>,
}

/// 导入格式
#[derive(Debug, Clone, PartialEq)]
pub enum ImportFormat {
    Csv,
    Toml,
    Json,
    Apkg,
}

/// 导出格式
#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
    Csv,
    Toml,
    Json,
    Apkg,
}

/// 卡片复习状态
#[derive(Debug, Clone)]
pub struct CardReviewState {
    pub session_progress: SessionProgress,
    pub current_card_display: CardDisplayState,
    pub review_mode: ReviewMode,
}

/// 卡片显示状态
#[derive(Debug, Clone, PartialEq)]
pub struct CardDisplayState {
    pub show_answer: bool,
    pub show_front_first: bool,
    pub current_side: CardSide,
}

/// 卡片面
#[derive(Debug, Clone, PartialEq)]
pub enum CardSide {
    Front,
    Back,
}

/// 复习模式
#[derive(Debug, Clone, PartialEq)]
pub enum ReviewMode {
    Normal,
    Browsing,
    Learning,
}

/// 统计状态
#[derive(Debug, Clone, PartialEq)]
pub struct StatisticsState {
    pub selected_deck_id: Option<Uuid>,
    pub time_period: StatsTimePeriod,
    pub display_mode: StatsDisplayMode,
}

/// 统计时间周期
#[derive(Debug, Clone, PartialEq)]
pub enum StatsTimePeriod {
    Today,
    Week,
    Month,
    Quarter,
    Year,
    AllTime,
}

/// 统计显示模式
#[derive(Debug, Clone, PartialEq)]
pub enum StatsDisplayMode {
    Overview,
    Detailed,
    Charts,
}

/// 设置状态
#[derive(Debug, Clone, PartialEq)]
pub struct SettingsState {
    pub current_tab: SettingsTab,
    pub edited_settings: HashMap<String, String>,
    pub pending_changes: bool,
}

/// 设置标签页
#[derive(Debug, Clone, PartialEq)]
pub enum SettingsTab {
    General,
    Learning,
    Display,
    Keyboard,
    Advanced,
}

/// 帮助状态
#[derive(Debug, Clone, PartialEq)]
pub struct HelpState {
    pub selected_topic: HelpTopic,
    pub scroll_position: usize,
}

/// 帮助主题
#[derive(Debug, Clone, PartialEq)]
pub enum HelpTopic {
    Overview,
    Shortcuts,
    DeckManagement,
    CardReview,
    Settings,
}

/// 确认退出状态
#[derive(Debug, Clone, PartialEq)]
pub struct ConfirmExitState {
    pub selected_option: bool, // true = yes, false = no
}

/// UI状态管理器 - 专门管理UI状态转换
#[derive(Debug, Clone)]
pub struct UIStateManager {
    pub current_state: UIState,
    pub previous_states: Vec<UIState>,
    pub state_history: Vec<StateTransition>,
    pub max_history_size: usize,
}

/// 状态转换记录
#[derive(Debug, Clone)]
pub struct StateTransition {
    pub from_state: UIState,
    pub to_state: UIState,
    pub timestamp: DateTime<Utc>,
    pub trigger: String,
}

impl UIStateManager {
    pub fn new() -> Self {
        Self {
            current_state: UIState::MainMenu(MainMenuState {
                selected_item: 0,
                menu_items: vec![
                    MenuItem {
                        id: "review".to_string(),
                        label: "Start Review".to_string(),
                        enabled: true,
                    },
                    MenuItem {
                        id: "deck_management".to_string(),
                        label: "Deck Management".to_string(),
                        enabled: true,
                    },
                    MenuItem {
                        id: "statistics".to_string(),
                        label: "Statistics".to_string(),
                        enabled: true,
                    },
                    MenuItem {
                        id: "settings".to_string(),
                        label: "Settings".to_string(),
                        enabled: true,
                    },
                    MenuItem {
                        id: "help".to_string(),
                        label: "Help".to_string(),
                        enabled: true,
                    },
                    MenuItem {
                        id: "quit".to_string(),
                        label: "Quit".to_string(),
                        enabled: true,
                    },
                ],
            }),
            previous_states: Vec::new(),
            state_history: Vec::new(),
            max_history_size: 50,
        }
    }

    /// 转换到新的UI状态
    pub fn transition_to(&mut self, new_state: UIState, trigger: String) -> anyhow::Result<()> {
        let transition = StateTransition {
            from_state: self.current_state.clone(),
            to_state: new_state.clone(),
            timestamp: Utc::now(),
            trigger,
        };

        // 保存前一个状态
        self.previous_states.push(self.current_state.clone());
        if self.previous_states.len() > 10 {
            self.previous_states.remove(0);
        }

        // 记录转换历史
        self.state_history.push(transition);
        if self.state_history.len() > self.max_history_size {
            self.state_history.remove(0);
        }

        // 更新当前状态
        self.current_state = new_state;

        Ok(())
    }

    /// 返回上一个状态
    pub fn go_back(&mut self) -> anyhow::Result<bool> {
        if let Some(previous_state) = self.previous_states.pop() {
            let transition = StateTransition {
                from_state: self.current_state.clone(),
                to_state: previous_state.clone(),
                timestamp: Utc::now(),
                trigger: "go_back".to_string(),
            };

            self.state_history.push(transition);
            self.current_state = previous_state;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 获取当前状态的引用
    pub fn get_current_state(&self) -> &UIState {
        &self.current_state
    }

    /// 获取状态历史
    pub fn get_state_history(&self) -> Vec<StateTransition> {
        self.state_history.clone()
    }

    // 状态访问器 - 提供便捷的状态访问方法
    pub fn as_main_menu_state(&self) -> Option<&MainMenuState> {
        match &self.current_state {
            UIState::MainMenu(state) => Some(state),
            _ => None,
        }
    }

    pub fn as_deck_management_state(&self) -> Option<&DeckManagementState> {
        match &self.current_state {
            UIState::DeckManagement(state) => Some(state),
            _ => None,
        }
    }

    pub fn as_deck_selection_state(&self) -> Option<&DeckSelectionState> {
        match &self.current_state {
            UIState::DeckSelection(state) => Some(state),
            _ => None,
        }
    }

    pub fn as_card_review_state(&self) -> Option<&CardReviewState> {
        match &self.current_state {
            UIState::CardReview(state) => Some(state),
            _ => None,
        }
    }

    pub fn as_statistics_state(&self) -> Option<&StatisticsState> {
        match &self.current_state {
            UIState::Statistics(state) => Some(state),
            _ => None,
        }
    }

    pub fn as_settings_state(&self) -> Option<&SettingsState> {
        match &self.current_state {
            UIState::Settings(state) => Some(state),
            _ => None,
        }
    }

    // 可变状态访问器
    pub fn as_mut_main_menu_state(&mut self) -> Option<&mut MainMenuState> {
        match &mut self.current_state {
            UIState::MainMenu(state) => Some(state),
            _ => None,
        }
    }

    pub fn as_mut_deck_management_state(&mut self) -> Option<&mut DeckManagementState> {
        match &mut self.current_state {
            UIState::DeckManagement(state) => Some(state),
            _ => None,
        }
    }

    pub fn as_mut_deck_selection_state(&mut self) -> Option<&mut DeckSelectionState> {
        match &mut self.current_state {
            UIState::DeckSelection(state) => Some(state),
            _ => None,
        }
    }

    pub fn as_mut_card_review_state(&mut self) -> Option<&mut CardReviewState> {
        match &mut self.current_state {
            UIState::CardReview(state) => Some(state),
            _ => None,
        }
    }

    pub fn as_mut_statistics_state(&mut self) -> Option<&mut StatisticsState> {
        match &mut self.current_state {
            UIState::Statistics(state) => Some(state),
            _ => None,
        }
    }

    pub fn as_mut_settings_state(&mut self) -> Option<&mut SettingsState> {
        match &mut self.current_state {
            UIState::Settings(state) => Some(state),
            _ => None,
        }
    }
}

impl Default for UIStateManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 系统消息 - 用于向用户显示状态信息
#[derive(Debug, Clone)]
pub struct SystemMessage {
    pub level: MessageLevel,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub auto_dismiss: Option<std::time::Duration>,
}

/// 消息级别
#[derive(Debug, Clone, PartialEq)]
pub enum MessageLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl SystemMessage {
    pub fn info(content: String) -> Self {
        Self {
            level: MessageLevel::Info,
            content,
            timestamp: Utc::now(),
            auto_dismiss: Some(std::time::Duration::from_secs(5)),
        }
    }

    pub fn success(content: String) -> Self {
        Self {
            level: MessageLevel::Success,
            content,
            timestamp: Utc::now(),
            auto_dismiss: Some(std::time::Duration::from_secs(3)),
        }
    }

    pub fn warning(content: String) -> Self {
        Self {
            level: MessageLevel::Warning,
            content,
            timestamp: Utc::now(),
            auto_dismiss: Some(std::time::Duration::from_secs(8)),
        }
    }

    pub fn error(content: String) -> Self {
        Self {
            level: MessageLevel::Error,
            content,
            timestamp: Utc::now(),
            auto_dismiss: None, // 错误消息不自动消失
        }
    }
}
