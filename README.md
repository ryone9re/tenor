# tenor

A fast, keyboard-first **TUI** for managing containers — a pragmatic alternative to Docker Desktop.

> **Status:** Early development (pre-v0.1).  
> **Backend:** Docker Engine (dockerd) first.  
> **UI:** Ratatui.  
> **Design goal:** UX/interaction quality is the #1 priority.

---

## Why

- Docker Desktop is paid in many commercial contexts.
- OrbStack is great but paid for commercial use.
- Rancher Desktop can be unstable depending on environment.
- Tenor aims to provide the “most used” Docker Desktop features in a **high-quality TUI**.

---

## What Tenor is (v0.1 MVP scope)

Tenor focuses on day-to-day operations:

### Containers

- List / filter / search
- Inspect (summary + details)
- Start / stop / restart / delete
- Logs: follow, pause, search
- Exec: interactive shell
- Stats: CPU / mem / net / IO

### Images

- List, inspect
- Pull, remove

### Volumes / Networks

- List, inspect
- Remove

> Non-goals for v0.1: Kubernetes integration, GUI, registry auth workflows, vulnerability scanning.

---

## Requirements

Tenor does **not** bundle a container engine. You need an engine running separately.

- Docker Engine (`dockerd`) reachable via a socket
  - Tenor automatically detects your current Docker context
  - Supports: Docker Desktop, OrbStack, Colima, Rancher Desktop, etc.
  - Linux: typically `unix:///var/run/docker.sock`
  - macOS: varies by tool (e.g., OrbStack uses `~/.orbstack/run/docker.sock`)

Connection priority:

1. Current Docker context (via `docker context inspect`)
2. Fallback to `/var/run/docker.sock`

⚠️ **Security note:** Access to `docker.sock` is effectively root-equivalent on the host. Treat it as privileged.

---

## Installation (dev)

> Tenor is currently in early development.

```bash
git clone https://github.com/ryone9re/tenor
cd tenor
cargo build --release
```

The binary will be at `target/release/tenor`.

---

## Run

The workspace includes three packages:

- `tenor-core` — domain models, state machine, Engine trait
- `tenor-docker` — Docker Engine API adapter (implements Engine trait)
- `tenor-tui` — Ratatui frontend (binary)

Run the TUI:

```bash
# Development
cargo run

# Or run directly from target
./target/release/tenor
```

Environment variables:

```bash
# Point to a specific Docker socket (future feature)
DOCKER_HOST=unixCurrent Implementation)

### Global
- Quit: `q` or `Ctrl+C`
- Switch tabs: `1`-`5` or `Tab`/`Shift+Tab`
- Refresh: `r` or `R`

### Containers Tab
- Navigate list: `↑↓` or `j`/`k`
- Start container: `s`
- Stop container: `t`

> Note: More keybindings (restart, delete, logs, exec, etc.) are planned and will be added incrementally. (open), `f` (follow), `p` (pause), `/` (search)
- Exec: `e`
- Refresh: `R`
- Command palette: `:`
- Help: `?`
- Quit: `q`

---

## Configuration (planned)

Config file:

- Linux/macOS: `~/.config/tenor/config.toml`

Example (planned):

```toml
[docker]
host = "unix:///var/run/docker.sock"

[ui]
refresh_ms = 1500
theme = "default"
```

---

## Architecture (high level)

Tenor is designed to be engine-agnostic:

- UI (Ratatui) depends only on `tenor-core`
- `tenor-core` defines an `Engine` trait (use-case driven)
- Backend adapters implement the trait (`tenor-docker` first)
- Future: containerd / CRI-O / custom runtime adapters

```plaintext
tenor-tui ──> tenor-core (domain, state, Engine trait) ──> tenor-docker (adapter) ──> dockerd
```

---

## Development

### Prerequisites

- Rust 1.70+ (2021 edition)
- Docker Engine running (dockerd)

### Build & Run

```bash
# Build all packages
cargo build

# Run TUI application
cargo run

# Release build
cargo build --release
```

### Code Quality

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Run all quality checks
cargo fmt --all -- --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all
```

### Testing

```bash
# Run unit tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run tests for specific package
cargo test -p tenor-core
cargo test -p tenor-docker
```

**Test Coverage:**

- 22 unit tests across all packages
- Tests for domain models, error handling, mappers, and context detection
- Integration tests planned for future releases

### Mutation Testing

Tenor uses [cargo-mutants](https://mutants.rs/) for mutation testing to ensure test quality:

```bash
# Install cargo-mutants
cargo install cargo-mutants

# Run mutation tests
cargo mutants

# Run on specific file
cargo mutants -f tenor-core/src/domain/container.rs

# Parallel execution (faster)
cargo mutants --no-shuffle -j 4
```

Configuration is in [.cargo-mutants.toml](.cargo-mutants.toml).

### Continuous Integration

CI runs on every push to `main` and on pull requests:

- ✅ Code formatting check (`cargo fmt`)
- ✅ Linting (`cargo clippy` with `-D warnings`)
- ✅ All unit tests
- ✅ Security audit ([cargo-audit](https://github.com/rustsec/rustsec))
- 🧬 Mutation tests (on `main` only)

See [.github/workflows/ci.yml](.github/workflows/ci.yml) for details.

### Workspace layout

```plaintext
tenor/
  tenor-core/       # Domain models, Engine trait, error types
  tenor-docker/     # Docker Engine API implementation
  tenor-tui/        # Ratatui TUI application (binary)
  docs/
    design/         # Design documents
      tenor-design-doc.md
      engine-api-spec.md
      ui-ux-spec.md
  .github/
    copilot-instructions.md  # AI agent instructions

---

## Roadmap

- v0.1: containers/logs/exec/stats + images/volumes/networks basics + polished UX
- v0.2: docker compose grouping (labels-based), remote/context support, safe prune flows
- v0.3+: containerd adapter, plugin/engine discovery
- future: runtime experimentation (likely separate repo)

---

## Contributing

This project is currently in rapid iteration.
If you want to contribute, open an issue first to align on UX + architecture constraints.
