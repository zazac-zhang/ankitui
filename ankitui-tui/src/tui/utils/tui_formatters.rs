//! TUI格式化器模块
//!
//! 提供各种数据格式化为TUI友好显示的功能

use ankitui_core::models::*;
use chrono::{DateTime, Utc};
use std::time::Duration;

/// TUI文本格式化器
pub struct TuiTextFormatter;

impl TuiTextFormatter {
    /// 格式化文本长度以适应TUI显示
    pub fn truncate_with_ellipsis(text: &str, max_width: usize) -> String {
        if text.len() <= max_width {
            text.to_string()
        } else {
            format!("{}...", &text[..max_width.saturating_sub(3)])
        }
    }

    /// 格式化文本为居中显示
    pub fn center_text(text: &str, width: usize) -> String {
        let padding = width.saturating_sub(text.len()) / 2;
        format!("{}{}", " ".repeat(padding), text)
    }

    /// 格式化文本为右对齐显示
    pub fn right_align_text(text: &str, width: usize) -> String {
        let padding = width.saturating_sub(text.len());
        format!("{}{}", " ".repeat(padding), text)
    }

    /// 格式化列表为TUI友好的项目符号
    pub fn format_list(items: &[String], style: ListStyle) -> Vec<String> {
        items
            .iter()
            .enumerate()
            .map(|(i, item)| match style {
                ListStyle::Bullets => format!("• {}", item),
                ListStyle::Numbered => format!("{}. {}", i + 1, item),
                ListStyle::Dashed => format!("- {}", item),
                ListStyle::Arrow => format!("→ {}", item),
                ListStyle::Star => format!("★ {}", item),
            })
            .collect()
    }

    /// 格式化标签为TUI友好的显示
    pub fn format_tags(tags: &[String], max_tags: usize) -> String {
        if tags.is_empty() {
            return "无标签".to_string();
        }

        let display_tags: Vec<String> = tags
            .iter()
            .take(max_tags)
            .map(|tag| format!("[{}]", tag))
            .collect();

        let mut result = display_tags.join(" ");

        if tags.len() > max_tags {
            result.push_str(&format!(" (+{})", tags.len() - max_tags));
        }

        result
    }

    /// 格式化进度条为TUI显示
    pub fn format_progress_bar(current: u32, total: u32, width: usize) -> String {
        if total == 0 {
            return "█".repeat(width);
        }

        let filled = (current as f64 / total as f64 * width as f64) as usize;
        let empty = width.saturating_sub(filled);

        format!("{}{}", "█".repeat(filled), "░".repeat(empty))
    }

    /// 格式化星级评分
    pub fn format_star_rating(rating: f32, max_stars: u8) -> String {
        let full_stars = (rating as u8).min(max_stars);
        let half_star = if rating.fract() >= 0.5 && full_stars < max_stars {
            1
        } else {
            0
        };
        let empty_stars = max_stars.saturating_sub(full_stars + half_star);

        format!(
            "{}{}{}",
            "★".repeat(full_stars as usize),
            "☆".repeat(half_star as usize),
            "☆".repeat(empty_stars as usize)
        )
    }
}

/// 列表样式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListStyle {
    Bullets,  // • 项目
    Numbered, // 1. 项目
    Dashed,   // - 项目
    Arrow,    // → 项目
    Star,     // ★ 项目
}

/// TUI数字格式化器
pub struct TuiNumberFormatter;

impl TuiNumberFormatter {
    /// 格式化大数字为人类可读格式
    pub fn format_large_number(num: u64) -> String {
        match num {
            n if n < 1_000 => n.to_string(),
            n if n < 1_000_000 => format!("{:.1}K", n as f64 / 1_000.0),
            n if n < 1_000_000_000 => format!("{:.1}M", n as f64 / 1_000_000.0),
            n => format!("{:.1}B", n as f64 / 1_000_000_000.0),
        }
    }

    /// 格式化百分比
    pub fn format_percentage(value: f32, total: f32) -> String {
        if total == 0.0 {
            return "0%".to_string();
        }
        let percentage = (value / total * 100.0).round();
        format!("{}%", percentage)
    }

    /// 格式化时间间隔
    pub fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.as_secs();

        if total_seconds < 60 {
            format!("{}秒", total_seconds)
        } else if total_seconds < 3600 {
            let minutes = total_seconds / 60;
            let seconds = total_seconds % 60;
            format!("{}分{}秒", minutes, seconds)
        } else if total_seconds < 86400 {
            let hours = total_seconds / 3600;
            let minutes = (total_seconds % 3600) / 60;
            format!("{}小时{}分", hours, minutes)
        } else {
            let days = total_seconds / 86400;
            let hours = (total_seconds % 86400) / 3600;
            format!("{}天{}小时", days, hours)
        }
    }

    /// 格式化数字范围
    pub fn format_range(start: u32, end: u32) -> String {
        if start == end {
            start.to_string()
        } else {
            format!("{}-{}", start, end)
        }
    }
}

