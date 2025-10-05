//! TUI适配器模块
//!
//! 将不适合TUI的概念转换为终端友好的表示

use ankitui_core::models::*;

/// TUI内容适配器
/// 将Web概念转换为TUI友好的格式
pub struct TuiContentAdapter;

impl TuiContentAdapter {
    /// 将HTML样式内容转换为TUI格式
    pub fn html_to_tui(content: &str) -> String {
        content
            .replace("<hr>", "---")
            .replace("<br>", "\n")
            .replace("<div>", "")
            .replace("</div>", "")
            .replace("<p>", "")
            .replace("</p>", "\n")
            .replace("<strong>", "**")
            .replace("</strong>", "**")
            .replace("<em>", "*")
            .replace("</em>", "*")
            .replace("<b>", "**")
            .replace("</b>", "**")
            .replace("<i>", "*")
            .replace("</i>", "*")
            .replace("<u>", "__")
            .replace("</u>", "__")
            // 清理多余的空行
            .split("\n\n\n")
            .collect::<Vec<&str>>()
            .join("\n\n")
    }

    /// 将URL转换为TUI友好的显示格式
    pub fn url_to_tui(url: &str) -> String {
        format!("🔗 {}", url)
    }

    /// 将颜色代码转换为TUI颜色描述
    pub fn color_to_tui_description(color: &str) -> &'static str {
        match color.to_lowercase().as_str() {
            "red" | "#ff0000" => "红色",
            "green" | "#00ff00" => "绿色",
            "blue" | "#0000ff" => "蓝色",
            "yellow" | "#ffff00" => "黄色",
            "purple" | "#800080" => "紫色",
            "orange" | "#ffa500" => "橙色",
            "cyan" | "#00ffff" => "青色",
            "black" | "#000000" => "黑色",
            "white" | "#ffffff" => "白色",
            "gray" | "#808080" => "灰色",
            _ => "未知颜色",
        }
    }

    /// 将复杂的布局指令转换为简单的TUI分隔符
    pub fn layout_to_tui_separators() -> TuiSeparators {
        TuiSeparators {
            horizontal: "─".repeat(20),
            vertical: "│".to_string(),
            cross: "┼".to_string(),
            double_horizontal: "═".repeat(20),
            double_vertical: "║".to_string(),
            double_cross: "╬".to_string(),
        }
    }
}

/// TUI分隔符集合
pub struct TuiSeparators {
    pub horizontal: String,
    pub vertical: String,
    pub cross: String,
    pub double_horizontal: String,
    pub double_vertical: String,
    pub double_cross: String,
}

/// TUI样式适配器
/// 将样式概念转换为ratatui兼容的格式
pub struct TuiStyleAdapter;

impl TuiStyleAdapter {
    /// 将CSS类名转换为TUI样式
    pub fn css_class_to_tui_style(class_name: &str) -> TuiStyle {
        match class_name {
            "highlight" | "important" => TuiStyle::Highlight,
            "success" | "correct" => TuiStyle::Success,
            "error" | "incorrect" => TuiStyle::Error,
            "warning" => TuiStyle::Warning,
            "info" | "hint" => TuiStyle::Info,
            "muted" | "subtle" => TuiStyle::Muted,
            "bold" | "strong" => TuiStyle::Bold,
            "italic" | "emphasis" => TuiStyle::Italic,
            "underline" => TuiStyle::Underline,
            _ => TuiStyle::Default,
        }
    }

    /// 根据内容重要性选择TUI样式
    pub fn content_based_style(content: &str) -> TuiStyle {
        if content.contains("正确") || content.contains("✓") {
            TuiStyle::Success
        } else if content.contains("错误") || content.contains("✗") {
            TuiStyle::Error
        } else if content.contains("提示") || content.contains("注意") {
            TuiStyle::Warning
        } else if content.contains("示例") || content.contains("例如") {
            TuiStyle::Info
        } else {
            TuiStyle::Default
        }
    }
}

/// TUI样式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiStyle {
    Default,
    Highlight,
    Success,
    Error,
    Warning,
    Info,
    Muted,
    Bold,
    Italic,
    Underline,
}

/// TUI交互适配器
/// 将复杂的交互概念转换为TUI友好的操作
pub struct TuiInteractionAdapter;

impl TuiInteractionAdapter {
    /// 将复杂的表单转换为TUI输入提示
    pub fn form_to_tui_prompts(form_fields: &[(&str, &str)]) -> Vec<TuiInputPrompt> {
        form_fields
            .iter()
            .map(|(label, placeholder)| TuiInputPrompt {
                label: label.to_string(),
                placeholder: placeholder.to_string(),
                input_type: TuiInteractionAdapter::guess_input_type(placeholder),
                validation: None,
            })
            .collect()
    }

    /// 将多选菜单转换为TUI选择列表
    pub fn menu_to_tui_selection(options: &[&str]) -> TuiSelectionList {
        TuiSelectionList {
            title: "请选择".to_string(),
            options: options.iter().map(|&opt| opt.to_string()).collect(),
            allow_multiple: false,
            max_visible: 10,
        }
    }

