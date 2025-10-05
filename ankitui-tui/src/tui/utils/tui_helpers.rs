//! TUI助手函数模块
//!
//! 提供各种TUI特定的助手函数和工具

use ankitui_core::models::*;
use ratatui::style::{Color, Modifier, Style};
use std::collections::HashMap;

/// TUI颜色助手
pub struct TuiColorHelper;

impl TuiColorHelper {
    /// 根据卡片状态获取对应的颜色
    pub fn card_state_color(state: CardState) -> Color {
        match state {
            CardState::New => Color::Cyan,
            CardState::Learning => Color::Yellow,
            CardState::Review => Color::Green,
            CardState::Relearning => Color::Magenta,
            CardState::Buried => Color::Gray,
            CardState::Suspended => Color::DarkGray,
        }
    }

    /// 根据答案评分获取对应的颜色
    pub fn rating_color(rating: i32) -> Color {
        match rating {
            0 => Color::Red,    // Again
            1 => Color::Yellow, // Hard
            2 => Color::Green,  // Good
            3 => Color::Blue,   // Easy
            _ => Color::White,
        }
    }

    /// 根据重要性获取文本颜色
    pub fn importance_color(importance: &str) -> Color {
        match importance.to_lowercase().as_str() {
            "critical" | "urgent" => Color::Red,
            "high" | "important" => Color::Yellow,
            "medium" | "normal" => Color::Green,
            "low" | "optional" => Color::Cyan,
            _ => Color::White,
        }
    }

    /// 创建渐变色（用于进度条等）
    pub fn gradient_colors(start: Color, end: Color, steps: usize) -> Vec<Color> {
        // 简化的颜色插值，返回预定义的渐变
        match (start, end) {
            (Color::Red, Color::Green) => vec![Color::Red, Color::Yellow, Color::Green],
            (Color::Blue, Color::Cyan) => vec![Color::Blue, Color::LightBlue, Color::Cyan],
            _ => vec![start; steps],
        }
    }
}

/// TUI符号助手
pub struct TuiSymbolHelper;

impl TuiSymbolHelper {
    /// 获取卡片状态的符号
    pub fn card_state_symbol(state: CardState) -> &'static str {
        match state {
            CardState::New => "🆕",
            CardState::Learning => "📚",
            CardState::Review => "📖",
            CardState::Relearning => "🔄",
            CardState::Buried => "⚰️",
            CardState::Suspended => "⏸️",
        }
    }

    /// 获取媒体类型的符号
    pub fn media_type_symbol(media_type: MediaType) -> &'static str {
        match media_type {
            MediaType::Audio => "🎵",
            MediaType::Image => "🖼️",
            MediaType::Video => "🎬",
        }
    }

    /// 获取评分的符号
    pub fn rating_symbol(rating: i32) -> &'static str {
        match rating {
            0 => "❌", // Again
            1 => "😓", // Hard
            2 => "😊", // Good
            3 => "😄", // Easy
            _ => "❓",
        }
    }

    /// 获取操作符号
    pub fn action_symbol(action: &str) -> &'static str {
        match action.to_lowercase().as_str() {
            "add" | "create" => "➕",
            "edit" | "modify" => "✏️",
            "delete" | "remove" => "🗑️",
            "search" | "find" => "🔍",
            "save" => "💾",
            "load" => "📂",
            "settings" | "config" => "⚙️",
            "help" | "info" => "❓",
            "home" => "🏠",
            "back" => "⬅️",
            "forward" => "➡️",
            "up" => "⬆️",
            "down" => "⬇️",
            "refresh" => "🔄",
            "check" | "verify" => "✓",
            "error" | "warning" => "⚠️",
            _ => "•",
        }
    }

    /// 创建进度条符号
    pub fn progress_bar_symbols(progress: f32, width: usize) -> String {
        let filled = (progress * width as f32) as usize;
        let empty = width.saturating_sub(filled);

        format!("{}{}", "█".repeat(filled), "░".repeat(empty))
    }
}

/// TUI样式助手
pub struct TuiStyleHelper;

impl TuiStyleHelper {
    /// 创建默认样式
    pub fn default_style() -> Style {
        Style::default()
    }

    /// 创建标题样式
    pub fn title_style() -> Style {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    }

