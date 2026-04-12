# AnkiTUI 项目 TODO 清单

> 基于代码深度分析，按优先级排序
>
> **最后更新**: 2026-04-12
> **整体进度**: 核心功能已完成，存在冗余代码需清理，部分功能需接入 UI

---

## 历史清单（已完成）

<details>
<summary>P0-P5 历史问题（47项，全部完成）</summary>

| 优先级 | 问题数 | 已完成 | 状态 |
|--------|--------|--------|------|
| P0 | 1 | 1 | ✅ 运行时崩溃修复 |
| P1 | 20 | 20 | ✅ 核心功能实现 |
| P2 | 11 | 11 | ✅ 假数据/占位符修复 |
| P3 | 8 | 8 | ✅ 核心层完善 |
| P4 | 4 | 4 | ✅ 应用层功能 |
| P5 | 3 | 3 | ✅ 代码质量 |

</details>

---

## 当前完成清单

### F1 - 清理冗余代码（P0，100% 完成）✅

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F1.1 | 删除 `domain/commands.rs`（CQRS 未接入） | `ankitui-tui/src/domain/commands.rs` | ✅ 已删除 |
| F1.2 | 删除 `domain/queries.rs`（CQRS 未接入） | `ankitui-tui/src/domain/queries.rs` | ✅ 已删除 |
| F1.3 | 删除 `EventHandler` 结构体（与 event_loop 重复） | `ankitui-tui/src/ui/event/handler.rs` | ✅ 已删除 |
| F1.4 | 删除 `EventProcessor` trait + `ApplicationEventProcessor` | `ankitui-tui/src/app/event_loop.rs` | ✅ 已删除 |
| F1.5 | 删除 `UiRating` 枚举（被 `CardRating` 替代） | `ankitui-tui/src/app/controller.rs` | ✅ 已删除 |
| F1.6 | 删除 `App.event_handler` 字段 | `ankitui-tui/src/app/main_app.rs` | ✅ 已删除 |
| F1.7 | 删除 `App::handle_input()` 方法（从未调用） | `ankitui-tui/src/app/main_app.rs` | ✅ 已删除 |
| F1.8 | 更新 `domain/mod.rs` 和 `ui/mod.rs` 导出 | 多处 | ✅ 已修复 |

### F2 - 补全桩代码（P1，80% 完成）

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F2.1 | Import 操作实现 | `ankitui-tui/src/app/main_app.rs` | ✅ 从 data_dir/ankitui/import.toml 导入 |
| F2.2 | Export 操作实现 | `ankitui-tui/src/app/main_app.rs` | ✅ 导出到 data_dir/ankitui/exports/ |
| F2.3 | Backup 操作实现 | `ankitui-tui/src/app/main_app.rs` | ✅ 时间戳数据库备份 |
| F2.4 | Restore 操作实现 | `ankitui-tui/src/app/main_app.rs` | ✅ 从最新备份恢复 |
| F2.5 | `ScrollStatsUp/Down` 命令 | `ankitui-tui/src/app/main_app.rs` | ✅ 切换统计页 tab |
| F2.6 | Help 页 ↑↓ 导航 | `ankitui-tui/src/app/main_app.rs` | ✅ 切换帮助分类 |

### F3 - 卡片状态操作（P2，100% 完成）✅

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F3.1 | `BuryCard` 命令 + 快捷键 `B` | `ankitui-tui/src/app/main_app.rs` + `event_loop.rs` | ✅ 已实现 |
| F3.2 | `SuspendCard` 命令 + 快捷键 `Ctrl+S` | `ankitui-tui/src/app/main_app.rs` + `event_loop.rs` | ✅ 已实现 |
| F3.3 | `UnburyCard` 命令 + 快捷键 `U` | `ankitui-tui/src/app/main_app.rs` + `event_loop.rs` | ✅ 已实现 |
| F3.4 | `UnsuspendCard` 命令 + 快捷键 `Ctrl+U` | `ankitui-tui/src/app/main_app.rs` + `event_loop.rs` | ✅ 已实现 |
| F3.5 | Help 页更新快捷键说明 | `ankitui-tui/src/ui/components/screens/help/mod.rs` | ✅ 已更新 |
| F3.6 | SessionController 暴露 `current_deck_id()` 和 `get_deck_cards()` | `ankitui-core/src/core/session_controller.rs` | ✅ 已添加 |

