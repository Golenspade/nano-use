<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/logo/nano-use-logo-dark.svg">
    <img src="assets/logo/nano-use-logo.svg" alt="nano-use logo" width="420">
  </picture>
</p>

<p align="center">
  <a href="README.md"><kbd>English</kbd></a>
  <a href="README.zh-CN.md"><kbd>简体中文</kbd></a>
</p>

<p align="center">
  A local macOS computer-use skill for agents.
</p>

<p align="center">
  <a href="#overview">Overview</a> ·
  <a href="#quick-start">Quick Start</a> ·
  <a href="#examples">Examples</a> ·
  <a href="#build">Build</a> ·
  <a href="#permissions">Permissions</a> ·
  <a href="#license">License</a>
</p>

---

<a id="overview"></a>

## Overview

`nano-use` is a local macOS computer-use skill for agents. It combines two parts:

- a lightweight Rust CLI that executes desktop actions locally;
- a `SKILL.md` file that tells an agent how to install and call the CLI.

The CLI provides low-level desktop-control primitives: screenshot, mouse clicks, keyboard input, and scrolling.

`nano-use` is not an autonomous agent. It does not plan, decide, or think for the user. It only gives an external agent a narrow and reliable local interface for observing and controlling the desktop.

<a id="quick-start"></a>

## Quick Start

There are two ways to install `nano-use`: let your agent install it, or install it manually.

### Agent-assisted install

Give this repository URL to your agent:

```text
https://github.com/Golenspade/nano-use
```

Then ask it to read and follow the installation instructions in `SKILL.md`.

The agent should:

1. clone this repository;
2. build the local CLI;
3. place the binary somewhere on `PATH`;
4. read `SKILL.md` to understand the tool contract;
5. remind you to grant the required macOS permissions.

### Manual install

Build the CLI:

```bash
git clone https://github.com/Golenspade/nano-use.git
cd nano-use
cargo build --release
```

Install the binary:

```bash
mkdir -p ~/.local/bin
cp target/release/nano-use ~/.local/bin/nano-use
```

Verify the installation:

```bash
nano-use --help
```

After that, point your agent to the repository's `SKILL.md`.

<a id="examples"></a>

## Examples

> Demo GIF coming soon.

<!--
Future GIF idea:
Agent reads SKILL.md → calls nano-use screenshot → observes the screen → performs click/type/scroll.
-->

<a id="build"></a>

## Build

`nano-use` is a Rust project.

```bash
cargo build --release
```

The release binary is generated at:

```text
target/release/nano-use
```

Current command surface:

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

## Permissions

`nano-use` captures the screen and controls local input, so it requires macOS system permissions.

| Permission | Used for |
| --- | --- |
| Screen Recording | Screenshot capture |
| Accessibility | Mouse and keyboard actions |

Grant these permissions in macOS System Settings before letting an agent call `nano-use`.

If a command runs but screenshots are empty or mouse/keyboard actions do not work, missing permissions are the most common cause.

<a id="license"></a>

## License

MIT License.

See [`LICENSE`](LICENSE).
