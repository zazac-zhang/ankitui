# 基于状态的Event处理重构

## 🎯 问题分析

您提出的问题非常正确：**同一个事件在不同状态下应该有不同的含义**。原始的静态Event到Command映射存在以下问题：

### 原始设计的问题

1. **静态映射**: Event → Command 的映射是固定的，不考虑上下文
2. **用户体验差**: 用户需要记住不同屏幕下的不同快捷键
3. **认知负荷高**: 相同操作在不同界面有不同行为
4. **扩展性差**: 添加新功能需要修改全局映射表

## 🔄 解决方案：StatefulEventHandler

### 核心设计理念

**上下文感知的事件处理**: 同一个事件根据当前应用状态产生不同的命令

```rust
// 之前：静态映射
Enter -> Select  // 在所有屏幕都是同样的行为

// 现在：上下文感知
Enter in MainMenu -> Confirm
Enter in DeckSelection -> StartStudySession
Enter in StudySession (question) -> ShowAnswer
Enter in StudySession (answer) -> RateGood
Enter in CardEditor -> SaveCard
```

## 🏗️ 架构设计

### 1. 扩展的状态管理

```rust
pub struct AppState {
    // 基本状态
    pub current_screen: Screen,
    pub selected_deck_id: Option<Uuid>,
    pub current_session: Option<SessionState>,

    // 上下文状态
    pub sub_state: String,                    // "showing_answer", "editing", etc.
    pub ui_state: HashMap<String, String>,    // "card_study_mode": "active"
    pub last_action: Option<String>,
    pub action_history: Vec<String>,
}
```

### 2. StatefulEventHandler

```rust
pub struct StatefulEventHandler {
    current_state: AppState,
}

impl StatefulEventHandler {
    pub fn handle_event(&self, event: Event) -> TuiResult<Command> {
        match (event, current_state) {
            // 上下文感知的事件处理
        }
    }
}
```

## 📋 具体实现案例

### 1. Enter键的上下文处理

```rust
fn handle_select_contextual(&self, screen: Screen, sub_state: &str) -> TuiResult<Command> {
    match screen {
        Screen::MainMenu => Ok(Command::user(CommandType::Confirm)),
        Screen::DeckSelection => Ok(Command::user(CommandType::StartStudySessionDefault)),
        Screen::StudySession => {
            if self.current_state.is_showing_answer() {
                Ok(Command::user(CommandType::RateCurrentCard(Rating::Good)))
            } else {
                Ok(Command::user(CommandType::ShowAnswer))
            }
        },
        Screen::CardEditor => Ok(Command::user(CommandType::SaveCard)),
        Screen::Settings => Ok(Command::user(CommandType::ConfirmSetting)),
        _ => Ok(Command::user(CommandType::Select)),
    }
}
```

### 2. 数字键的学习模式限制

```rust
// 数字键只在Study Session中有效
(KeyCode::Char('1'), KeyModifiers::NONE) if screen == Screen::StudySession =>
    Ok(Command::user(CommandType::RateCurrentCard(Rating::Again))),

// 在其他屏幕中数字键被忽略或用于不同目的
(KeyCode::Char('1'), KeyModifiers::NONE) if screen == Screen::MainMenu =>
    Ok(Command::user(CommandType::Unknown)), // 或其他MainMenu相关命令
```

### 3. 鼠标点击的精确上下文

```rust
fn handle_left_click_contextual(&self, x: u16, y: u16, screen: Screen, _sub_state: &str) -> TuiResult<Command> {
    match screen {
        Screen::StudySession => {
            if self.current_state.is_showing_answer() {
                // 检查点击是否在评分按钮区域
                if y >= 10 && y <= 14 { // 评分按钮区域
                    let rating = match x {
                        10..=15 => Rating::Again,
                        17..=22 => Rating::Hard,
                        24..=29 => Rating::Good,
                        31..=36 => Rating::Easy,
                        _ => return Ok(Command::user(CommandType::ShowAnswer)),
                    };
                    Ok(Command::user(CommandType::RateCurrentCard(rating)))
                } else {
                    Ok(Command::user(CommandType::ShowAnswer))
                }
            } else {
                Ok(Command::user(CommandType::ShowAnswer))
            }
        },
        _ => Ok(Command::user(CommandType::Click(x, y))),
    }
}
```