/// TUI时间格式化器
pub struct TuiTimeFormatter;

impl TuiTimeFormatter {
    /// 格式化相对时间（如"3小时前"）
    pub fn format_relative_time(datetime: DateTime<Utc>) -> String {
        let now = Utc::now();
        let duration = now.signed_duration_since(datetime);

        if duration.num_days() > 0 {
            let days = duration.num_days();
            if days == 1 {
                "1天前".to_string()
            } else {
                format!("{}天前", days)
            }
        } else if duration.num_hours() > 0 {
            let hours = duration.num_hours();
            if hours == 1 {
                "1小时前".to_string()
            } else {
                format!("{}小时前", hours)
            }
        } else if duration.num_minutes() > 0 {
            let minutes = duration.num_minutes();
            if minutes == 1 {
                "1分钟前".to_string()
            } else {
                format!("{}分钟前", minutes)
            }
        } else if duration.num_seconds() > 0 {
            let seconds = duration.num_seconds();
            if seconds == 1 {
                "1秒前".to_string()
            } else {
                format!("{}秒前", seconds)
            }
        } else {
            "刚刚".to_string()
        }
    }

    /// 格式化绝对时间为TUI友好格式
    pub fn format_datetime(datetime: DateTime<Utc>) -> String {
        datetime.format("%Y-%m-%d %H:%M").to_string()
    }

    /// 格式化时间为简短格式
    pub fn format_time_short(datetime: DateTime<Utc>) -> String {
        let now = Utc::now();
        let duration = now.signed_duration_since(datetime);

        if duration.num_days() == 0 {
            datetime.format("%H:%M").to_string()
        } else if duration.num_days() < 7 {
            datetime.format("%m-%d").to_string()
        } else {
            datetime.format("%Y-%m-%d").to_string()
        }
    }

    /// 格式化到期时间
    pub fn format_due_time(datetime: DateTime<Utc>) -> String {
        let now = Utc::now();
        let duration = datetime.signed_duration_since(now);

        if duration.num_seconds() < 0 {
            let past_duration = -duration;
            if past_duration.num_days() > 0 {
                format!("逾期{}天", past_duration.num_days())
            } else if past_duration.num_hours() > 0 {
                format!("逾期{}小时", past_duration.num_hours())
            } else {
                "刚刚逾期".to_string()
            }
        } else if duration.num_seconds() < 300 {
            // 5分钟内
            "现在复习".to_string()
        } else if duration.num_hours() < 24 {
            format!("{}小时后", duration.num_hours())
        } else if duration.num_days() < 7 {
            format!("{}天后", duration.num_days())
        } else {
            format!("{}", TuiTimeFormatter::format_datetime(datetime))
        }
    }
}

/// TUI统计格式化器
pub struct TuiStatsFormatter;

impl TuiStatsFormatter {
    /// 格式化学习统计
    pub fn format_learning_stats(
        cards_studied: u32,
        total_time: Duration,
        accuracy: f32,
    ) -> Vec<String> {
        vec![
            format!(
                "已学习卡片: {}",
                TuiNumberFormatter::format_large_number(cards_studied as u64)
            ),
            format!(
                "学习时长: {}",
                TuiNumberFormatter::format_duration(total_time)
            ),
            format!(
                "正确率: {}",
                TuiNumberFormatter::format_percentage(accuracy, 100.0)
            ),
        ]
    }

    /// 格式化卡片状态统计
    pub fn format_card_status_stats(
        new: u32,
        learning: u32,
        review: u32,
        suspended: u32,
    ) -> Vec<String> {
        let total = new + learning + review + suspended;
        vec![
            format!(
                "新卡: {} ({})",
                new,
                TuiNumberFormatter::format_percentage(new as f32, total as f32)
            ),
            format!(
                "学习中: {} ({})",
                learning,
                TuiNumberFormatter::format_percentage(learning as f32, total as f32)
            ),
            format!(
                "复习中: {} ({})",
                review,
                TuiNumberFormatter::format_percentage(review as f32, total as f32)
            ),
            format!(
                "已暂停: {} ({})",
                suspended,
                TuiNumberFormatter::format_percentage(suspended as f32, total as f32)
            ),
        ]
    }

