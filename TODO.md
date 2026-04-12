# AnkiTUI 项目 TODO 清单

> 基于代码深度分析，按优先级排序

---

## P0 - 运行时崩溃风险

| # | 问题 | 文件 | 说明 |
|---|------|------|------|
| 1 | `todo!()` 宏会导致 panic | `ankitui-core/src/core/tag_manager.rs:596` | 测试辅助函数 `create_test_card()` 中 `updated_at: todo!()`，运行相关测试时会直接 panic |

---

## P1 - 核心功能缺失

### 1. 学习会话服务完全未实现

| # | 问题 | 文件 | 说明 |
|---|------|------|------|
| 2 | `StudyService::start_session()` 空实现 | `ankitui-tui/src/domain/services.rs:103` | 直接返回 `Ok(())`，没有实际启动会话 |
| 3 | `StudyService::end_session()` 返回空数据 | `ankitui-tui/src/domain/services.rs:109` | 返回空的 `StudySessionStats`，无实际会话统计 |
| 4 | `StudyService::rate_current_card()` 空实现 | `ankitui-tui/src/domain/services.rs:115` | 直接返回 `Ok(())`，没有调用 `SessionController` 评分 |
| 5 | `StudyService::skip_current_card()` 空实现 | `ankitui-tui/src/domain/services.rs:121` | 直接返回 `Ok(())`，没有跳过卡片逻辑 |

**影响**: 整个学习流程的核心操作（开始/结束/评分/跳过）全部是空壳，用户操作后没有实际效果。

### 2. 所有设置/统计页面显示 "To be implemented"

| # | 问题 | 文件 | 说明 |
|---|------|------|------|
| 6 | `SettingsScreen` 未实现 | `ankitui-tui/src/ui/components/screens/settings/mod.rs` | 仅渲染 "(To be implemented)" |
| 7 | `StudyPrefsScreen` 未实现 | 同上 | 仅渲染 "(To be implemented)" |
| 8 | `UiSettingsScreen` 未实现 | 同上 | 仅渲染 "(To be implemented)" |
| 9 | `DataManageScreen` 未实现 | 同上 | 仅渲染 "(To be implemented)" |
| 10 | `StatsScreen` 未实现 | `ankitui-tui/src/ui/components/screens/stats/mod.rs` | 仅渲染 "(To be implemented)" |
| 11 | `GlobalStatsScreen` 未实现 | 同上 | 仅渲染 "(To be implemented)" |
| 12 | `DeckStatsScreen` 未实现 | 同上 | 仅渲染 "(To be implemented)" |
| 13 | `ProgressScreen` 未实现 | 同上 | 仅渲染 "(To be implemented)" |
| 14 | `DeckEditScreen` 未实现 | `ankitui-tui/src/ui/components/screens/deck/edit.rs` | 仅渲染 "(To be implemented)" |
| 15 | `DeckManageScreen` 未实现 | `ankitui-tui/src/ui/components/screens/deck/manage.rs` | 仅渲染 "(To be implemented)" |
| 16 | `MenuScreen` 占位符 | `ankitui-tui/src/ui/components/screens/menu/mod.rs` | 占位实现 |

### 3. 基础 Widget 组件未实现

| # | 问题 | 文件 | 说明 |
|---|------|------|------|
| 17 | `Input` widget 不接受输入 | `ankitui-tui/src/ui/components/widgets/mod.rs` | `handle_input` 返回 `Ok(false)`，无法输入文本 |
| 18 | `List` widget 无键盘导航 | 同上 | `handle_input` 返回 `Ok(false)`，无法用键盘选择 |
| 19 | `Table` widget 空结构体 | 同上 | 只有 `pub struct Table;`，无实现 |
| 20 | `Dialog` widget 空结构体 | 同上 | 只有 `pub struct Dialog;`，无实现 |

**影响**: 没有可用的交互式表单输入组件，无法实现设置编辑、对话框等功能。

---

## P2 - 功能不完整/数据造假

### 1. 渲染层使用硬编码假数据

| # | 问题 | 文件 | 说明 |
|---|------|------|------|
| 21 | `render_deck_selection()` 使用硬编码示例数据 | `ankitui-tui/src/ui/render/mod.rs:225-231` | 5 个写死的示例牌组 |
| 22 | `render_deck_selection_enhanced()` 同样硬编码 | 同上:355-362 | 6 个写死的示例牌组 |
| 23 | `render_study_session()` 使用假卡片内容 | 同上:447-454 | "What is the Pythagorean theorem?" |
| 24 | `render_statistics()` 全部硬编码 | 同上:505-531 | 假统计数据、假保留率 |
| 25 | `render_settings()` 硬编码菜单项 | 同上:595-602 | 设置项无法交互 |
| 26 | 保留率计算使用 UUID 哈希造假 | 同上:943 | `85.0 + (deck.uuid.to_string().chars().next()... % 20)` |
| 27 | 平均学习时间使用 UUID 哈希造假 | 同上:947 | 同样从 UUID 派生的假值 |

**注意**: `_with_real_data` 变体确实调用了真实服务，但仍然存在假数据计算（保留率、平均时间），且与假数据版本共存造成维护混乱。

### 2. ViewModel 占位逻辑