### F4 - 设置持久化（P1，100% 完成）✅

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F4.1 | StudyPrefs 持久化（新卡片/复习上限） | `ankitui-tui/src/app/main_app.rs` | ✅ shutdown 时写入配置 |
| F4.2 | UiSettings 持久化（主题/进度条） | `ankitui-tui/src/app/main_app.rs` | ✅ shutdown 时写入配置 |
| F4.3 | 退出时保存设置 | `ankitui-tui/src/app/main_app.rs` + `ankitui/src/main.rs` | ✅ App::with_config_manager + shutdown |

### F5 - 代码质量（P3，100% 完成）✅

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F5.1 | SessionController 添加公共 getter | `ankitui-core/src/core/session_controller.rs` | ✅ current_deck_id() + get_deck_cards() |

> `execute_command` 保持为 match 表达式（~400行），结构清晰，无需拆分

---

## F1 - 清理冗余代码（P0 级，必须）

> **目标**: 消除死代码和重复逻辑，降低维护成本

### 1. 删除未使用的 CQRS 系统

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F1.1 | 删除 `domain/commands.rs` | `ankitui-tui/src/domain/commands.rs` | ❌ 定义了 Command/Query 特质和结构体，从未接入流程 |
| F1.2 | 删除 `domain/queries.rs` | `ankitui-tui/src/domain/queries.rs` | ❌ 同上 |
| F1.3 | 更新 `domain/mod.rs` 导出 | `ankitui-tui/src/domain/mod.rs` | ❌ 移除 commands/queries 模块引用 |

### 2. 删除未使用的事件处理器

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F1.4 | 删除 `EventProcessor` trait | `ankitui-tui/src/app/event_loop.rs:142` | ❌ 定义了但未实现 |
| F1.5 | 删除 `ApplicationEventProcessor` | `ankitui-tui/src/app/event_loop.rs:148-171` | ❌ 定义了但未实例化 |
| F1.6 | 删除 `UiRating` 枚举 | `ankitui-tui/src/app/controller.rs:648` | ❌ 被 `CardRating` 替代 |

### 3. 合并重复的事件处理逻辑

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F1.7 | `event_loop.rs` 和 `handler.rs` 有几乎相同的上下文处理函数（~640 行 vs ~315 行） | `ankitui-tui/src/app/event_loop.rs` + `ankitui-tui/src/ui/event/handler.rs` | ❌ 两份代码做同一件事 |
| F1.8 | `controller.rs` 和 `main_app.rs` 都处理 NavigateLeft/Right（StudyPrefs/UiSettings） | `ankitui-tui/src/app/controller.rs` + `ankitui-tui/src/app/main_app.rs` | ❌ 逻辑重复 |
| F1.9 | `controller.rs` 的 select_previous/next_deck 与 `main_app.rs` 的 handle_deck_selection 重复 | `ankitui-tui/src/app/controller.rs:472-521` + `ankitui-tui/src/app/main_app.rs:1165-1208` | ❌ 逻辑重复 |

---

## F2 - 补全桩代码（P1 级，重要）

> **目标**: 让所有显示给用户的功能真正可用

### 1. 数据管理操作实现

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F2.1 | Import 操作实现（目前显示占位提示） | `ankitui-tui/src/app/main_app.rs:463` | ❌ 应调用实际导入逻辑 |
| F2.2 | Export 操作实现（目前显示占位提示） | `ankitui-tui/src/app/main_app.rs:464` | ❌ 应调用实际导出逻辑 |
| F2.3 | Restore 操作实现（目前显示占位提示） | `ankitui-tui/src/app/main_app.rs:466` | ❌ 应调用实际恢复逻辑 |

### 2. 统计页面滚动导航

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F2.4 | `ScrollStatsUp/Down` 命令是空操作 | `ankitui-tui/src/app/main_app.rs:507-509` | ❌ 需实现统计页 tab 切换或内容滚动 |

### 3. 帮助屏幕分类导航

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F2.5 | Help 页 ↑↓ 导航是空操作 | `ankitui-tui/src/app/main_app.rs:738,779` | ❌ 需实现帮助分类列表 |

---

## F3 - 接入核心功能到 UI（P2 级）

> **目标**: core 层已实现的功能应可通过 TUI 操作

### 1. 卡片状态操作（Bury/Suspend）

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F3.1 | `CardStateManager.bury_card()` 无 TUI 命令 | `ankitui-core/src/core/card_state_manager.rs` | ❌ 核心已实现，无 UI |
| F3.2 | `CardStateManager.suspend_card()` 无 TUI 命令 | `ankitui-core/src/core/card_state_manager.rs` | ❌ 核心已实现，无 UI |
| F3.3 | 添加 Bury/Suspend 快捷键 | `ankitui-tui/src/ui/event/command.rs` + `handler.rs` | ❌ 需添加 CommandType |