    /// 创建副标题样式
    pub fn subtitle_style() -> Style {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    }

    /// 创建成功样式
    pub fn success_style() -> Style {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    }

    /// 创建错误样式
    pub fn error_style() -> Style {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    }

    /// 创建警告样式
    pub fn warning_style() -> Style {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    }

    /// 创建信息样式
    pub fn info_style() -> Style {
        Style::default().fg(Color::Cyan)
    }

    /// 创建次要文本样式
    pub fn secondary_style() -> Style {
        Style::default().fg(Color::DarkGray)
    }

    /// 创建高亮样式
    pub fn highlight_style() -> Style {
        Style::default()
            .fg(Color::White)
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD)
    }

    /// 创建禁用样式
    pub fn disabled_style() -> Style {
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::DIM)
    }

    /// 创建卡片样式
    pub fn card_style(is_active: bool) -> Style {
        if is_active {
            Style::default()
                .fg(Color::White)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        }
    }

    /// 创建按钮样式
    pub fn button_style(is_selected: bool) -> Style {
        if is_selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White).bg(Color::Blue)
        }
    }

    /// 创建输入框样式
    pub fn input_style(is_focused: bool) -> Style {
        if is_focused {
            Style::default().fg(Color::Yellow).bg(Color::Black)
        } else {
            Style::default().fg(Color::White).bg(Color::Black)
        }
    }
}

/// TUI布局助手
pub struct TuiLayoutHelper;

impl TuiLayoutHelper {
    /// 计算居中位置
    pub fn center_in_area(area_width: u16, content_width: u16) -> u16 {
        if content_width >= area_width {
            0
        } else {
            (area_width - content_width) / 2
        }
    }

    /// 创建边框标题
    pub fn border_title(title: &str, style: Style) -> (String, Style) {
        (format!(" {} ", title), style)
    }

    /// 创建状态栏文本
    pub fn status_bar_text(message: &str, is_error: bool, is_success: bool) -> (String, Style) {
        let style = if is_error {
            TuiStyleHelper::error_style()
        } else if is_success {
            TuiStyleHelper::success_style()
        } else {
            TuiStyleHelper::info_style()
        };

        (message.to_string(), style)
    }

    /// 格式化快捷键提示
    pub fn shortcut_hint(key: &str, description: &str) -> String {
        format!(" [{}]: {}", key, description)
    }

    /// 创建分隔线
    pub fn separator_line(length: u16, double: bool) -> String {
        if double {
            "═".repeat(length as usize)
        } else {
            "─".repeat(length as usize)
        }
    }
}

/// TUI验证助手
pub struct TuiValidationHelper;

impl TuiValidationHelper {
    /// 验证文件名（TUI友好）
    pub fn is_valid_filename(filename: &str) -> bool {
        !filename.is_empty()
            && filename.len() <= 255
            && !filename
                .chars()
                .any(|c| c.is_control() || "/\\:*?\"<>|".contains(c))
    }

    /// 验证标签名
    pub fn is_valid_tag_name(tag: &str) -> bool {
        !tag.is_empty()
            && tag.len() <= 50
            && tag
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    }

    /// 验证卡片内容不为空
    pub fn is_valid_card_content(front: &str, back: &str) -> bool {
        !front.trim().is_empty() && !back.trim().is_empty()
    }

    /// 创建验证消息样式
    pub fn validation_message_style(is_valid: bool) -> Style {
        if is_valid {
            TuiStyleHelper::success_style()
        } else {
            TuiStyleHelper::error_style()
        }
    }

    /// 格式化验证结果
    pub fn format_validation_result(result: bool, success_msg: &str, error_msg: &str) -> String {
        if result {
            format!("✓ {}", success_msg)
        } else {
            format!("✗ {}", error_msg)
        }
    }
}

/// TUI搜索助手
pub struct TuiSearchHelper;

