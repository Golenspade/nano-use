# nano-use 设计决策文档

## 项目定义

nano-use 是一个最小化的 macOS computer-use 工具包。
设计哲学：minimal 代码量，实现最 general 的 GUI 自动化能力。
对 agent 暴露 CLI 接口，agent 通过 bash tool 调用。

---

## 技术栈决策

| 决策项 | 结论 | 原因 |
|---|---|---|
| 截图 API | ScreenCaptureKit | 苹果官方推荐，macOS 12.3+，CGWindowListCreateImage 在 macOS 14+ 已 deprecated，70% 目标用户在 14+ |
| 鼠标/键盘 API | CoreGraphics `CGEventCreateMouseEvent` / `CGEventCreateKeyboardEvent` | 成熟稳定，FFI 零额外开销 |
| 实现语言 | Rust | 静态链接单一 binary，零运行时依赖，FFI 调用 C API 无性能损耗 |
| 暴露层 | Skill + CLI | pi 的 native 路径，agent 通过 bash tool 调用，无需 MCP Extension |
| VLM 选型 | 用户自己决定 | pi 支持 15+ providers，模型选择权交给用户，nano-use 不做嗅探或降级 |

---

## 依赖（Cargo.toml）

```toml
[package]
name = "nano-use"
version = "0.1.0"
edition = "2021"

[dependencies]
screencapturekit = "1.5"                     # 截图（ScreenCaptureKit safe bindings）
core-graphics = "0.23"                       # 鼠标 + 键盘事件（CGEventCreateMouseEvent 等）
core-foundation = "0.9"                      # macOS 类型系统基础
base64 = "0.22"                              # screenshot 输出编码
clap = { version = "4", features = ["derive"] }  # CLI 参数解析（后续可砍）
```

> 版本号为调研时参考值，写代码时去 crates.io 确认最新版本。
> clap 保留，后续视需要决定是否替换为手写解析。
> core-graphics 保留用于鼠标键盘事件，截图职责已移交 screencapturekit。

---

## 工具集（8 个命令）

```bash
./nano-use screenshot                        # → stdout: base64 PNG，全屏所有显示器
./nano-use click <x> <y>                     # → 左键单击
./nano-use right_click <x> <y>              # → 右键单击
./nano-use double_click <x> <y>             # → 左键双击
./nano-use drag <x1> <y1> <x2> <y2>        # → 按住拖拽
./nano-use type "<text>"                     # → 键盘输入文字
./nano-use keypress "<keys>"                 # → 组合键，e.g. "cmd+tab" "return"
./nano-use scroll <x> <y> <dy>              # → 滚轮，dy 正=上 负=下
```

### 设计依据

- `screenshot` 全屏 only，无参数——与 Anthropic / OpenAI computer-use 实现对齐
- 去掉 `list-windows` / `--window` / `--region`——过早优化，token 成本在个人使用场景下可忽略
- `right_click` / `double_click` 保留——无法用其他命令替代
- `drag` 保留——slider、看板、排序列表等场景无法绕过

### 工具与 agent loop 的映射

```
STEP 2 WHERE  → screenshot
STEP 5 ACT    → click / right_click / double_click / drag / type / keypress / scroll
STEP 6 CHECK  → screenshot
```

---

## 错误处理原则

**所有错误一律 hard error：exit 1，原因输出到 stderr。**

| 错误类型 | 处理方式 |
|---|---|
| 坐标越界 | exit 1，stderr 报具体坐标和屏幕尺寸 |
| 空字符串（type） | exit 1，stderr 报错 |
| 未知命令 | exit 1，stderr 报错 |
| macOS API 调用失败 | exit 1，stderr 报原因 |

原则：explicit over implicit。静默失败会让 agent 误以为操作成功，延迟暴露问题。

---

## SKILL.md 定稿

```markdown
---
name: nano-use
description: Gives the agent eyes and hands on macOS. Use when the task requires seeing the screen or physical interaction (click, type, scroll, keypress). Do not use for tasks solvable via shell commands alone.
compatibility: macOS 12.3+. nano-use binary must be in skill directory. Model must support image input (vision).
---

# nano-use

A minimal macOS computer-use toolkit.

## Commands

    ./nano-use screenshot                        # → stdout: base64 PNG of full screen
    ./nano-use click <x> <y>                     # → left click at (x, y)
    ./nano-use right_click <x> <y>              # → right click at (x, y)
    ./nano-use double_click <x> <y>             # → double click at (x, y)
    ./nano-use drag <x1> <y1> <x2> <y2>        # → click and drag from (x1,y1) to (x2,y2)
    ./nano-use type "<text>"                     # → keyboard type, quote if spaces
    ./nano-use keypress "<keys>"                 # → e.g. "cmd+tab"  "return"  "escape"
    ./nano-use scroll <x> <y> <dy>              # → dy: positive=up  negative=down

## Errors

- Non-zero exit = action failed, reason on stderr
- Screen unchanged after action: retry once, then report stuck
- Permission dialog on screen: click Allow/OK before continuing
- screenshot returns black screen: tell the user to grant Screen Recording
  permission in System Settings → Privacy & Security, then retry
```

---

## 项目结构

```
nano-use/
├── Cargo.toml
├── src/
│   └── main.rs
└── skill/
    ├── SKILL.md
    └── nano-use          ← cargo build --release 产出的 binary 复制到这里
```

---

## 实现顺序

1. `screenshot` — ScreenCaptureKit → base64 PNG → stdout
2. `click` / `right_click` / `double_click` — 共享 `CGEventCreateMouseEvent`
3. `drag` — mousedown → move → mouseup
4. `type` + `keypress` — `CGEventCreateKeyboardEvent`
5. `scroll` — `CGEventCreateScrollWheelEvent`

---

## 关键技术说明

**截图 API 迁移原因**
macOS 用户版本分布（Homebrew 开发者数据）：Tahoe 26 ~38%，Sequoia 15 ~22%，Sonoma 14 ~8%。
70% 目标用户在 macOS 14+，`CGWindowListCreateImage` 在 14+ 已 deprecated。
改用 `screencapturekit` crate（ScreenCaptureKit safe bindings），覆盖 macOS 12.3+，100% 目标用户。

**为什么没有 C 或 Swift？**
CoreGraphics 是苹果的 C API，相关 Rust crate 已做 FFI 绑定，直接调用无额外开销。
ScreenCaptureKit 是 Objective-C API，`screencapturekit` crate 提供 safe Rust 封装。

**多显示器坐标系**
macOS 所有显示器共享统一虚拟坐标空间，ScreenCaptureKit 全屏捕获自动包含所有显示器。
坐标天然跨屏，无需指定显示器。

**性能基线**
- ScreenCaptureKit 捕获一帧：~5-15ms
- base64 编码：~1-3ms
- `CGEventPost` 鼠标/键盘事件：~0.1ms