    /// 猜测输入类型
    fn guess_input_type(placeholder: &str) -> TuiInputType {
        if placeholder.to_lowercase().contains("password") {
            TuiInputType::Password
        } else if placeholder.to_lowercase().contains("number") {
            TuiInputType::Number
        } else if placeholder.to_lowercase().contains("email") {
            TuiInputType::Email
        } else {
            TuiInputType::Text
        }
    }
}

/// TUI输入提示
#[derive(Debug, Clone)]
pub struct TuiInputPrompt {
    pub label: String,
    pub placeholder: String,
    pub input_type: TuiInputType,
    pub validation: Option<String>,
}

/// TUI输入类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiInputType {
    Text,
    Number,
    Email,
    Password,
    Date,
}

/// TUI选择列表
#[derive(Debug, Clone)]
pub struct TuiSelectionList {
    pub title: String,
    pub options: Vec<String>,
    pub allow_multiple: bool,
    pub max_visible: usize,
}

/// TUI媒体适配器
/// 将媒体概念转换为TUI友好的表示
pub struct TuiMediaAdapter;

impl TuiMediaAdapter {
    /// 将图片转换为TUI描述
    pub fn image_to_tui_description(media_ref: &MediaRef) -> String {
        match media_ref.media_type {
            MediaType::Image => {
                format!("🖼️  图片: {}", media_ref.path)
            }
            MediaType::Audio => {
                format!("🎵 音频: {}", media_ref.path)
            }
            MediaType::Video => {
                format!("🎬 视频: {}", media_ref.path)
            }
        }
    }

    /// 创建TUI媒体控制提示
    pub fn media_controls_tui() -> Vec<&'static str> {
        vec![
            "M: 打开/播放媒体文件",
            "↑↓: 调节音量 (音频)",
            "Space: 播放/暂停 (音频/视频)",
            "Esc: 关闭媒体预览",
        ]
    }

    /// 生成媒体文件的ASCII艺术占位符
    pub fn media_ascii_placeholder(media_type: MediaType) -> &'static str {
        match media_type {
            MediaType::Image => {
                r#"
┌─────────────────┐
│                 │
│     🖼️  图片     │
│                 │
│   (按M查看图片)   │
│                 │
└─────────────────┘
"#
            }
            MediaType::Audio => {
                r#"
┌─────────────────┐
│     ♪ ♪ ♪       │
│                 │
│     🎵 音频      │
│                 │
│   (按M播放音频)   │
│                 │
└─────────────────┘
"#
            }
            MediaType::Video => {
                r#"
┌─────────────────┐
│                 │
│     ▶️ 🎬        │
│                 │
│     视频        │
│                 │
│   (按M播放视频)   │
│                 │
└─────────────────┘
"#
            }
        }
    }
}

/// TUI导航适配器
/// 将复杂的导航结构转换为TUI友好的菜单
pub struct TuiNavigationAdapter;

impl TuiNavigationAdapter {
    /// 将面包屑导航转换为TUI路径显示
    pub fn breadcrumb_to_tui(path: &[&str]) -> String {
        path.join(" → ")
    }

    /// 将分页导航转换为TUI控制提示
    pub fn pagination_to_tui_controls(current: usize, total: usize) -> Vec<String> {
        let mut controls = vec![];

        if current > 1 {
            controls.push("P: 上一页".to_string());
        }
        controls.push(format!("第 {}/{} 页", current, total));
        if current < total {
            controls.push("N: 下一页".to_string());
        }

        controls
    }

    /// 创建TUI快捷键帮助
    pub fn shortcut_help_tui() -> Vec<(&'static str, &'static str)> {
        vec![
            ("1-4", "评分 (再次-困难-良好-简单)"),
            ("Space", "显示答案"),
            ("Enter", "提交答案"),
            ("M", "打开媒体"),
            ("Esc", "返回/退出"),
            ("Tab", "切换焦点"),
            ("?", "显示帮助"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_to_tui_conversion() {
        let html = "<p>Hello <strong>world</strong></p><hr>";
        let tui = TuiContentAdapter::html_to_tui(html);
        assert!(tui.contains("Hello **world**"));
        assert!(tui.contains("---"));
        assert!(!tui.contains("<"));
    }

    #[test]
    fn test_css_class_to_tui_style() {
        assert_eq!(
            TuiStyleAdapter::css_class_to_tui_style("success"),
            TuiStyle::Success
        );
        assert_eq!(
            TuiStyleAdapter::css_class_to_tui_style("unknown"),
            TuiStyle::Default
        );
    }

    #[test]
    fn test_form_to_tui_prompts() {
        let form = vec![("用户名", "请输入用户名"), ("密码", "请输入密码")];
        let prompts = TuiInteractionAdapter::form_to_tui_prompts(&form);
        assert_eq!(prompts.len(), 2);
        assert_eq!(prompts[1].input_type, TuiInputType::Password);
    }
}