impl TuiSearchHelper {
    /// 高亮搜索关键词
    pub fn highlight_search_term(text: &str, term: &str) -> Vec<(String, Style)> {
        if term.is_empty() {
            return vec![(text.to_string(), TuiStyleHelper::default_style())];
        }

        let mut result = Vec::new();
        let mut last_pos = 0;

        while let Some(pos) = text[last_pos..].to_lowercase().find(&term.to_lowercase()) {
            let abs_pos = last_pos + pos;

            // 添加匹配前的文本
            if abs_pos > last_pos {
                result.push((
                    text[last_pos..abs_pos].to_string(),
                    TuiStyleHelper::default_style(),
                ));
            }

            // 添加匹配的文本（高亮）
            let end_pos = abs_pos + term.len();
            result.push((
                text[abs_pos..end_pos].to_string(),
                TuiStyleHelper::highlight_style(),
            ));

            last_pos = end_pos;
        }

        // 添加剩余的文本
        if last_pos < text.len() {
            result.push((
                text[last_pos..].to_string(),
                TuiStyleHelper::default_style(),
            ));
        }

        result
    }

    /// 创建搜索过滤器描述
    pub fn format_search_filters(filters: &HashMap<String, String>) -> String {
        if filters.is_empty() {
            "无过滤器".to_string()
        } else {
            filters
                .iter()
                .map(|(key, value)| format!("{}: {}", key, value))
                .collect::<Vec<_>>()
                .join(", ")
        }
    }

    /// 模糊匹配评分
    pub fn fuzzy_match_score(text: &str, pattern: &str) -> f32 {
        if pattern.is_empty() {
            return 0.0;
        }

        let text_lower = text.to_lowercase();
        let pattern_lower = pattern.to_lowercase();

        let mut score = 0.0;
        let mut pattern_pos = 0;

        for text_char in text_lower.chars() {
            if pattern_pos < pattern_lower.len() {
                let pattern_char = pattern_lower.chars().nth(pattern_pos).unwrap();
                if text_char == pattern_char {
                    score += 1.0;
                    pattern_pos += 1;
                }
            }
        }

        if pattern_lower.len() > 0 {
            score / pattern_lower.len() as f32
        } else {
            0.0
        }
    }
}

/// TUI消息助手
pub struct TuiMessageHelper;

impl TuiMessageHelper {
    /// 格式化成功消息
    pub fn success_message(message: &str) -> (String, Style) {
        (format!("✓ {}", message), TuiStyleHelper::success_style())
    }

    /// 格式化错误消息
    pub fn error_message(message: &str) -> (String, Style) {
        (format!("✗ {}", message), TuiStyleHelper::error_style())
    }

    /// 格式化警告消息
    pub fn warning_message(message: &str) -> (String, Style) {
        (format!("⚠ {}", message), TuiStyleHelper::warning_style())
    }

    /// 格式化信息消息
    pub fn info_message(message: &str) -> (String, Style) {
        (format!("ℹ {}", message), TuiStyleHelper::info_style())
    }

    /// 格式化进度消息
    pub fn progress_message(current: u32, total: u32, message: &str) -> (String, Style) {
        let percentage = if total > 0 {
            (current as f32 / total as f32 * 100.0) as u32
        } else {
            0
        };

        (
            format!("{} ({}%)", message, percentage),
            TuiStyleHelper::info_style(),
        )
    }

    /// 格式化帮助消息
    pub fn help_message(shortcuts: &[(&str, &str)]) -> Vec<String> {
        shortcuts
            .iter()
            .map(|(key, desc)| format!("{}: {}", key, desc))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_state_colors() {
        assert_eq!(
            TuiColorHelper::card_state_color(CardState::New),
            Color::Cyan
        );
        assert_eq!(
            TuiColorHelper::card_state_color(CardState::Review),
            Color::Green
        );
    }

    #[test]
    fn test_card_state_symbols() {
        assert_eq!(TuiSymbolHelper::card_state_symbol(CardState::New), "🆕");
        assert_eq!(TuiSymbolHelper::card_state_symbol(CardState::Review), "📖");
    }

    #[test]
    fn test_style_creation() {
        let title_style = TuiStyleHelper::title_style();
        assert!(title_style.fg == Some(Color::Cyan));
    }

    #[test]
    fn test_validation() {
        assert!(TuiValidationHelper::is_valid_filename("test.txt"));
        assert!(!TuiValidationHelper::is_valid_filename("test/file.txt"));
    }

    #[test]
    fn test_highlighting() {
        let text = "Hello world";
        let highlighted = TuiSearchHelper::highlight_search_term(text, "world");
        assert_eq!(highlighted.len(), 2); // "Hello " + "world"
    }
}
