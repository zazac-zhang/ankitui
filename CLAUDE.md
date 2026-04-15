# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

AnkiTUI is a terminal-based spaced repetition learning system compatible with Anki's SM-2 algorithm. It's built with a 3-layer architecture: Core (business logic), Data (persistence), and TUI-V2 (modern terminal interface).

### Current Project Structure

```
ankitui/
├── ankitui-core/           # 核心业务逻辑层
│   ├── src/
│   │   ├── core/          # SM-2算法、牌组管理、会话控制等
│   │   ├── data/          # 数据持久化、模型定义
│   │   └── config/        # 配置管理
│   └── Cargo.toml
├── ankitui-tui/        # 现代化终端UI层
│   ├── src/
│   │   ├── ui/
│   │   │   └── components/screens/  # 页面组件
│   │   │       ├── common/          # 通用对话框等
│   │   │       ├── study/           # 学习相关页面
│   │   │       ├── deck/            # 牌组管理页面
│   │   │       ├── stats/           # 统计页面
│   │   │       ├── settings/        # 设置页面
│   │   │       └── menu/            # 主菜单
│   │   ├── domain/                  # 应用状态和视图模型
│   │   │   ├── app_state.rs        # UI应用状态定义
│   │   │   └── viewmodels.rs       # UI视图模型
│   │   └── lib.rs
│   └── Cargo.toml
├── ankitui/                 # 主应用程序入口
├── docs/                    # 文档目录
└── Cargo.toml              # Workspace配置
```

## Build and Development Commands

```bash
# 构建整个项目
cargo build

# 构建优化版本
cargo build --release

# 运行测试
cargo test

# 运行测试并显示输出
cargo test -- --nocapture

# 检查代码（不构建）
cargo check

# 快速检查错误
cargo check --message-format short | grep "error"

# 格式化代码
cargo fmt

# 运行代码检查
cargo clippy

# 运行应用程序
cargo run

# 运行特定包的测试
cargo test -p ankitui-core
cargo test -p ankitui-tui

# 检查特定包的编译
cargo check -p ankitui-core
cargo check -p ankitui-tui

# 清理构建产物
cargo clean

```

## Project Architecture

## 项目结构

1、合理拆分 crate，保持模块职责单一
2、mod.rs 和 lib.rs 文件要包含模块说明文档
3、先查看现有依赖的功能，避免重复实现
4、workspace 内的 crate 导入导出都要显式声明

## 代码规范

1、合理使用派生宏，减少样板代码
2、优先使用第三方库，其次 workspace 内 crate，最后自己实现
3、避免过度使用 getter/setter，优先考虑直接访问字段或使用构建器模式

## 测试策略

1、只编写核心业务逻辑的必要测试，减少测试函数，避免过度测试

## 调试修复

1、使用 `cargo check --message-format short | grep error` 快速定位关键错误
2、大量错误通常由同一根本问题引起，需深度分析，然后修复问题

## 编译构建

1、只关注编译错误，忽略 lint 警告

## 终端限制

1、终端环境不适合实现文本编辑功能（输入框、光标移动、复杂表单等），应避免实现此类功能
2、对于需要文本输入的场景，使用以下替代方案：
   - 预定义选项/下拉选择
   - 数值调整（←→增减）
   - 布尔切换（Enter 开关）
   - 命令行参数/配置文件
3、CardEditor 等需要复杂输入的功能不应在 TUI 中实现，考虑通过配置文件或外部工具处理

## TUI 开发准则

1、**渲染闭内禁止阻塞 I/O**：render 回调中不得调用 `block_in_place`/`block_on` 执行数据库或网络请求，数据应预取到 AppState
2、**导航状态单一数据源**：不得同时维护多套 navigation history，统一使用 StateStore
3、**列表边界基于动态长度**：导航上限应从实际数据长度计算，不得在事件处理和渲染中各自硬编码
4、**鼠标坐标不得写死绝对值**：点击区域应通过 Layout chunks 计算相对位置，而非硬编码行列范围
5、**终端退出必须用 Drop 守卫**：enable_raw_mode/EnterAlternateScreen 必须配对 Drop guard 保证 panic 时也能恢复
6、**Emoji 宽度不可假设**：终端中 Emoji 占位宽度不一致，表格/列表对齐应预留弹性或使用替代符号
7、**长列表必须维护滚动状态**：使用 `ListState` 跟踪选中/滚动偏移，否则超出视口的项无法访问
8、**状态更新与渲染必须同步**：状态变更后立即触发重绘并给出视觉反馈（高亮/箭头指示器）
9、**快捷键冲突应在全局统一处理**：同一快捷键在不同屏幕下行为不得冲突，全局事件处理器应覆盖所有场景
10、**读取状态应批量获取**：一次 lock 读取所有需要的字段，避免重复 acquire 同一 RwLock
