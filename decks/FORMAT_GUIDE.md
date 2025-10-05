# AnkiTUI 卡片组格式说明文档

本文档详细说明了 AnkiTUI 支持的卡片组文件格式，包括 TOML 和 JSON 两种格式的字段要求和最佳实践。

## 目录

1. [通用字段说明](#通用字段说明)
2. [TOML 格式规范](#toml-格式规范)
3. [JSON 格式规范](#json-格式规范)
4. [字段详细说明](#字段详细说明)
5. [最佳实践](#最佳实践)
6. [常见错误](#常见错误)

## 通用字段说明

### 卡片组 (Deck) 必需字段

| 字段名 | 类型 | 说明 | 示例 |
|--------|------|------|------|
| `name` | String | 卡片组名称，显示在界面上 | `"macOS 常用快捷键"` |
| `uuid` | String | 唯一标识符，格式为标准UUID | `"550e8400-e29b-41d4-a716-446655440100"` |

### 卡片组 (Deck) 可选字段

| 字段名 | 类型 | 说明 | 默认值 |
|--------|------|------|--------|
| `description` | String | 卡片组描述 | `None` |
| `created_at` | String | 创建时间 (ISO 8601格式) | 当前时间 |
| `modified_at` | String | 修改时间 (ISO 8601格式) | 当前时间 |

### 卡片 (Card) 必需字段

| 字段名 | 类型 | 说明 | 示例 |
|--------|------|------|------|
| `id` | String | 卡片唯一标识符，格式为标准UUID | `"550e8400-e29b-41d4-a716-446655440101"` |
| `front` | String | 卡片正面，通常是问题或提示 | `"如何在 macOS 中复制选中的文本？"` |
| `back` | String | 卡片背面，通常是答案 | `"⌘ + C\n\nCommand + C"` |

### 卡片 (Card) 可选字段

| 字段名 | 类型 | 说明 | 默认值 |
|--------|------|------|--------|
| `tags` | Array | 标签数组，用于分类和搜索 | `[]` |
| `media` | Object | 媒体引用对象 | `None` |
| `custom` | Object | 自定义字段键值对 | `{}` (TOML必须包含) |
| `created_at` | String | 创建时间 (ISO 8601格式) | 当前时间 |
| `modified_at` | String | 修改时间 (ISO 8601格式) | 当前时间 |

## TOML 格式规范

### 完整结构示例

```toml
[deck]
name = "卡片组名称"
description = "卡片组描述"
uuid = "550e8400-e29b-41d4-a716-446655440100"
created_at = "2025-01-01T00:00:00Z"
modified_at = "2025-01-01T00:00:00Z"

[[cards]]
id = "550e8400-e29b-41d4-a716-446655440101"
front = "问题内容"
back = "答案内容"
tags = ["标签1", "标签2"]
custom = {}
created_at = "2025-01-01T00:00:00Z"
modified_at = "2025-01-01T00:00:00Z"

[[cards]]
# 更多卡片...
```

### TOML 格式要求

1. **顶级结构**: 必须包含 `[deck]` 节和 `[[cards]]` 数组
2. **custom 字段**: 在 TOML 格式中必须存在，即使为空 (`custom = {}`)
3. **数组语法**: 使用 `[[cards]]` 表示卡片数组中的每个元素
4. **字符串**: 使用双引号包围
5. **日期时间**: 使用 ISO 8601 格式

### TOML 格式特点

- ✅ **人类可读性强**: 结构清晰，易于手动编辑
- ✅ **支持注释**: 使用 `#` 添加注释
- ✅ **类型安全**: 明确的字段类型定义
- ✅ **层次结构**: 支持复杂的嵌套数据
- ❌ **文件体积**: 相对较大
- ❌ **解析速度**: 比 JSON 稍慢

## JSON 格式规范

### 完整结构示例

```json
{
  "name": "卡片组名称",
  "description": "卡片组描述",
  "created_at": "2025-01-01T00:00:00Z",
  "modified_at": "2025-01-01T00:00:00Z",
  "cards": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440101",
      "front": "问题内容",
      "back": "答案内容",
      "tags": ["标签1", "标签2"],
      "created_at": "2025-01-01T00:00:00Z",
      "modified_at": "2025-01-01T00:00:00Z"
    }
  ]
}
```

### JSON 格式要求

1. **根对象**: 包含卡片组信息和 cards 数组
2. **cards 字段**: 必须存在，包含卡片数组
3. **字符串**: 使用双引号包围
4. **逗号**: 对象和数组元素之间需要逗号分隔
5. **自定义字段**: 可选，不需要包含空对象

### JSON 格式特点

- ✅ **文件紧凑**: 文件体积较小
- ✅ **解析快速**: 广泛支持，解析速度快
- ✅ **程序友好**: 易于程序生成和处理
- ✅ **跨平台**: 标准格式，广泛支持
- ❌ **可读性**: 相对 TOML 较差
- ❌ **不支持注释**: 无法在文件中添加注释

## 字段详细说明

### UUID 生成

每个卡片组和卡片都需要唯一的 UUID。可以使用以下方式生成：

```bash
# Linux/macOS
uuidgen

# Python
python -c "import uuid; print(uuid.uuid4())"

# JavaScript (Node.js)
node -e "console.log(require('crypto').randomUUID())"
```

### 时间格式

使用 ISO 8601 格式的 UTC 时间：
```
2025-01-01T00:00:00Z
2025-01-01T12:34:56.789Z
```

### 标签使用

标签用于卡片分类和搜索，建议：
- 使用简洁的描述性标签
- 标签名称保持一致性
- 使用英文标签（便于搜索）
- 限制标签数量（建议每张卡片不超过5个）

### 自定义字段

`custom` 字段用于存储额外的元数据：

```toml
# TOML 格式
custom = { difficulty = "easy", source = "教材第3章", priority = 1 }

# JSON 格式
"custom": {
  "difficulty": "easy",
  "source": "教材第3章",
  "priority": 1
}
```

## 最佳实践

### 1. 文件命名

- 使用描述性的文件名
- 使用下划线代替空格
- 包含版本号或日期（如需要）
```
✅ macos_shortcuts_v1.toml
✅ japanese_nouns_2025.json
❌ My Deck.toml
❌ deck 1.json
```

### 2. 内容组织

- 每张卡片聚焦一个概念
- 问题清晰明确
- 答案完整准确
- 适当使用换行符和格式

### 3. 标签策略

```toml
# 好的标签示例
tags = ["基础操作", "文本编辑", "快捷键"]

# 避免
tags = ["1", "重要", "待学习"]
```

### 4. 数据验证

导入前验证文件格式：

```bash
# TOML 格式验证
cargo run -- import your_file.toml --deck "test" --format toml --dry-run

# JSON 格式验证
jq . your_file.json
```

## 常见错误

### 1. 缺少必需字段

```toml
# 错误：缺少 id 字段
[[cards]]
front = "问题"
back = "答案"
# 应该添加：
# id = "generated-uuid-here"
```

### 2. TOML custom 字段缺失

```toml
# 错误：缺少 custom 字段
[[cards]]
id = "550e8400-e29b-41d4-a716-446655440101"
front = "问题"
back = "答案"

# 正确：添加空的 custom 字段
[[cards]]
id = "550e8400-e29b-41d4-a716-446655440101"
front = "问题"
back = "答案"
custom = {}
```

### 3. JSON 格式错误

```json
// 错误：缺少逗号
{
  "name": "测试",
  "cards": [
    {
      "id": "test"
      "front": "问题"  // 这里缺少逗号
    }
  ]
}

// 正确：添加逗号
{
  "name": "测试",
  "cards": [
    {
      "id": "test",
      "front": "问题"
    }
  ]
}
```

### 4. 时间格式错误

```toml
# 错误的时间格式
created_at = "2025-01-01"
created_at = "2025/01/01 00:00:00"

# 正确的时间格式
created_at = "2025-01-01T00:00:00Z"
```

## 导入命令示例

### TOML 导入

```bash
# 基本导入
cargo run -- import macos_shortcuts.toml --deck "macOS快捷键" --format toml

# 覆盖现有卡片组
cargo run -- import macos_shortcuts.toml --deck "macOS快捷键" --format toml --overwrite

# 包含学习状态
cargo run -- import macos_shortcuts.toml --deck "macOS快捷键" --format toml --include-states
```

### JSON 导入

```bash
# 基本导入
cargo run -- import vscode_shortcuts.json --deck "VSCode快捷键" --format json

# 覆盖现有卡片组
cargo run -- import vscode_shortcuts.json --deck "VSCode快捷键" --format json --overwrite

# 包含学习状态
cargo run -- import vscode_shortcuts.json --deck "VSCode快捷键" --format json --include-states
```

## 导出命令示例

```bash
# TOML 导出
cargo run -- export my_deck.toml --deck "我的卡片组" --format toml --include-states

# JSON 导出
cargo run -- export my_deck.json --deck "我的卡片组" --format json --include-states
```

---

**提示**: 建议使用 TOML 格式进行手动编辑和维护，使用 JSON 格式进行程序化生成和处理。两种格式在功能上完全等效，选择哪种主要取决于使用场景。