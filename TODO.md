# AnkiTUI 项目 TODO 清单

> 基于代码深度分析，按优先级排序
>
> **最后更新**: 2026-04-12
> **整体进度**: 核心功能全部完成，F9批量操作和CardEditor（终端不适合编辑）暂不实现

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

## F1 - 核心功能可用性（P0 级，必须实现）

> **目标**: 确保 TUI 界面的每个功能都可实际操作，而不是仅显示菜单

### 1. 学习设置管理 (100% 完成) ✅

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F1.1 | `StudyPrefsScreen` 交互逻辑 | `ankitui-tui/src/ui/components/screens/settings/mod.rs` | ✅ 已完成 |
| F1.2 | 每日新卡片上限设置 | - | ✅ ↑↓导航, ←→调整数值 |
| F1.3 | 每日复习卡片上限设置 | - | ✅ ↑↓导航, ←→调整数值 |
| F1.4 | 自动前进设置 | - | ✅ Enter 切换布尔值 |

### 2. 牌组编辑功能 (100% 完成) ✅

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F2.1 | `DeckEditScreen` 交互逻辑 | `ankitui-tui/src/ui/components/screens/deck/edit.rs` | ✅ 已完成 |
| F2.2 | 重命名牌组 | - | ✅ 导航+状态显示 |
| F2.3 | 编辑牌组描述 | - | ✅ 导航+状态显示 |
| F2.4 | 编辑牌组调度配置 | - | ✅ ←→调整数值, Ctrl+S保存 |

### 3. 数据管理交互 (100% 完成) ✅

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F3.1 | `DataManageScreen` 交互逻辑 | `ankitui-tui/src/ui/components/screens/settings/mod.rs` | ✅ 已完成 |
| F3.2 | 导入数据到当前牌组 | - | ✅ 菜单导航+状态反馈 |
| F3.3 | 导出数据到文件 | - | ✅ 菜单导航+状态反馈 |
| F3.4 | 备份当前数据 | - | ✅ 菜单导航+状态反馈 |

### 4. UI 设置交互 (100% 完成) ✅

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F4.1 | `UiSettingsScreen` 交互逻辑 | `ankitui-tui/src/ui/components/screens/settings/mod.rs` | ✅ 已完成 |
| F4.2 | 显示名设置 | - | ✅ 导航+状态显示 |
| F4.3 | 显示/隐藏进度条 | - | ✅ Enter 切换, ←→切换主题 |

---

## F2 - 用户体验提升（P1 级，重要）

### 1. 搜索功能 (100% 完成) ✅

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F5.1 | 搜索屏幕实现 | `ankitui-tui/src/ui/components/screens/search/mod.rs` | ✅ 已完成 |
| F5.2 | 牌组搜索界面 | - | ✅ 导航+切换类型+输入 |
| F5.3 | 卡片搜索界面 | - | ✅ Tab 切换搜索类型 |

### 2. 统计可视化 (100% 完成) ✅

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F6.1 | `ProgressScreen` 内容 | `ankitui-tui/src/ui/components/screens/stats/mod.rs` | ✅ 已完成 |
| F6.2 | 学习进度图表 | - | ✅ Overview/Retention/Forecast 三个 tab |
| F6.3 | 趋势分析展示 | - | ✅ 学习天数、留存率、预测 |

### 3. 牌组管理增强 (100% 完成) ✅

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F7.1 | `DeckManageScreen` 操作连接 | `ankitui-tui/src/ui/components/screens/deck/manage.rs` | ✅ 已完成 |
| F7.2 | 导出牌组功能 | - | ✅ 导航+状态反馈 |
| F7.3 | 删除牌组确认流程 | - | ✅ 菜单导航+执行反馈 |
| F7.4 | 牌组统计查看 | - | ✅ 导航+状态反馈 |

---

## F3 - 高级功能（P2 级，部分完成）

### 1. 主题系统 (60% 完成)

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F8.1 | 主题定义 | `ankitui-tui/src/ui/theme.rs` | ✅ ColorScheme for 3 themes |
| F8.2 | 亮色/暗色主题 | - | ✅ Default/Dark/Light color schemes |

