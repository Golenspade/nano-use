---
name: nano-use
description: Gives the agent eyes and hands on macOS. Use when the task requires seeing the screen or physical interaction (click, type, scroll, keypress). Do not use for tasks solvable via shell commands alone.
compatibility: macOS 12.3+. nano-use binary must be in skill directory. Model must support image input (vision).
---

# nano-use

A minimal macOS computer-use toolkit.

## Commands

```bash
./nano-use screenshot                        # → stdout: base64 PNG of full screen
./nano-use click <x> <y>                     # → left click at (x, y)
./nano-use right_click <x> <y>              # → right click at (x, y)
./nano-use double_click <x> <y>             # → double click at (x, y)
./nano-use drag <x1> <y1> <x2> <y2>        # → click and drag from (x1,y1) to (x2,y2)
./nano-use type "<text>"                     # → keyboard type, quote if spaces
./nano-use keypress "<keys>"                 # → e.g. "cmd+tab"  "return"  "escape"
./nano-use scroll <x> <y> <dy>              # → dy: positive=up  negative=down
```

## Errors

- Non-zero exit = action failed, reason on stderr
- Screen unchanged after action: retry once, then report stuck
- Permission dialog on screen: click Allow/OK before continuing
- screenshot returns black screen: tell the user to grant Screen Recording
  permission in System Settings → Privacy & Security, then retry
