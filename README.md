<p align="center">
  <img src="assets/logo/nano-use-logo.svg" alt="nano-use logo" width="128" />
</p>

<h1 align="center">nano-use</h1>

<p align="center">
  A tiny, action-oriented toolkit for building controlled automation primitives.
</p>

<p align="center">
  <a href="#what-is-nano-use">Overview</a> ·
  <a href="#design-principles">Design principles</a> ·
  <a href="#status">Status</a> ·
  <a href="#roadmap">Roadmap</a>
</p>

---

## What is nano-use?

`nano-use` is intended to be a compact foundation for automation workflows where each action should be explicit, inspectable, and easy to compose.

The project name reflects two constraints:

- **nano**: keep the runtime, API surface, and mental model small.
- **use**: focus on practical execution rather than abstract orchestration.

The current repository is in its initial setup stage. This README defines the project direction and visual identity before the first implementation is added.

## Design principles

1. **Small core**  
   Keep the base package minimal. Prefer simple primitives over a large framework surface.

2. **Explicit actions**  
   Actions should be readable, traceable, and easy to audit. Hidden side effects should be avoided.

3. **Composable workflows**  
   Users should be able to combine small operations into larger flows without adopting a heavy runtime.

4. **Developer-first ergonomics**  
   The default experience should be easy to run locally, easy to inspect, and easy to modify.

5. **Control before autonomy**  
   Automation should expose what it is doing before it attempts to do too much on its own.

## Status

`nano-use` is currently a scaffold.

Planned near-term work:

- define the first public API shape;
- add minimal examples;
- add tests and CI;
- document supported use cases;
- decide packaging, release, and license details.

Until the first implementation lands, treat this repository as an early project shell rather than a stable package.

## Intended usage shape

The final API is not fixed yet. The intended direction is a small interface for composing explicit actions:

```python
# Concept sketch only. This API is not implemented yet.
from nano_use import Action, Flow

open_page = Action("open_page", target="https://example.com")
extract_text = Action("extract_text", selector="main")

flow = Flow([open_page, extract_text])
result = flow.run()
```

The exact API should be added only after the first implementation is available.

## Repository layout

```text
nano-use/
├── assets/
│   └── logo/
│       └── nano-use-logo.svg
└── README.md
```

The layout will expand as source code, tests, examples, and documentation are added.

## Logo

The current mark is the **Cursor Action** direction: a compact `n` paired with a cursor shape. It is meant to communicate lightweight practical automation rather than a generic AI brand.

Logo asset:

```text
assets/logo/nano-use-logo.svg
```

## Roadmap

- [ ] Decide the first concrete scope: browser actions, local tool actions, or general workflow primitives.
- [ ] Add package structure.
- [ ] Add the first executable example.
- [ ] Add tests for the core action model.
- [ ] Add contribution guidelines.
- [ ] Add license file.

## Contributing

This project is still being initialized. Before opening large changes, prefer starting with a small issue or pull request that clarifies one concrete part of the project direction.

Good first contributions after the initial code lands will likely include:

- examples;
- documentation fixes;
- test cases;
- small action primitives;
- API design notes.

## License

No license has been declared yet. Add a `LICENSE` file before publishing or distributing the project as a reusable package.