| # | 问题 | 文件 | 说明 |
|---|------|------|------|
| 28 | `DeckViewModel::has_due_cards()` 逻辑错误 | `ankitui-tui/src/domain/viewmodels.rs:50` | 返回 `self.deck.description.is_some()`，与是否有到期卡片无关 |
| 29 | `DeckViewModel::has_new_cards()` 硬返回 true | 同上:56 | 永远返回 `true` |

### 3. 状态选择器硬编码

| # | 问题 | 文件 | 说明 |
|---|------|------|------|
| 30 | `MainMenuStateSelector::select()` 硬编码 `has_decks: true` | `ankitui-tui/src/ui/state/selector.rs:146` | 应检查实际牌组数量 |

### 4. 导航器占位

| # | 问题 | 文件 | 说明 |
|---|------|------|------|
| 31 | `Navigator::current_deck()` 返回 None | `ankitui-tui/src/ui/navigator/mod.rs:80` | 占位实现 |

---

## P3 - 核心层不完善

### 1. 会话持久化/恢复未实现

| # | 问题 | 文件 | 说明 |
|---|------|------|------|
| 32 | `save_session_state()` 未实现持久化 | `ankitui-core/src/core/session_controller.rs:761` | TODO: "Implement actual persistence of recovery data" |
| 33 | `recover_session()` 永远返回 false | `ankitui-core/src/core/session_controller.rs:769` | TODO: "Load session recovery data from storage" |

### 2. 媒体管理不完整

| # | 问题 | 文件 | 说明 |
|---|------|------|------|
| 34 | `add_media_from_url()` 未实现下载 | `ankitui-core/src/core/media_manager.rs:107` | TODO: "Implement URL download functionality" |
| 35 | `extract_metadata()` 未实现元数据提取 | 同上:220-221 | 时长、尺寸提取为 TODO |
| 36 | `validate_media()` 验证不完整 | 同上:288 | TODO: "Add more validation based on media type" |

### 3. 统计引擎不完整

| # | 问题 | 文件 | 说明 |
|---|------|------|------|
| 37 | `suspended_cards` 和 `buried_cards` 硬编码为 0 | `ankitui-core/src/core/stats_engine.rs:284-285` | TODO 注释 |
| 38 | `average_study_time_per_card` 为 0 | 同上:293 | 未追踪单卡片学习时间 |
| 39 | `longest_study_streak` 为 0 | 同上:304 | 未实现连续学习追踪 |
| 40 | 时间线数据为模拟生成 | 同上 | 非真实历史数据 |

### 4. 卡片状态管理

| # | 问题 | 文件 | 说明 |
|---|------|------|------|
| 41 | `get_card_status()` 返回占位符 `Active` | `ankitui-core/src/core/card_state_manager.rs:146` | 未检查数据库中的实际卡片状态 |

### 5. 数据同步适配器

| # | 问题 | 文件 | 说明 |
|---|------|------|------|
| 42 | `update_card_content()` 为空实现 | `ankitui-core/src/data/sync_adapter.rs:493` | TODO: "For now, this is a placeholder implementation" |

---

## P4 - 应用层缺失功能

### 1. 数据导入/导出

| # | 问题 | 文件 | 说明 |
|---|------|------|------|
| 43 | `App::export_data()` 未实现 | `ankitui-tui/src/app/main_app.rs:807` | 仅记录日志，无实际导出 |
| 44 | `App::import_data()` 未实现 | `ankitui-tui/src/app/main_app.rs:821` | 仅调用 `force_refresh()` |

### 2. 状态持久化

| # | 问题 | 文件 | 说明 |
|---|------|------|------|
| 45 | `App::save_state()` 空实现 | `ankitui-tui/src/app/main_app.rs:341` | 注释说"Implementation would save preferences to storage" |
| 46 | `App::load_state()` 空实现 | `ankitui-tui/src/app/main_app.rs:353` | 注释说"Implementation would load preferences from storage" |

---

## P5 - 代码质量改进

| # | 问题 | 说明 |
|---|------|------|
| 47 | 渲染层存在两套实现（假数据 + 真实数据），应统一 | `render/mod.rs` 中 `render_*` 和 `render_*_with_real_data` 共存 |
| 48 | 学习服务的方法与 `SessionController` 未真正连接 | `StudyService` 持有 `session_controller` 但未在 `start_session`/`rate_current_card` 中使用 |
| 49 | CLI 模式模拟学习会话为占位实现 | `ankitui/src/util/cli.rs:652` 仅 sleep 并打印 |

---

## 统计概览

| 优先级 | 问题数 | 类别 |
|--------|--------|------|
| P0 | 1 | 运行时崩溃 |
| P1 | 20 | 核心功能缺失 |
| P2 | 11 | 功能不完整/假数据 |
| P3 | 11 | 核心层不完善 |
| P4 | 4 | 应用层缺失功能 |
| P5 | 3 | 代码质量 |
| **合计** | **50** | |

## 已完成且运行良好的模块

- SM-2 调度算法（核心算法完整）
- 牌组管理器（CRUD 操作完整）
- 卡片模板引擎（模板解析和替换完整）
- 数据同步适配器（内容/状态合并完整）
- 内容存储（TOML 基础存储完整）
- 状态存储（SQLite 持久化完整）
- 配置系统（配置文件、验证、模板完整）
- 增量学习（优先级队列管理完整）
- 标签管理器（层级、搜索、过滤、操作完整）
- 学习页面（Question/Answer/Rating/Finished 页面渲染真实数据）
- 主应用框架（事件处理、命令分发、导航流程完整）
