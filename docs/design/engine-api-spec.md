# Tenor Engine API Spec (v0.1 Draft)

Status: Draft  
Core principle: UI depends on `tenor-core` Engine trait only.  
Docker specifics live in `tenor-docker`.

---

## 1. Goals

- Define a stable `Engine` interface for Tenor use-cases:
  - containers/logs/exec/stats/images/volumes/networks
- Support async + streaming with cancellation.
- Allow future adapters (containerd/CRI-O/custom) without UI changes.

Non-goals:

- Exhaustively expose every engine capability.
- Docker API DTOs leaking into core.

---

## 2. Domain Models (core)

### 2.1 Identifiers

- `ContainerId(String)`
- `ImageId(String)`
- `VolumeName(String)`
- `NetworkId(String)`

### 2.2 Common fields

- `labels: BTreeMap<String, String>`
- `created_at: DateTime<Utc>` (or chrono)

### 2.3 Container

- id, name
- image (name/tag)
- state: enum { Running, Exited, Paused, Restarting, Dead, Unknown }
- status_text: String (engine-native summary)
- ports: `Vec<PortMapping>`
- labels

### 2.4 ContainerDetail

- extends Container with:
  - command, entrypoint, env summary
  - mounts
  - network settings summary
  - raw_json (optional, behind debug flag)

### 2.5 Image / Volume / Network

Minimum set required for list + inspect + remove.

---

## 3. Error Model (core)

`EngineError` with categories:

- `UserActionable { message, hint, source }`
- `Retryable { message, source }`
- `Bug { message, source }`

Mapping rules:

- permission denied => UserActionable + hint
- not found => UserActionable
- timeouts/temporary network => Retryable
- deserialization/invariant => Bug

---

## 4. Streaming Types

### 4.1 LogStream

- async stream of `LogEvent`
- supports:
  - follow
  - since
  - timestamps
  - stdout/stderr separation

`LogEvent`:

- ts: `Option<DateTime<Utc>>`
- stream: Stdout/Stderr
- line: String

Cancellation:

- dropping stream cancels upstream task.

### 4.2 StatsStream (optional for v0.1 but designed now)

- async stream of `StatsEvent`
- `StatsEvent` includes CPU%, mem usage, net io, block io

---

## 5. Exec Session

Exec is interactive. Provide two-layer abstraction:

### 5.1 Engine-level

- `create_exec(id, ExecSpec) -> ExecHandle`
- `start_exec(handle) -> ExecSession`

`ExecSpec`:

- cmd: `Vec<String>`
- tty: bool
- stdin: bool
- env: optional
- user: optional (future)

`ExecSession`:

- `stdin: AsyncWrite`
- `stdout: AsyncRead`
- `stderr: AsyncRead` (if tty=false)
- `resize(tx,ty)` (if tty=true)

### 5.2 UI-level

UI decides how to render session:

- embedded terminal mode OR raw passthrough

---

## 6. Engine trait (core)

Proposed minimal interface:

Containers

- `list_containers(filter: ContainerFilter) -> Result<Vec<Container>>`
- `inspect_container(id: &ContainerId) -> Result<ContainerDetail>`
- `start_container(id) -> Result<()>`
- `stop_container(id, timeout: Option<Duration>) -> Result<()>`
- `restart_container(id, timeout: Option<Duration>) -> Result<()>`
- `delete_container(id, opts: DeleteContainerOpts) -> Result<()>`

Logs

- `stream_logs(id, opts: LogOpts) -> Result<LogStream>`

Exec

- `create_exec(id, spec: ExecSpec) -> Result<ExecHandle>`
- `start_exec(handle) -> Result<ExecSession>`

Stats (optional)

- `stream_stats(id, opts) -> Result<StatsStream>`

Images

- `list_images(filter) -> Result<Vec<Image>>`
- `inspect_image(id) -> Result<ImageDetail>`
- `pull_image(ref) -> Result<PullProgressStream | simple events>`
- `remove_image(id, force: bool) -> Result<()>`

Volumes

- `list_volumes(filter) -> Result<Vec<Volume>>`
- `inspect_volume(name) -> Result<VolumeDetail>`
- `remove_volume(name, force: bool) -> Result<()>`

Networks

- `list_networks(filter) -> Result<Vec<Network>>`
- `inspect_network(id) -> Result<NetworkDetail>`
- `remove_network(id) -> Result<()>`

System

- `ping() -> Result<()>`
- `engine_info() -> Result<EngineInfo>`

---

## 7. Connection abstraction (macOS first, future remote)

Tenor core should not assume unix socket only.

Define:

- `ConnectionTarget`:
  - `UnixSocket(PathBuf)`
  - `Tcp { host, tls: Option<TlsConfig> }`
  - `Context(String)` (future; resolved by adapter/config layer)

Resolution strategy (v0.1):

- Prefer explicit config/env:
  - `TENOR_DOCKER_HOST` (or reuse `DOCKER_HOST` — decide via ADR)
- Else: "current docker context" resolution (future-friendly)
  - v0.1 may call `docker context inspect` to resolve host (decision pending)

---

## 8. Adapter (tenor-docker) responsibilities

- Implement Engine trait using Docker Engine API
- Translate:
  - Docker container list/inspect -> core models
  - Docker logs -> LogEvent
  - Exec hijack/attach -> ExecSession
  - Stats stream -> StatsEvent
- Compose grouping relies on labels:
  - `com.docker.compose.*`

---

## 9. Open Engine Questions (to decide via ADR)

1) Docker context resolution:
   - parse docker config/meta files vs shell out to `docker context inspect`
2) Exec implementation:
   - use HTTP hijack for attach (Docker way) with tokio
   - how to map into terminal rendering
3) API client choice:
   - existing crate vs minimal custom client
4) Cancellation + timeout strategy:
   - per request timeouts
   - stream reconnection behavior
