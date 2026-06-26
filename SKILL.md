# nano-use 技能说明

你正在使用 `nano-use`，一个本地 macOS 桌面操控 CLI。

## 安装

在仓库根目录构建二进制文件：

```bash
cargo build --release
```

安装到 PATH：

```bash
mkdir -p ~/.local/bin
cp target/release/nano-use ~/.local/bin/nano-use
```

确认安装成功：

```bash
nano-use --help
```

## 工具契约

使用 `nano-use screenshot` 来观察屏幕。

`nano-use screenshot` 会将当前桌面截图编码为 base64 PNG，并输出到 stdout。

仅在需要执行具体操作时，才使用鼠标和键盘命令。

可用命令：

* `nano-use screenshot`
* `nano-use click <x> <y>`
* `nano-use right_click <x> <y>`
* `nano-use double_click <x> <y>`
* `nano-use drag <x1> <y1> <x2> <y2>`
* `nano-use type <text>`
* `nano-use keypress <keys>`
* `nano-use scroll <x> <y> <dy>`

## 使用规则

* 优先使用 argv 形式的子进程调用。
* 不要用不可信的模型输出来拼接 shell 字符串。
* 提醒用户授予 macOS 屏幕录制和辅助功能权限。
* 用截图来观察，用动作来执行。
* 只有在确定了具体目标后，才执行操作。
