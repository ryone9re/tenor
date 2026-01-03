# Tenor — Agent Instructions (AI / Copilot / Cursor / etc.)

Please refer to the following document for details.

- [`docs/design/tenor-design-doc.md`](../docs/design/tenor-design-doc.md)
- [`docs/design/engine-api-spec.md`](../docs/design/engine-api-spec.md)
- [`docs/design/ui-ux-spec.md`](../docs/design/ui-ux-spec.md)

---

## 0) Project summary

**tenor** is a Rust **TUI** (Ratatui) for container management, initially targeting **Docker Engine (dockerd)** via Docker Engine API.

Top priorities:
1. **UI/UX quality** (keyboard-first, responsiveness, low friction)
2. Engine-agnostic architecture (adapter pattern)
3. Solid error handling + safety around destructive operations

---

## 1) Non-negotiables

- **Never block the UI thread** on I/O. Use async tasks + state updates.
- Keep Docker-specific concepts out of the UI as much as possible.
- Engine interaction must go through `tenor-core` abstractions.
- Prefer incremental PR-sized changes with tests or at least clear manual test steps.
- If a decision affects architecture/UX significantly, **ask the user** (see §8).

---

## 2) Workspace structure (target)

Use `cargo workspace` and keep dependencies tight.

Planned crates:
- `tenor-core`
  - Domain models: Container/Image/Volume/Network
  - `Engine` trait (use-case oriented)
  - State machine / reducers for UI
  - Shared error types + event definitions
- `tenor-docker`
  - Docker Engine API client
  - Implements `Engine` trait
  - DTO ↔ domain mapping, error mapping
- `tenor-tui`
  - Ratatui components, screens, focus handling
  - Input/keymap, command palette, toasts
  - Renders `tenor-core` state only
- Optional later:
  - `tenor-config` (config parsing + defaults)
  - `tenor-cli` (diagnostics and tooling)

Do not introduce new crates unless it clearly improves boundaries.

---

## 3) Coding conventions (Rust)

- Formatting: `rustfmt`
- Lints: `clippy` (no new warnings)
- Error handling:
  - Use `thiserror` for error enums
  - Use `anyhow` only at app boundaries if needed
- Async:
  - Tokio is assumed unless explicitly changed
- Naming:
  - Domain structs: `Container`, `Image`, etc.
  - IDs: prefer `ContainerId(String)` newtypes in core if useful

---

## 4) UI/UX requirements (Ratatui)

- Keyboard-first navigation
- Consistent focus model (which pane is active)
- Provide:
  - command palette (`:`)
  - help overlay (`?`)
  - non-intrusive toast notifications
- For destructive actions (delete/remove/prune):
  - require explicit confirmation (dialog + typed confirm for large operations if needed)
- Logs and stats:
  - must support streaming
  - must remain responsive under high output volume (buffer strategy required)

---

## 5) Engine abstraction rules

Define the `Engine` trait based on **what the UI needs**, not Docker’s API shapes.

The adapter is responsible for:
- converting Docker API DTOs → `tenor-core` domain models
- mapping errors into `core::Error` categories:
  - User-actionable (daemon down, permission, not found)
  - Retryable (timeout, transient network)
  - Bug/unexpected (parsing failures, invariant breaks)

Avoid leaking Docker-specific fields unless truly universal (e.g., labels are ok).

---

## 6) Safety & security

- `docker.sock` is privileged. Document it and show the connection target in UI.
- Avoid defaulting to dangerous operations:
  - No “prune all” without confirmation
  - No silent deletes
- Prefer read-only operations by default.

---

## 7) Testing expectations

Minimum:
- `tenor-core`: unit tests for reducers/state transitions
- `tenor-docker`: mapping tests using JSON fixtures (no live daemon required)
- Provide manual test steps for UI changes:
  - “Open containers tab → select running container → logs follow → pause → search”

If integration tests are added, keep them opt-in (feature flag or env guard).

---

## 8) Decision points — when to ask the user

Ask the user (do not guess) when:
- Keybindings, navigation model, or UX flows change materially
- Destructive behavior is added/changed (delete/prune)
- Engine trait changes that affect long-term compatibility are needed
- Introducing a new major dependency or crate

When asking, provide:
- 2–3 options
- recommended default
- trade-offs in 3–5 bullets

---

## 9) Output style for AI changes

When proposing code changes:
- Provide a short plan (bullets)
- Provide patch-sized code blocks per file
- Include `cargo fmt/clippy/test` commands to run
- Mention any backward-incompatible changes explicitly

---

## 10) Initial implementation order (suggested)

1. `tenor-core`: domain models + Engine trait + minimal state
2. `tenor-docker`: list containers + basic actions
3. `tenor-tui`: containers list screen + details pane
4. logs streaming, exec session, stats streaming
5. images/volumes/networks

Stop and reassess UX after step 3.
