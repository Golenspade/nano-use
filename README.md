<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/logo/nano-use-logo-dark.svg">
    <img src="assets/logo/nano-use-logo.svg" alt="nano-use logo" width="420">
  </picture>
</p>

<p align="center">
  为 Agent 而生的本地 macOS 桌面操控技能。
</p>

<p align="center">
  <a href="#overview">项目简介</a> ·
  <a href="#quick-start">快速上手</a> ·
  <a href="#examples">使用示例</a> ·
  <a href="#build">构建</a> ·
  <a href="#permissions">权限说明</a> ·
  <a href="#license">开源协议</a>
</p>

---

<a id="overview"></a>

## 项目简介

`nano-use` 是一个为 Agent 设计的本地 macOS 桌面操控技能。它由两部分组成：

- 一个轻量的 Rust CLI，负责在本地执行桌面操作；
- 一份 `SKILL.md`，告诉 Agent 如何安装和调用这个 CLI。

CLI 提供了一系列底层的桌面操控原语：截屏、鼠标点击、键盘输入、滚动。

`nano-use` 不是自主 Agent。它不会规划，不会决策，不会替你思考。它只是为外部 Agent 提供了一道窄而可靠的本地接口，用来观察和操控桌面。

<a id="quick-start"></a>

## 快速上手

安装 `nano-use` 有两种方式：交给 Agent，或者自己动手。

### 让 Agent 自行安装

把仓库地址交给你的 Agent：

```text
https://github.com/Golenspade/nano-use
```

然后让它读取并执行 `SKILL.md` 中的安装指引。

Agent 应该完成以下步骤：

1. 克隆本仓库；
2. 构建本地 CLI；
3. 将二进制文件放到 `PATH` 中的某个位置；
4. 读取 `SKILL.md` 了解工具契约；
5. 提醒你授予所需的 macOS 系统权限。

### 手动安装

构建 CLI：

```bash
git clone https://github.com/Golenspade/nano-use.git
cd nano-use
cargo build --release
```

安装二进制文件：

```bash
mkdir -p ~/.local/bin
cp target/release/nano-use ~/.local/bin/nano-use
```

验证安装：

```bash
nano-use --help
```

完成后，让你的 Agent 读取仓库中的 `SKILL.md`。

<a id="examples"></a>

## 使用示例

> 演示 GIF 即将上线。

<!--
未来 GIF 构想：
Agent 读取 SKILL.md → 调用 nano-use 截屏 → 观察屏幕 → 执行点击/输入/滚动。
-->

<a id="build"></a>

## 构建

`nano-use` 是一个 Rust 项目。

```bash
cargo build --release
```

Release 二进制文件生成路径：

```text
target/release/nano-use
```

当前命令接口：

```text
screenshot
click
right_click
double_click
drag
type
keypress
scroll
```

<a id="permissions"></a>

## 权限说明

`nano-use` 需要捕获屏幕并控制本地输入，因此必须获得 macOS 的系统权限。

| 权限 | 用途 |
| --- | --- |
| 屏幕录制 | 截屏 |
| 辅助功能 | 鼠标和键盘操作 |

请在 macOS 系统设置中提前开启这些权限，再让 Agent 调用。

如果命令执行后没有截图、或者鼠标键盘没有响应，最常见的原因就是权限尚未正确授予。

<a id="license"></a>

## 开源协议

MIT License。

详见 [`LICENSE`](LICENSE) 文件。
