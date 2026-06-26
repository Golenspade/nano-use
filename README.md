<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/logo/nano-use-logo-dark.svg">
    <img src="assets/logo/nano-use-logo.svg" alt="nano-use logo" width="420">
  </picture>
</p>

<p align="center">
  Minimal macOS computer-use toolkit.
</p>

<p align="center">
  <a href="#overview">Overview</a> ·
  <a href="#commands">Commands</a> ·
  <a href="#build">Build</a> ·
  <a href="#permissions">Permissions</a> ·
  <a href="#logo">Logo</a>
</p>

---

## Overview

`nano-use` is a small Rust CLI for macOS computer-use actions.

It provides a compact interface for:

- taking a desktop screenshot and writing it as a base64-encoded PNG;
- sending mouse actions such as click, right click, double click, drag, and scroll;
- sending keyboard actions such as text input and key combinations.

The project is intentionally narrow: small command surface, explicit coordinates, and direct local execution.

## Commands

```bash
nano-use <COMMAND>
```

| Command | Description |
|---|---|
| `screenshot` | Capture the desktop and print a base64-encoded PNG to stdout. |
| `click <x> <y>` | Left-click at screen coordinate `(x, y)`. |
| `right_click <x> <y>` | Right-click at screen coordinate `(x, y)`. |
| `double_click <x> <y>` | Double-click at screen coordinate `(x, y)`. |
| `drag <x1> <y1> <x2> <y2>` | Drag from `(x1, y1)` to `(x2, y2)`. |
| `type <text>` | Type text through the macOS keyboard event path. |
| `keypress <keys>` | Send a key combination, for example `cmd+c` or `shift+tab`. |
| `scroll <x> <y> <dy>` | Move the cursor to `(x, y)` and send a vertical scroll event. |

Supported `keypress` modifiers include:

```text
cmd, command, ctrl, control, alt, option, shift, fn, function
```

Supported named keys include common keys such as:

```text
enter, return, tab, space, delete, escape, up, down, left, right,
home, end, pageup, pagedown, forward_delete, f1 ... f12
```

Single US-layout characters such as `a`, `b`, `1`, `-`, `/`, and `.` are also supported.

## Build

```bash
git clone https://github.com/Golenspade/nano-use.git
cd nano-use
cargo build --release
```

Run the CLI from the release binary:

```bash
./target/release/nano-use --help
```

## Examples

Capture a screenshot and decode it into a PNG on macOS:

```bash
./target/release/nano-use screenshot | base64 -D > screenshot.png
```

Click and type:

```bash
./target/release/nano-use click 400 300
./target/release/nano-use type "hello from nano-use"
```

Send a keyboard shortcut:

```bash
./target/release/nano-use keypress cmd+c
```

Scroll at a coordinate:

```bash
./target/release/nano-use scroll 600 500 -400
```

## Permissions

Because `nano-use` controls local screen, mouse, and keyboard behavior, macOS may require permissions before all commands work correctly.

Typical permissions:

- **Screen Recording** for `screenshot`.
- **Accessibility** for mouse and keyboard actions.

Grant these permissions in macOS System Settings if the command runs but cannot capture or control the UI.

## Project status

`nano-use` is currently an early CLI. The implemented interface is command-line oriented, not a stable library API.

Near-term work:

- add CI and tests;
- add clearer error messages for macOS permission failures;
- document coordinate behavior for multi-display setups;
- add examples for agent/tool integrations;
- decide license and release workflow.

## Repository layout

```text
nano-use/
├── assets/
│   └── logo/
│       ├── nano-use-logo.svg
│       └── nano-use-logo-dark.svg
├── src/
│   └── main.rs
├── Cargo.toml
└── README.md
```

## Logo

The current mark is the **Cursor Action** direction: a compact `n` paired with a cursor shape.

The README uses a responsive logo pair:

```text
assets/logo/nano-use-logo.svg       # light/default version, black wordmark
assets/logo/nano-use-logo-dark.svg  # dark-mode version, white wordmark
```

This avoids the low-contrast issue where a light-mode README would display a pale wordmark on a light background.

## License

No license has been declared yet. Add a `LICENSE` file before publishing or distributing the project as a reusable package.
