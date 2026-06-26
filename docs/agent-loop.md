# Minimal Computer Use Agent Loop

A 9-step cognitive framework for autonomous GUI interaction on macOS.

---

## Overview

This framework defines how a minimal computer use agent operates. The design principle is deliberate: **keep code minimal, delegate cognitive complexity to the VLM**. Only the execution primitives (screenshot, click, type, etc.) live in code — planning, grounding, and verification happen entirely inside the model's context window.

---

## The 9-Step Loop

### 1. GOAL — Intent Anchoring

The user provides a natural language directive that anchors every subsequent step. This goal is never mutated — it stays in the model's system context as the single source of truth for the entire session.

> Example: *"Open Safari, go to GitHub, and star the repository 'lutzroeder/agents'."*

---

### 2. WHERE — Environmental Perception

Before any action, the agent must know where it is. A `screenshot()` call captures the current visual state of the OS, returning a base64-encoded PNG that is passed directly to the VLM.

**MCP tool:** `screenshot() → base64 PNG`

---

### 3. LOCATE — Semantic Grounding

The VLM receives the screenshot and maps the GOAL's intent to concrete screen coordinates. This is the "grounding" step — translating *"click the search bar"* into `{x: 640, y: 88}`.

**This step happens entirely inside the VLM — zero code required.**

---

### 4. SPEC — Action Planning

Given the current state and the grounded target, the VLM plans the next atomic action. Output is a structured JSON object:

```json
{ "type": "click", "x": 640, "y": 88 }
{ "type": "type",  "text": "github.com/lutzroeder/agents" }
{ "type": "keypress", "keys": ["Return"] }
```

**This step happens entirely inside the VLM — zero code required.**

---

### 5. ACT — API Execution

The agent translates the VLM's structured action into macOS system API calls via `pyobjc`. This is the only step that mutates the environment.

| Action | macOS API |
|--------|-----------|
| `click(x, y)` | `CGEventCreateMouseEvent` (CoreGraphics) |
| `type(text)` | `CGEventCreateKeyboardEvent` (CoreGraphics) |
| `keypress(keys)` | `CGEventCreateKeyboardEvent` + modifier flags |
| `scroll(x, y, dy)` | `CGEventCreateScrollWheelEvent` |
| `shell(cmd)` | `subprocess.run` |

---

### 6. CHECK — Post-Action Observation

Immediately after execution, the agent captures a fresh screenshot. This is structurally identical to step 2 but semantically different: it observes the *consequence* of the last action, not the initial state.

**MCP tool:** `screenshot() → base64 PNG`

---

### 7. STATUS — Global Progress Evaluation

The VLM receives the new screenshot alongside the full action history and original GOAL. It evaluates: *"Did this action move us closer to the goal, stay neutral, or introduce an error?"*

The history log is the agent's "memory" — a growing list of `(action, screenshot)` pairs that provide context for this evaluation.

**This step happens entirely inside the VLM — zero code required.**

---

### 8. VERIFY — Robustness Decision

The critical branching node. The VLM outputs one of three signals:

| Signal | Meaning | Next Step |
|--------|---------|-----------|
| `continue` | Progress made, task incomplete | Return to step 2 (WHERE) |
| `done` | GOAL fully achieved | Proceed to step 9 (DONE) |
| `failed` | Stuck or error detected | Retry with corrective action, or abort |

This step is what separates a robust agent from a brittle one. Most minimal implementations omit it, causing agents to loop infinitely when blocked by a popup, permission dialog, or unexpected UI state.

---

### 9. DONE — Termination

The agent confirms the GOAL is satisfied, halts execution, and returns a success signal to the user. The session ends cleanly.

---

## Tool Mapping

The entire 9-step loop requires only **5–6 MCP tools**. All cognitive steps (LOCATE, SPEC, STATUS, VERIFY) are delegated to the VLM.

```
screenshot()          →  Steps 2 (WHERE) + 6 (CHECK)
click(x, y, button)   →  Step 5 (ACT)
type(text)            →  Step 5 (ACT)
keypress(keys[])      →  Step 5 (ACT)
scroll(x, y, delta)   →  Step 5 (ACT)
shell(cmd)            →  Step 5 (ACT) — optional, extends to non-GUI tasks
```

---

## Code Skeleton

```python
async def agent_loop(goal: str):
    history = []

    while True:
        # Step 2: WHERE
        screen = screenshot()

        # Steps 3+4+7+8: LOCATE + SPEC + STATUS + VERIFY (all inside VLM)
        action = vlm_decide(goal, screen, history)

        # Step 9: DONE
        if action["type"] == "done":
            break
        if action["type"] == "failed":
            raise AgentError(action.get("reason"))

        # Step 5: ACT
        execute(action)

        # Step 6: CHECK
        screen_after = screenshot()
        history.append({"action": action, "screen": screen_after})
```

---

## Design Principles

**Minimal code, maximal capability.** By exposing only atomic primitives and delegating all reasoning to the VLM, the codebase stays under 100 lines while remaining capable of any GUI task the model can reason about.

**VERIFY is non-negotiable.** The difference between a toy demo and a production-grade agent is the explicit verify loop. Without it, an agent that encounters a "Are you sure?" dialog will click indefinitely.

**History is memory.** The growing `(action, screenshot)` history log is the agent's only persistent state. It enables the STATUS step to evaluate global progress — not just local, one-step outcomes.