## 🎮 用户体验改进

### 1. 直观的交互

| 事件 | 主菜单 | 牌组选择 | 学习会话 | 卡片编辑 |
|------|--------|----------|----------|----------|
| **Enter** | 确认选择 | 开始学习 | 显示答案/评分 | 保存卡片 |
| **Space** | - | 选择牌组 | 显示答案/评分 | 切换卡片面 |
| **Escape** | - | 返回主菜单 | 结束会话 | 取消编辑 |
| **1-4** | - | - | 评分卡片 | - |

### 2. 状态感知的快捷键

```rust
// 空格键在Study Session中的智能行为
match (screen, showing_answer) {
    (Screen::StudySession, false) => ShowAnswer,
    (Screen::StudySession, true)  => RateGood,
    (Screen::DeckSelection, _)    => SelectDeck,
}
```

### 3. 上下文相关的鼠标交互

- **Study Session**: 点击不同区域有不同含义
  - 评分按钮区域: 直接评分
  - 卡片区域: 显示答案
  - 其他区域: 显示答案

- **Deck Selection**: 右键显示上下文菜单
- **Card Editor**: 双击选择文本

## 🔧 技术实现

### 1. 状态更新机制

```rust
// 学习会话状态更新
pub fn set_showing_answer(&self, showing: bool) -> TuiResult<()> {
    self.update_state(|state| {
        if showing {
            state.set_ui_state("showing_answer".to_string(), "true".to_string());
            state.sub_state = "showing_answer".to_string();
        } else {
            state.set_ui_state("showing_answer".to_string(), "false".to_string());
            state.sub_state = "studying".to_string();
        }
    })
}
```

### 2. 事件处理流程

```rust
// 1. 获取当前状态
let current_state = state_store.get_state();

// 2. 创建状态感知处理器
let handler = StatefulEventHandler::new(current_state);

// 3. 处理事件
let command = handler.handle_event(event)?;

// 4. 执行命令
execute_command(command)?;
```

### 3. 上下文查询方法

```rust
impl AppState {
    pub fn is_study_session_active(&self) -> bool {
        self.current_session.is_some() && self.current_screen == Screen::StudySession
    }

    pub fn is_showing_answer(&self) -> bool {
        self.ui_state.get("showing_answer")
            .map(|s| s == "true")
            .unwrap_or(false)
    }
}
```

## 📊 优势对比

| 方面 | 原始设计 | Stateful设计 |
|------|----------|--------------|
| **用户体验** | 需要记住不同快捷键 | 直观的一致行为 |
| **认知负荷** | 高 - 不同界面不同规则 | 低 - 行为符合预期 |
| **扩展性** | 差 - 全局影响 | 好 - 局部上下文 |
| **维护性** | 复杂 - 全局映射表 | 简单 - 独立处理器 |
| **测试性** | 困难 - 复杂状态依赖 | 容易 - 独立上下文测试 |

## 🎯 最佳实践

### 1. 状态设计原则

- **最小化状态**: 只保留必要的上下文信息
- **清晰命名**: sub_state应该清楚地描述当前状态
- **状态转换**: 明确定义状态之间的转换规则

### 2. 事件处理原则

- **一致性**: 相似操作在相似上下文中有相似行为
- **可预测性**: 用户能够预测操作结果
- **容错性**: 无效操作不应该产生副作用

### 3. 扩展原则

- **新屏幕**: 添加新的屏幕类型和对应的事件处理逻辑
- **新功能**: 在现有屏幕中添加新的UI状态和事件处理
- **向后兼容**: 保持现有快捷键的行为一致性

## ✅ 总结

通过实现基于状态的Event处理，我们解决了原始设计的核心问题：

1. **用户体验提升**: 同一事件在不同上下文中有直观的行为
2. **认知负荷降低**: 用户不需要记住复杂的快捷键映射
3. **扩展性增强**: 新功能可以独立添加而不影响现有行为
4. **维护性改善**: 事件处理逻辑更加清晰和模块化

这个设计让AnkiTUI V2能够提供更加直观和用户友好的交互体验，符合现代TUI应用的最佳实践。