    /// 格式化间隔统计
    pub fn format_interval_stats(intervals: &[i32]) -> Vec<String> {
        if intervals.is_empty() {
            return vec!["暂无数据".to_string()];
        }

        let avg_interval = intervals.iter().sum::<i32>() as f32 / intervals.len() as f32;
        let min_interval = *intervals.iter().min().unwrap();
        let max_interval = *intervals.iter().max().unwrap();

        vec![
            format!("平均间隔: {:.1}天", avg_interval),
            format!("最短间隔: {}天", min_interval),
            format!("最长间隔: {}天", max_interval),
            format!("总卡片数: {}", intervals.len()),
        ]
    }

    /// 创建TUI统计图表
    pub fn create_simple_bar_chart(data: &[(String, u32)], width: usize) -> Vec<String> {
        let max_value = data.iter().map(|(_, v)| *v).max().unwrap_or(1);

        data.iter()
            .map(|(label, value)| {
                let bar_width = (*value as f64 / max_value as f64 * width as f64) as usize;
                let bar = "█".repeat(bar_width);
                format!(
                    "{}: {} |{}",
                    TuiTextFormatter::right_align_text(label, 12),
                    value,
                    bar
                )
            })
            .collect()
    }
}

/// TUI表格格式化器
pub struct TuiTableFormatter;

impl TuiTableFormatter {
    /// 创建简单的TUI表格
    pub fn format_table(headers: &[String], rows: &[Vec<String>]) -> Vec<String> {
        if rows.is_empty() {
            return vec!["暂无数据".to_string()];
        }

        // 计算每列的最大宽度
        let mut column_widths = vec![0; headers.len()];

        // 计算表头宽度
        for (i, header) in headers.iter().enumerate() {
            column_widths[i] = column_widths[i].max(header.len());
        }

        // 计算数据行宽度
        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < column_widths.len() {
                    column_widths[i] = column_widths[i].max(cell.len());
                }
            }
        }

        let mut result = vec![];

        // 创建分隔线
        let separator: String = column_widths
            .iter()
            .map(|&w| "─".repeat(w + 2))
            .collect::<Vec<_>>()
            .join("┼");

        // 添加表头
        let header_row: String = headers
            .iter()
            .enumerate()
            .map(|(i, header)| {
                format!(
                    " {} ",
                    TuiTextFormatter::center_text(header, column_widths[i])
                )
            })
            .collect::<Vec<_>>()
            .join("│");

        result.push(header_row);
        result.push(separator);

        // 添加数据行
        for row in rows {
            let data_row: String = row
                .iter()
                .enumerate()
                .take_while(|(i, _)| *i < column_widths.len())
                .map(|(i, cell)| {
                    format!(
                        " {} ",
                        TuiTextFormatter::right_align_text(cell, column_widths[i])
                    )
                })
                .collect::<Vec<_>>()
                .join("│");

            result.push(data_row);
        }

        result
    }

    /// 格式化卡片信息为表格
    pub fn format_card_info(card: &Card) -> Vec<String> {
        let headers = vec!["属性".to_string(), "值".to_string()];
        let rows = vec![
            vec!["ID".to_string(), card.content.id.to_string()],
            vec!["类型".to_string(), "Basic".to_string()], // 这里应该从实际类型获取
            vec![
                "创建时间".to_string(),
                TuiTimeFormatter::format_datetime(card.content.created_at),
            ],
            vec![
                "修改时间".to_string(),
                TuiTimeFormatter::format_datetime(card.content.modified_at),
            ],
            vec!["标签数量".to_string(), card.content.tags.len().to_string()],
            vec!["间隔".to_string(), format!("{}天", card.state.interval)],
            vec![
                "难度因子".to_string(),
                format!("{:.1}", card.state.ease_factor),
            ],
            vec!["复习次数".to_string(), card.state.reps.to_string()],
            vec!["遗忘次数".to_string(), card.state.lapses.to_string()],
        ];

        Self::format_table(&headers, &rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_truncation() {
        let long_text = "This is a very long text that should be truncated";
        let result = TuiTextFormatter::truncate_with_ellipsis(long_text, 20);
        assert!(result.ends_with("..."));
        assert!(result.len() <= 23); // 20 + 3 for "..."
    }

    #[test]
    fn test_large_number_formatting() {
        assert_eq!(TuiNumberFormatter::format_large_number(500), "500");
        assert_eq!(TuiNumberFormatter::format_large_number(1500), "1.5K");
        assert_eq!(TuiNumberFormatter::format_large_number(2_500_000), "2.5M");
    }

    #[test]
    fn test_relative_time_formatting() {
        let now = Utc::now();
        let past = now - chrono::Duration::hours(2);
        let result = TuiTimeFormatter::format_relative_time(past);
        assert!(result.contains("2小时前"));
    }

    #[test]
    fn test_progress_bar() {
        let progress = TuiTextFormatter::format_progress_bar(50, 100, 10);
        assert_eq!(progress.len(), 10);
        assert_eq!(progress, "█████░░░░░");
    }
}