### 2. 批量操作 (10% 完成)

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F9.1 | 多选卡片功能 | - | ❌ 未实现 |
| F9.2 | 批量删除卡片 | - | ❌ 未实现 |
| F9.3 | 批量导出牌组 | - | ❌ 未实现 |

### 3. 帮助系统 (100% 完成) ✅

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F10.1 | F1 帮助绑定 | `ankitui-tui/src/ui/event/handler.rs` | ✅ 已有快捷键 |
| F10.2 | 帮助内容显示 | `ankitui-tui/src/ui/components/screens/help/mod.rs` | ✅ 已完成 |
| F10.3 | 帮助屏幕导航接入 | `ankitui-tui/src/ui/state/store.rs` + `render/mod.rs` | ✅ Screen::Help + render + Esc 返回 |

### 4. Settings 子屏幕接入 (100% 完成) ✅

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F11.1 | ConfirmSetting 命令处理 | `ankitui-tui/src/app/controller.rs` | ✅ Enter 根据索引跳转 |
| F11.2 | StudyPrefs 子屏幕 | `ankitui-tui/src/ui/render/mod.rs` | ✅ render_study_prefs |
| F11.3 | UiSettings 子屏幕 | `ankitui-tui/src/ui/render/mod.rs` | ✅ render_ui_settings |
| F11.4 | DataManage 子屏幕 | `ankitui-tui/src/ui/render/mod.rs` | ✅ render_data_manage |
| F11.5 | ←→ 调整数值 | `ankitui-tui/src/app/controller.rs` | ✅ NavigateLeft/Right 处理 |
| F11.6 | Esc 返回 Settings | `ankitui-tui/src/app/controller.rs` | ✅ NavigateBack 处理 |

### 5. 搜索屏幕输入接入 (100% 完成) ✅

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F12.1 | 字符输入累积 | `ankitui-tui/src/app/main_app.rs` | ✅ SearchDecks/Backspace 处理 |
| F12.2 | Tab 切换搜索类型 | `ankitui-tui/src/app/main_app.rs` | ✅ ToggleCardSide |
| F12.3 | 搜索结果显示 | `ankitui-tui/src/ui/render/mod.rs` | ✅ 实时过滤牌组/卡片 |
| F12.4 | Backspace 删除 | `ankitui-tui/src/app/main_app.rs` | ✅ SearchBackspace 命令 |

### 6. 数据管理操作反馈 (100% 完成) ✅

| # | 问题 | 文件 | 状态 |
|---|------|------|------|
| F13.1 | Enter 执行操作 | `ankitui-tui/src/app/main_app.rs` | ✅ Confirm 处理 DataManage |
| F13.2 | 操作结果反馈 | `ankitui-tui/src/app/main_app.rs` | ✅ SystemMessage 提示 |
| F13.3 | UiSettings Enter 切换 | `ankitui-tui/src/app/main_app.rs` | ✅ 布尔值切换 |

### 7. 卡片编辑器 (不实现)

> 终端环境不适合实现复杂的文本编辑功能（光标移动、插入/删除、多行输入等）
> 替代方案：通过配置文件或 CLI 工具管理卡片

---

## 统计概览

| 功能模块 | 完成度 | 优先级 |
|---------|--------|--------|
| **学习流程** | 100% | ✅ 已完成 |
| **牌组管理** | 100% | ✅ 已完成 |
| **统计分析** | 100% | ✅ 已完成 |
| **搜索过滤** | 100% | ✅ 已完成 |
| **设置管理** | 100% | ✅ 已完成 |
| **数据管理** | 100% | ✅ 已完成 |
| **导航系统** | 100% | ✅ 已完成 |
| **帮助系统** | 100% | ✅ 已完成 |
| **主题系统** | 60% | 已移出TODO |
| **卡片编辑** | - | 不实现（终端限制） |
| **批量操作** | 10% | F9 暂不实现 |

---

## 实现优先级

| 优先级 | 任务 | 预计工作量 |
|--------|------|-----------|
| **P0** | F1 (设置管理) + F2 (牌组编辑) + F3 (数据管理) + F4 (UI设置) | 4个功能 |
| **P1** | F5 (搜索) + F6 (统计可视化) + F7 (牌组管理增强) | 3个功能 |
| **P2** | F8 (主题) + F9 (批量操作) + F10 (帮助) | 3个功能 |