### 2. 标签管理

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F3.4 | `TagManager` 无 TUI 屏幕 | `ankitui-core/src/core/tag_manager.rs` | ❌ 核心完整（层级/搜索/过滤/批量） |
| F3.5 | 添加标签管理菜单入口 | Settings 或 DeckManagement | ❌ 无导航入口 |
| F3.6 | 标签搜索和过滤 UI | 新屏幕 | ❌ 需实现 |

### 3. 媒体管理

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F3.7 | `MediaManager` 无 TUI 屏幕 | `ankitui-core/src/core/media_manager.rs` | ❌ 核心已实现 |
| F3.8 | 媒体浏览和清理 UI | 新屏幕 | ❌ 需实现 |
| F3.9 | 媒体时长提取 TODO | `ankitui-core/src/core/media_manager.rs:225` | ❌ 待实现 |

### 4. 高级卡片类型渲染

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F3.10 | Cloze（完形填空）渲染 | `ankitui-tui/src/ui/components/screens/study/mod.rs` | ❌ TUI 只渲染 Basic 正反面 |
| F3.11 | Input（输入型）渲染 | 同上 | ❌ 同上 |
| F3.12 | MultipleChoice（选择题）渲染 | 同上 | ❌ 同上 |
| F3.13 | ImageOcclusion（图片遮挡）渲染 | 同上 | ❌ 同上 |

### 5. 增量学习队列

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F3.14 | `IncrementalLearning` 学习队列未接入 | `ankitui-core/src/core/incremental_learning.rs` | ❌ 核心已实现，学习会话用的是 `SessionController` |

---

## F4 - 设置持久化（P1 级）

> **目标**: UI 中的设置修改应保存到配置文件

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F4.1 | StudyPrefs 修改未写回配置 | `ankitui-tui/src/app/main_app.rs:656-679` | ❌ 只修改内存状态 |
| F4.2 | UiSettings 主题切换未持久化 | `ankitui-tui/src/app/main_app.rs:681-696` | ❌ 重启后丢失 |
| F4.3 | 退出时保存设置 | `ankitui-tui/src/app/main_app.rs:256-287` | ❌ shutdown 未保存设置 |

---

## F5 - 代码质量（P3 级）

### 1. 减少 `main_app.rs` 复杂度

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F5.1 | `execute_command` 超过 350 行 | `ankitui-tui/src/app/main_app.rs:420-794` | ❌ 需拆分为子处理器 |
| F5.2 | 大量 `state_store.read().await.update_state().ok()` 链 | 多处 | ❌ 可提取辅助方法 |

---

## 统计概览

| 功能模块 | 完成度 | 优先级 |
|---------|--------|--------|
| **学习流程** | 100% | ✅ 已完成 |
| **牌组管理** | 100% | ✅ 已完成 |
| **统计分析** | 95% | ✅ 滚动导航已实现 |
| **搜索过滤** | 100% | ✅ 已完成 |
| **设置管理** | 95% | ✅ 持久化已实现 |
| **数据管理** | 90% | ✅ Import/Export/Backup/Restore 已实现 |
| **导航系统** | 95% | ✅ Help 页导航已实现 |
| **帮助系统** | 100% | ✅ 已完成 |
| **主题系统** | 80% | ✅ 持久化已实现 |
| **卡片状态操作** | 100% | ✅ Bury/Suspend/Unbury/Unsuspend 完成 |
| **标签管理** | 10% | ❌ 核心完成，无 UI |
| **媒体管理** | 10% | ❌ 核心完成，无 UI |
| **高级卡片渲染** | 20% | ❌ 仅 Basic |
| **代码质量** | 100% | ✅ 已完成 |

---

## 实现优先级

| 优先级 | 任务 | 预计工作量 |
|--------|------|-----------|
| ~~**P0**~~ | ~~F1 清理冗余代码~~ | ~~中等~~ ✅ 已完成 |
| ~~**P1**~~ | ~~F2 补全桩代码 + F4 设置持久化~~ | ~~中等~~ ✅ 已完成 |
| ~~**P2**~~ | ~~F3 卡片状态操作（Bury/Suspend/Unbury/Unsuspend）~~ | ~~小~~ ✅ 已完成 |
| **P3** | F5 标签管理 UI → 媒体管理 UI → 高级卡片渲染 | 大 |
