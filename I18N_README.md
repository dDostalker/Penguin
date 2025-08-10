## 概述

Penguin 使用了一个基于 `build.rs` 和宏的编译时，允许在编译时选择不同的语言。

## 系统架构

### 1. 配置文件 (`config/language.toml`)
- 定义当前使用的语言
- 包含所有支持语言的文本内容
- 支持的语言：chinese, english(可以自己添加)

### 2. 构建脚本 (`build.rs`)
- 在编译时读取配置文件
- 生成对应语言的常量代码
- 设置环境变量

### 3. 模块 (`src/i18n.rs`)
- 包含构建时生成的常量
- 提供宏和工具函数
- 支持语言检测和格式化

## 使用方法

### 1. 切换语言

编辑 `config/language.toml` 文件，修改 `language` 字段：

```toml
# 切换到英文
language = "english"

# 切换到日文
language = "japanese"

# 切换到韩文
language = "korean"

# 切换到中文
language = "chinese"
```

### 2. 在代码中使用

#### 使用宏获取文本：
```rust
use crate::i18n;

// 获取应用标题
let title = i18n::APP_TITLE;

// 使用宏（推荐）
let title = i18n!(APP_TITLE);
```

#### 检查当前语言：
```rust
use crate::i18n;

// 检查是否为英文
if i18n::is_language("english") {
    // 英文特定逻辑
}

// 获取当前语言
let current_lang = i18n::get_current_language();
```

#### 格式化数字和地址：
```rust
use crate::i18n::utils;

// 根据语言格式化数字
let formatted = utils::format_number(1234567);

// 格式化地址
let addr = utils::format_address(0x12345678);
```

### 3. 添加新的文本

1. 在 `config/language.toml` 中为所有语言添加新条目：
```toml
[chinese]
new_text = "新文本"

[english]
new_text = "New Text"

2. 在代码中使用：
```rust
let text = i18n::NEW_TEXT;
```

### 4. 添加新语言

1. 在 `config/language.toml` 中添加新语言部分：
```toml
[german]
app_title = "Penguin PE Analysator"
# ... 其他文本
```

2. 在 `build.rs` 中更新默认配置（可选）

## 编译流程

1. `build.rs` 读取 `config/language.toml`
2. 根据 `language` 字段选择对应语言
3. 生成 `language_constants.rs` 文件
4. 设置 `CURRENT_LANGUAGE` 环境变量
5. 主程序编译时包含生成的常量

## 注意事项

1. 修改配置文件后需要重新编译
2. 所有语言必须包含相同的键
3. 生成的常量文件位于 `target/` 目录
4. 支持的语言在 `build.rs` 中硬编码

## 示例

### 配置文件示例
```toml
language = "english"

[english]
app_title = "Penguin PE Analyzer"
welcome_message = "Welcome to Penguin!"

[chinese]
app_title = "Penguin PE 分析器"
welcome_message = "欢迎使用 Penguin！"
```

### 代码使用示例
```rust
use crate::i18n;

fn show_welcome() {
    println!("{}", i18n::APP_TITLE);
    println!("{}", i18n::WELCOME_MESSAGE);
}
```

编译时选择 `english` 会输出：
```
Penguin PE Analyzer
Welcome to Penguin!
```

选择 `chinese` 会输出：
```
Penguin PE 分析器
欢迎使用 Penguin！
```
