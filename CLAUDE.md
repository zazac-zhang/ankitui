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
