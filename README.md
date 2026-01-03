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
  - Linux: `unix:///var/run/docker.sock`
  - macOS: via Docker Desktop / Colima / etc. (as long as the Docker API socket is reachable)
  - Windows: WSL2 scenarios TBD (tracked in issues)

⚠️ **Security note:** Access to `docker.sock` is effectively root-equivalent on the host. Treat it as privileged.

---

## Installation (dev)

> Tenor is not released yet; use from source.

```bash
git clone <your-repo-url>
cd tenor
cargo build
````

---

## Run

Workspace packages (planned):

- `tenor-tui` — Ratatui frontend
- `tenor-core` — domain + state machine + engine trait
- `tenor-docker` — Docker Engine API adapter

Run TUI:

```bash
cargo run -p tenor-tui
```

If you need to point to a non-default socket (planned):

```bash
TENOR_DOCKER_HOST=unix:///var/run/docker.sock cargo run -p tenor-tui
```

---

## Keybindings (draft)

> UX is still evolving; this is the current direction.

- Navigation: `↑↓←→` / `hjkl`
- Open details: `Enter`
- Start / Stop / Restart: `s` / `t` / `r`
- Delete: `d`
- Logs: `l` (open), `f` (follow), `p` (pause), `/` (search)
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

### Formatting & Lint

```bash
cargo fmt
cargo clippy --all-targets --all-features
```

### Tests

```bash
cargo test
```

### Workspace layout (planned)

```plaintext
tenor/
  crates/
    tenor-core/
    tenor-docker/
    tenor-tui/
  docs/
    design/
    adr/
```

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
