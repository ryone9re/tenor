# Tenor UI/UX Spec (v0.1 Draft)

Status: Draft  
Target: macOS / dockerd (Docker Engine API)  
UI Framework: Ratatui  
Top priority: UX quality (keyboard-first, responsive, safe)

---

## 1. UX Principles (Non-negotiables)

1) Keyboard-first: マウス不要で完結  
2) Responsive: I/O待ちでUIが固まらない  
3) Safe by default: 破壊的操作は必ず確認  
4) Discoverable: ヘルプ/コマンドパレット/キーバインド表示  
5) Debuggable: エラーは握りつぶさず、復旧手順を提示

---

## 2. Information Architecture (Screens)

Top-level: 5 tabs

- Containers (primary)
- Images
- Volumes
- Networks
- System (connection/status/about)

### 2.1 Containers screen (primary)

3-pane layout (default)

- Left: List (search/filter + items)
- Right-top: Details (summary/inspect)
- Right-bottom: Mode switcher
  - Logs
  - Stats (if supported / optional view)
  - Exec (launch)

### 2.2 Images / Volumes / Networks

- List + Details (2-pane)
- Delete/remove is always confirmed

### 2.3 System screen

- Current connection target (context/host)
- Engine info (version, API version)
- Diagnostics (permissions, socket reachability)
- Config path + effective config preview

---

## 3. Navigation & Focus Model

### 3.1 Focusable areas

- Tabs bar
- List pane
- Details pane
- Logs pane (when active)
- Command palette / Modal dialogs

### 3.2 Focus rules

- Default focus: List pane
- `Tab` cycles focus forward, `Shift+Tab` backward
- When Logs is opened: focus moves to Logs pane
- When Modal opened: focus trapped until close

### 3.3 Selection model

- Single selection only (v0.1)
- Selected entity is the "active target" for actions (start/stop/exec/logs)

---

## 4. Keybindings (v0.1 target)

Global

- Quit: `q` (if no modal); if modal open -> closes modal
- Help overlay: `?`
- Command palette: `:`
- Refresh: `R`
- Tabs: `1..5` or `H/L` (or `g` + number)  ※決定はADRへ

List navigation

- Move: `↑↓` / `j k`
- Page: `PgUp/PgDn` / `Ctrl+u / Ctrl+d`
- Top/Bottom: `g` / `G`
- Open details: `Enter`

Container actions (on selected container)

- Start: `s`
- Stop: `t`
- Restart: `r`
- Delete: `d` (opens confirm modal)
- Logs: `l` (opens logs view)
- Exec: `e` (opens exec launcher)
- Inspect raw: `i`

Logs view

- Follow toggle: `f`
- Pause toggle: `p`
- Search: `/` (opens search prompt)
- Jump to end: `G`
- Copy mode (optional): `y` to copy selected line  ※v0.1では保留でもOK

Images/Volumes/Networks

- Remove: `d` (confirm modal)
- Inspect: `Enter`

---

## 5. Search / Filter / Sort

### 5.1 List search

- `/` in list opens "filter query"
- Query matches:
  - name
  - image name (containers)
  - labels (optional)
- Default: case-insensitive contains

### 5.2 Filters (Containers)

- Status filters: running / exited / paused / restarting
- Compose grouping: enabled toggle (see §7)

### 5.3 Sort

- Default sort:
  - Containers: running first, then name
  - Images: created_at desc
- Sort control: command palette actions
  - `:sort name`
  - `:sort created`

---

## 6. Modal / Confirmation UX (Safety)

Destructive operations require confirmation:

- container delete
- image remove
- volume remove
- network remove
- prune (future)

Confirm modal includes:

- Target name + id short
- Consequence statement
- Default focus on "Cancel"
- Confirm: `Enter` or typing `delete` for high-impact ops (volume remove等)
  - v0.1: container/image remove = Enter confirm
  - v0.1: volume/network remove = type-to-confirm (提案)

---

## 7. Compose UX (v0.1 lightweight)

Goal: docker compose を「プロジェクト単位」で把握しやすくする。

Approach:

- Group containers by labels:
  - `com.docker.compose.project`
  - `com.docker.compose.service`
  - `com.docker.compose.oneoff`
- UI:
  - list shows tree-like grouping
    - project
      - service containers
- Toggle:
  - `c` to toggle compose grouping on/off (案)

Fallback:

- labels absent => normal flat list

---

## 8. Logs UX details

### 8.1 Streaming & buffering

- Follow mode uses streaming.
- Buffer policy:
  - keep last N lines (default 2,000; configurable)
  - drop oldest when exceeding
- Backpressure:
  - do not render per line; render on tick (e.g., 30-60fps max)

### 8.2 Search

- `/` opens prompt
- search highlights matches
- `n` next, `N` prev (案)

### 8.3 Timestamps

- toggle timestamps: `T` (案)

---

## 9. Exec UX details

Exec launcher modal:

- Command preset:
  - `/bin/bash` then fallback `/bin/sh`
- Options:
  - TTY on/off
  - user (optional; v0.1 may omit)
- Confirm launches interactive session in terminal-like pane or switches to "raw terminal mode"

Note:

- Exec is UX-critical. Session management and terminal mode will be detailed in Engine Spec.

---

## 10. Error UX

Error categories shown as:

- Toast (short)
- Details drawer (expandable) with:
  - summary
  - suggested action (e.g., "Docker daemon is not reachable. Check context/host.")
  - raw error (copyable)

Special cases:

- daemon unreachable: show banner + System tab shortcut
- permission denied: show guidance (group membership / socket perms)

---

## 11. Config UX (v0.1)

- Show effective connection target in status bar:
  - `[host: desktop-linux]` or `[host: unix:///...]`
- Show refresh interval
- show current tab + selected entity

---

## 12. Open UX Questions (to decide via ADR)

1) Tabs switching key: `H/L` vs `1..5` vs both
2) Confirm policy: volume/network remove の type-to-confirm を v0.1から入れるか
3) Exec 表示: TUI内埋め込み terminal vs 外部端末起動（どちら優先にするか）
4) Logs search: regex対応は v0.1に入れるか（まずは substringのみ？）
