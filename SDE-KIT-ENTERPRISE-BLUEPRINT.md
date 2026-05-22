# SDE-KIT: Enterprise-Grade Production Blueprint

> **Purpose:** Feed this document to an AI (Claude, Cursor, etc.) to implement every fix, upgrade, and structural change needed to make SDE-KIT production-ready and enterprise-grade. Each section is self-contained and actionable.

---

## Table of Contents

1. [Critical Bugs (Fix First)](#1-critical-bugs-fix-first)
2. [Database & Persistence](#2-database--persistence)
3. [Rust Backend Architecture](#3-rust-backend-architecture)
4. [Security Hardening](#4-security-hardening)
5. [Frontend Architecture](#5-frontend-architecture)
6. [Error Handling Strategy](#6-error-handling-strategy)
7. [CI/CD Pipeline](#7-cicd-pipeline)
8. [Testing Strategy](#8-testing-strategy)
9. [Release & Distribution](#9-release--distribution)
10. [Observability & Crash Reporting](#10-observability--crash-reporting)
11. [Code Organization Refactor](#11-code-organization-refactor)
12. [Performance](#12-performance)
13. [Documentation Standards](#13-documentation-standards)
14. [Implementation Order](#14-implementation-order)

---

## 1. Critical Bugs (Fix First)

These are active defects that corrupt data or break features today.

### 1.1 Phantom Tauri Commands in `local-db.ts`

**Severity: CRITICAL — entire export feature is broken and will throw at runtime.**

`apps/desktop/src/lib/services/database/local-db.ts` calls these Tauri commands that do NOT exist in the Rust backend:
- `db_init`
- `db_query`
- `db_export_sqlite`
- `get_project` (only `get_projects` exists)

**Fix:** Either implement these commands in Rust, or rewrite `LocalDatabase` to use the existing commands.

Implement the following Rust commands in `apps/desktop/src-tauri/src/commands/export.rs`:

```rust
#[tauri::command]
pub fn export_project_json(
    project_id: String,
    db: State<Database>,
) -> Result<String, AppError> {
    let conn = db.conn.lock()?;
    // query project, tasks, milestones, graph nodes/edges
    // serialize to JSON string and return
    // frontend converts to Blob
}

#[tauri::command]
pub fn export_project_sqlite(
    project_id: String,
    db: State<Database>,
    app: AppHandle,
) -> Result<Vec<u8>, AppError> {
    // copy the SQLite file bytes after a checkpoint
    // return as Vec<u8> for the frontend to write via save-dialog
}
```

Register both in `lib.rs` invoke_handler.

Rewrite `LocalDatabase` in TypeScript to use only real commands. Remove `ConflictResolution` type (no sync, no conflict — AGENTS.md principle).

---

### 1.2 File Watcher Thread Leak

**Severity: HIGH — each workspace switch spawns a new OS watcher thread; old ones never stop.**

`apps/desktop/src-tauri/src/watcher.rs`: `start_watching()` spawns a new thread + watcher every call. The previous watcher/thread pair has no shutdown signal.

**Fix:** Use a cancellation token pattern.

```rust
// watcher.rs
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

pub struct WatcherHandle {
    cancel: Arc<AtomicBool>,
}

impl Drop for WatcherHandle {
    fn drop(&mut self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}

pub fn start_watching(app: AppHandle, path: String) -> Result<WatcherHandle, String> {
    let cancel = Arc::new(AtomicBool::new(false));
    let cancel_clone = cancel.clone();
    thread::spawn(move || {
        // existing watcher logic — check cancel_clone.load() in loop
    });
    Ok(WatcherHandle { cancel })
}
```

Store `WatcherHandle` in `AppState`. When `set_workspace_root` is called, drop the old handle before creating a new one. This triggers `WatcherHandle::drop()` and signals the old thread to exit.

---

### 1.3 Non-Transactional `delete_project`

**Severity: HIGH — partial deletes leave orphaned rows on failure.**

`commands/mod.rs` `delete_project` runs 3 separate `conn.execute()` calls with no transaction. If the process crashes between calls, data is corrupt.

**Fix:**

```rust
#[tauri::command]
pub fn delete_project(id: String, db: State<Database>) -> Result<(), AppError> {
    let conn = db.conn.lock()?;
    let tx = conn.unchecked_transaction()?;
    tx.execute("DELETE FROM tasks WHERE project_id = ?1", params![id])?;
    tx.execute("DELETE FROM milestones WHERE project_id = ?1", params![id])?;
    tx.execute("DELETE FROM projects WHERE id = ?1", params![id])?;
    tx.commit()?;
    Ok(())
}
```

Apply the same pattern to `save_graph` in `persistence/mod.rs`.

---

### 1.4 `save_graph` Not Called on App Close

**Severity: HIGH — graph data is lost on crash or forced quit.**

The in-memory `GraphState` (`Mutex<Graph>`) is saved only when graph commands mutate it via explicit `save_graph` calls. There is no `on_close_requested` handler in `lib.rs`.

**Fix:** Add a close handler in `lib.rs`:

```rust
.on_window_event(|window, event| {
    if let tauri::WindowEvent::CloseRequested { .. } = event {
        let app = window.app_handle();
        let db = app.state::<Database>();
        let graph_state = app.state::<GraphState>();
        let conn = db.conn.lock().expect("db lock on close");
        let g = graph_state.0.lock().expect("graph lock on close");
        let snap = g.snapshot();
        let _ = crate::persistence::save_graph(&conn, &snap);
    }
})
```

---

### 1.5 Silent Row-Error Suppression in DB Queries

**Severity: MEDIUM — corrupted or mismatched rows silently vanish.**

Every query uses `.filter_map(|r| r.ok())`. A deserialisation error (e.g., unexpected NULL, schema mismatch after partial migration) silently drops rows.

**Fix:** Collect errors and log them:

```rust
let mut tasks = Vec::new();
for row_result in stmt.query_map(...)? {
    match row_result {
        Ok(t) => tasks.push(t),
        Err(e) => log::warn!("Skipping malformed task row: {e}"),
    }
}
```

---

### 1.6 `evictLruIfNeeded` Never Called

**Severity: MEDIUM — unbounded memory growth in editor store.**

`apps/desktop/src/lib/stores/editor.ts` defines `evictLruIfNeeded()` but it is never called. The `fileContents` map grows without bound as files are opened.

**Fix:** Call `evictLruIfNeeded()` inside `setFileContent()`:

```typescript
export function setFileContent(path: string, content: string) {
    fileContents.update((map) => {
        // ... existing logic ...
        return new Map(map);
    });
    evictLruIfNeeded(); // add this
}
```

---

### 1.7 Arbitrary Filesystem Access Without Workspace Root

**Severity: HIGH — security boundary bypass.**

`check_root()` in `commands/fs.rs` has a fallback when `WorkspaceRoot` is `None` that allows `write_file`, `delete_file`, and `read_file` to operate on any absolute path the frontend provides.

**Fix:** Enforce workspace root requirement for write/delete:

```rust
fn require_root(root: &Option<PathBuf>) -> Result<&PathBuf, AppError> {
    root.as_ref().ok_or(AppError::NoWorkspaceRoot)
}

#[tauri::command]
pub fn write_file(path: String, content: String, root: State<WorkspaceRoot>) -> Result<(), AppError> {
    let root_guard = root.0.lock()?;
    let base = require_root(&root_guard)?;
    let resolved = resolve_within_root(base, &path)?;
    drop(root_guard);
    std::fs::write(&resolved, &content)?;
    Ok(())
}
```

`read_file` may optionally allow absolute paths, but must still sanitize.

---

## 2. Database & Persistence

### 2.1 Replace Ad-Hoc Migration with `rusqlite_migration`

**Current state:** A single `schema_version` integer in a table, migrations done with `if ver < N { conn.execute_batch(...).ok() }`. The `.ok()` swallows migration failures silently.

**Fix:** Add `rusqlite_migration` crate and define migrations as versioned constants.

```toml
# apps/desktop/src-tauri/Cargo.toml
rusqlite_migration = "1"
```

```rust
// persistence/migrations.rs
use rusqlite_migration::{Migrations, M};

pub fn migrations() -> Migrations<'static> {
    Migrations::new(vec![
        M::up(include_str!("sql/001_initial.sql")),
        M::up(include_str!("sql/002_add_milestone_id_to_tasks.sql")),
        // add new migrations here as files, never edit existing ones
    ])
}
```

Create `persistence/sql/001_initial.sql` with the full schema DDL from the current `execute_batch` string. Create `002_add_milestone_id_to_tasks.sql` with the `ALTER TABLE tasks ADD COLUMN milestone_id ...` statement.

In `Database::new()`:

```rust
let mut conn = Connection::open(db_path)?;
crate::persistence::migrations::migrations()
    .to_latest(&mut conn)
    .expect("DB migration failed"); // fail fast on startup — better than silent corruption
```

Remove `schema_version` table management from application code entirely.

---

### 2.2 Connection Pool (Replace Single Mutex)

**Current state:** `Mutex<Connection>` — one connection, serializes all DB access.

**Fix:** Use `r2d2` + `r2d2_sqlite`:

```toml
r2d2 = "0.8"
r2d2_sqlite = "0.24"
```

```rust
// persistence/mod.rs
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub struct Database {
    pub pool: Pool<SqliteConnectionManager>,
}

impl Database {
    pub fn new(app_dir: &Path) -> Result<Self, DbError> {
        let manager = SqliteConnectionManager::file(app_dir.join("sde-kit.db"))
            .with_init(|conn| {
                conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;
                Ok(())
            });
        let pool = Pool::builder().max_size(4).build(manager)?;
        // run migrations on a single connection first
        let mut conn = pool.get()?;
        migrations::migrations().to_latest(&mut conn)?;
        Ok(Database { pool })
    }
}
```

Update all commands to use `db.pool.get()?` instead of `db.conn.lock()`.

---

### 2.3 Pagination for All List Queries

**Current state:** `get_tasks()` fetches ALL rows. At 10,000+ tasks this is a memory and latency problem.

**Fix:** Add `limit` and `offset` parameters to all list commands:

```rust
#[tauri::command]
pub fn get_tasks(
    limit: Option<i64>,
    offset: Option<i64>,
    db: State<Database>,
) -> Result<Vec<Task>, AppError> {
    let limit = limit.unwrap_or(200).min(500);
    let offset = offset.unwrap_or(0);
    // SELECT ... ORDER BY created_at DESC LIMIT ?1 OFFSET ?2
}
```

Update frontend `api.ts` to pass pagination params. Default first page: limit 200.

---

### 2.4 Notes Storage — Dedicated Table

**Current state:** Notes stored as `note:{id}` keys in `workspace_state` (a generic key-value table). Mixing concerns; no ability to list all notes without a full table scan.

**Fix:** Add migration `003_notes_table.sql`:

```sql
CREATE TABLE IF NOT EXISTS notes (
    id   TEXT PRIMARY KEY,
    title TEXT NOT NULL DEFAULT '',
    content TEXT NOT NULL DEFAULT '',
    project_id TEXT REFERENCES projects(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

Add Rust commands: `get_notes`, `get_note`, `create_note`, `update_note`, `delete_note`. Remove `get_note`/`save_note` workarounds in `commands/mod.rs`.

---

## 3. Rust Backend Architecture

### 3.1 Unified Error Type

**Current state:** Every command returns `Result<T, String>`. Frontend cannot distinguish error types; cannot display localized or actionable messages.

**Fix:** Create `src/error.rs`:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Pool error: {0}")]
    Pool(#[from] r2d2::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("No workspace root set")]
    NoWorkspaceRoot,

    #[error("Path traversal denied")]
    PathTraversal,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Lock poisoned")]
    LockPoisoned,
}

impl serde::Serialize for AppError {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = s.serialize_struct("AppError", 2)?;
        state.serialize_field("code", &self.error_code())?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

impl AppError {
    fn error_code(&self) -> &'static str {
        match self {
            AppError::NoWorkspaceRoot => "NO_WORKSPACE_ROOT",
            AppError::PathTraversal   => "PATH_TRAVERSAL",
            AppError::NotFound(_)     => "NOT_FOUND",
            AppError::Validation(_)   => "VALIDATION_ERROR",
            _                         => "INTERNAL_ERROR",
        }
    }
}
```

Replace all `Result<T, String>` with `Result<T, AppError>` in every command. Update frontend `api.ts` error handling to check `error.code`.

---

### 3.2 Split `commands/mod.rs` into Modules

**Current state:** 400+ line monolith handles projects, tasks, milestones, notes, workspace state.

**Target structure:**

```
commands/
  mod.rs           ← re-exports only
  projects.rs      ← get_projects, create_project, update_project, delete_project
  tasks.rs         ← get_tasks, get_tasks_by_project, create_task, update_task, update_task_status, delete_task
  milestones.rs    ← get_milestones, create_milestone, update_milestone_status, delete_milestone, assign_task_to_milestone
  notes.rs         ← get_notes, get_note, create_note, update_note, delete_note
  workspace.rs     ← get_workspace_state, set_workspace_state
  export.rs        ← export_project_json, export_project_sqlite
  fs.rs            ← (already separate — keep)
  graph.rs         ← (already separate — keep)
```

Each file holds only its domain commands plus private helpers. Move `parse_status`, `parse_priority`, `parse_milestone_status` into `src/models/mod.rs` as `impl` methods (`TaskStatus::from_str`, etc.), using `std::str::FromStr`.

---

### 3.3 Input Validation

**Current state:** No length checks, no sanitization. A 10MB task title can be inserted.

**Fix:** Add validation in each create/update command:

```rust
fn validate_title(title: &str) -> Result<(), AppError> {
    if title.trim().is_empty() {
        return Err(AppError::Validation("Title cannot be empty".into()));
    }
    if title.len() > 500 {
        return Err(AppError::Validation("Title too long (max 500 chars)".into()));
    }
    Ok(())
}
```

Apply to: project name, task title, milestone title, note title.

---

### 3.4 Structured Logging in Release Builds

**Current state:** `tauri_plugin_log` only initialized under `cfg!(debug_assertions)`. Release builds are silent.

**Fix:**

```rust
.setup(|app| {
    let log_dir = app.path().app_log_dir().expect("log dir");
    app.handle().plugin(
        tauri_plugin_log::Builder::default()
            .level(if cfg!(debug_assertions) {
                log::LevelFilter::Debug
            } else {
                log::LevelFilter::Warn
            })
            .targets([
                tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                    file_name: Some("sde-kit".into()),
                }),
                tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stderr),
            ])
            .max_file_size(5_000_000) // 5MB rotate
            .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
            .build(),
    )?;
    Ok(())
})
```

Add `tauri-plugin-log` to dependencies unconditionally (remove the `if cfg!(debug_assertions)` guard on the plugin init).

---

## 4. Security Hardening

### 4.1 Remove `unsafe-inline` from CSP

**Current state:** `tauri.conf.json` CSP: `style-src 'self' 'unsafe-inline'`.

**Fix:** TailwindCSS v4 (CSS-first) generates a static stylesheet. Serve it as a file. Use a nonce if dynamic styles are absolutely required. Replace with:

```json
"csp": "default-src 'self'; style-src 'self'; script-src 'self'; img-src 'self' data:; font-src 'self' data:"
```

If Svelte/CodeMirror injects inline styles that can't be avoided, use a `nonce` or `sha256` hash for those specific styles instead of a blanket `unsafe-inline`.

---

### 4.2 Tauri Capabilities — Least Privilege

**Current state:** Check `apps/desktop/src-tauri/capabilities/`. Ensure no over-broad permissions.

**Fix:** Audit `default.json` capability file. Ensure:
- `fs:allow-read-file` is scoped to app data dir and workspace only.
- `shell:allow-open` is restricted to safe URL schemes (`https://`) only.
- No `fs:allow-write` for paths outside the workspace.

Example scope restriction:

```json
{
  "identifier": "fs:allow-read-file",
  "allow": [{ "path": "$APPDATA/**" }, { "path": "$HOME/**" }]
}
```

---

### 4.3 Rate-Limit Expensive Commands

`search_in_files` can be expensive on large workspaces. Add a debounce on the frontend (300ms) and a Rust-side guard:

```rust
// In AppState, add:
search_in_progress: AtomicBool,

// In search_in_files:
if state.search_in_progress.swap(true, Ordering::SeqCst) {
    return Err(AppError::Validation("Search already in progress".into()));
}
// ... search ...
state.search_in_progress.store(false, Ordering::SeqCst);
```

---

### 4.4 Binary File Guard in `search_in_files`

**Current state:** `std::fs::read_to_string()` silently skips binary files (returns `Err` which is ignored). But for UTF-8 files with embedded nulls it may hang or produce garbage.

**Fix:** Check file size before reading; skip files > 1MB:

```rust
let meta = std::fs::metadata(&path).map_err(|e| e.to_string())?;
if meta.len() > 1_024_000 {
    continue; // skip large files
}
```

Also cap total results at 200 (currently 500) to avoid large IPC payloads.

---

## 5. Frontend Architecture

### 5.1 Fix Static Import in `api.ts`

**Current state:**

```typescript
async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
    const { invoke } = await import('@tauri-apps/api/core'); // dynamic import on every call
    return await invoke<T>(cmd, args);
}
```

**Fix:**

```typescript
import { invoke as tauriInvoke } from '@tauri-apps/api/core';

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
    return tauriInvoke<T>(cmd, args);
}
```

---

### 5.2 Typed Error Handling in `api.ts`

**Current state:** All errors become `new Error('API not available')`, losing the structured `AppError` from Rust.

**Fix:**

```typescript
export interface ApiError {
    code: string;
    message: string;
}

export function isApiError(e: unknown): e is ApiError {
    return typeof e === 'object' && e !== null && 'code' in e && 'message' in e;
}

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
    try {
        return await tauriInvoke<T>(cmd, args);
    } catch (e) {
        if (isApiError(e)) throw e;
        throw { code: 'UNKNOWN', message: String(e) } satisfies ApiError;
    }
}
```

Update all call sites to check `error.code` for user-facing messages (e.g., `NO_WORKSPACE_ROOT` → "Please open a workspace first").

---

### 5.3 Global Loading & Error Store

Add `apps/desktop/src/lib/stores/app.ts`:

```typescript
import { writable } from 'svelte/store';
import type { ApiError } from '$lib/services/api';

export const globalError = writable<ApiError | null>(null);
export const isLoading = writable<boolean>(false);

export function withLoading<T>(fn: () => Promise<T>): Promise<T> {
    isLoading.set(true);
    return fn().finally(() => isLoading.set(false));
}
```

Use `withLoading()` in component event handlers. Wire `globalError` to the existing `Toast` notification system.

---

### 5.4 Svelte Component Split Audit

Review all panel components (e.g., `TasksPanel.svelte`, `MilestonesPanel.svelte`). Each should be under 300 lines. Extract sub-components:
- `TaskCard.svelte` from `TasksPanel.svelte`
- `MilestoneCard.svelte` from `MilestonesPanel.svelte`
- `FileTreeNode.svelte` from `FileTree.svelte`

This enables isolated unit testing per component.

---

### 5.5 Strict TypeScript

`apps/desktop/tsconfig.json` — ensure these are set:

```json
{
  "compilerOptions": {
    "strict": true,
    "noUncheckedIndexedAccess": true,
    "exactOptionalPropertyTypes": true
  }
}
```

Fix any resulting type errors. Do not suppress with `// @ts-ignore`.

---

## 6. Error Handling Strategy

### 6.1 Rust Panic Hook — Write to Log Dir (Not Temp)

**Current state:** Crash log written to `std::env::temp_dir()` — ephemeral on some OSes.

**Fix:**

```rust
fn setup_panic_hook(log_dir: PathBuf) {
    std::panic::set_hook(Box::new(move |info| {
        // ... existing msg/location extraction ...
        let crash_path = log_dir.join(format!(
            "crash-{}.log",
            chrono::Utc::now().format("%Y%m%d-%H%M%S")
        ));
        let _ = std::fs::write(&crash_path, report);
        log::error!("PANIC written to {:?}", crash_path);
    }));
}
```

Call `setup_panic_hook(app.path().app_log_dir()?)` in `setup()` closure.

---

### 6.2 Frontend Error Boundary — Add to All Panels

`ErrorBoundary.svelte` exists but check that it wraps every panel in `MainContent.svelte`. If any panel crashes, only that panel should show the error; the rest of the app should remain functional.

```svelte
<!-- MainContent.svelte -->
<ErrorBoundary>
    {#if activePanel === 'tasks'}
        <TasksPanel />
    {/if}
</ErrorBoundary>
```

---

### 6.3 Watcher Panic — Thread Safety

`watcher.rs` calls `.expect()` inside the spawned thread. A panic there is silent (just kills the thread with no recovery).

**Fix:** Replace `.expect()` with proper error propagation. Send errors back via a channel:

```rust
match RecommendedWatcher::new(...) {
    Ok(mut w) => { /* proceed */ }
    Err(e) => {
        log::error!("Failed to create file watcher: {e}");
        return; // thread exits cleanly
    }
}
```

---

## 7. CI/CD Pipeline

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  test-rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - uses: Swatinem/rust-cache@v2
      - name: Fmt check
        run: cargo fmt --check
      - name: Clippy
        run: |
          cargo clippy --manifest-path apps/desktop/src-tauri/Cargo.toml -- -D warnings
          cargo clippy -p sde-kit-graph -- -D warnings
      - name: Tests
        run: cargo test -p sde-kit-graph

  test-frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: npm
      - run: npm ci
      - name: Type check
        run: npm run check -w apps/desktop
      - name: Unit tests
        run: npm run test -w apps/desktop
      - name: Lint
        run: npm run lint -w apps/desktop

  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Validate constraints
        run: bash scripts/validate.sh
```

Add `.github/workflows/release.yml` triggered on `push: tags: ['v*']` that builds on `macos-latest`, `ubuntu-latest`, `windows-latest` in a matrix.

---

### 7.1 Add ESLint + Prettier

**Current state:** `npm run lint` runs only `svelte-check` (type checking). No style or lint enforcement.

Install:

```bash
npm install -D eslint @typescript-eslint/eslint-plugin eslint-plugin-svelte prettier prettier-plugin-svelte -w apps/desktop
```

Create `apps/desktop/.eslintrc.cjs`:

```js
module.exports = {
  extends: ['eslint:recommended', 'plugin:@typescript-eslint/recommended', 'plugin:svelte/recommended'],
  rules: {
    '@typescript-eslint/no-explicit-any': 'error',
    'no-console': ['warn', { allow: ['warn', 'error'] }],
  }
};
```

Create `apps/desktop/.prettierrc`:

```json
{
  "semi": true,
  "singleQuote": true,
  "trailingComma": "es5",
  "plugins": ["prettier-plugin-svelte"],
  "overrides": [{ "files": "*.svelte", "options": { "parser": "svelte" } }]
}
```

Update `package.json` scripts:

```json
"lint": "eslint src --ext .ts,.svelte && prettier --check src",
"format": "prettier --write src"
```

---

### 7.2 Pre-commit Hooks with `lefthook`

```bash
npm install -D lefthook
```

Create `lefthook.yml`:

```yaml
pre-commit:
  parallel: true
  commands:
    rustfmt:
      glob: "*.rs"
      run: cargo fmt -- {staged_files}
    clippy:
      glob: "*.rs"
      run: cargo clippy --quiet
    prettier:
      glob: "*.{ts,svelte}"
      run: npx prettier --write {staged_files}
    eslint:
      glob: "*.{ts,svelte}"
      run: npx eslint {staged_files}
```

---

## 8. Testing Strategy

### 8.1 Rust — Command Integration Tests

Create `apps/desktop/src-tauri/src/tests/`:

```rust
// tests/commands_test.rs
#[cfg(test)]
mod tests {
    use crate::persistence::Database;
    use crate::commands::*;
    use tempfile::tempdir;

    fn test_db() -> Database {
        let dir = tempdir().unwrap();
        Database::new(dir.path()).unwrap()
    }

    #[test]
    fn test_create_and_get_project() {
        let db = test_db();
        // use tauri::test::mock_builder() + app.state::<Database>()
        // to test commands end-to-end
    }

    #[test]
    fn test_delete_project_cascades() {
        // create project, create task for it, delete project
        // verify task no longer exists
    }
}
```

Add `tempfile = "3"` to `[dev-dependencies]` in `Cargo.toml`.

Use `tauri::test::mock_builder()` to test Tauri commands without a real window.

---

### 8.2 Frontend — Vitest Component Tests

`vitest.config.ts` already exists. Add component tests:

```typescript
// src/lib/tests/stores.test.ts
import { describe, it, expect, vi } from 'vitest';
import { get } from 'svelte/store';
import { openTab, closeTab, openTabs, activeTabId } from '$lib/stores/workspace';

describe('workspace store', () => {
    it('opens a tab', () => { /* ... */ });
    it('closes active tab and selects next', () => { /* ... */ });
    it('does not duplicate tabs', () => { /* ... */ });
});
```

Mock Tauri IPC:

```typescript
// src/lib/tests/setup.ts
vi.mock('@tauri-apps/api/core', () => ({
    invoke: vi.fn().mockResolvedValue([]),
}));
```

Target: 80% coverage on all stores and utility functions.

---

### 8.3 E2E Tests with WebdriverIO + Tauri Driver

```bash
npm install -D @wdio/cli @tauri-apps/wdio-tauri-service
```

Create `apps/desktop/e2e/workspace.test.ts`:

```typescript
describe('Workspace', () => {
    it('opens a folder and shows file tree', async () => {
        // use tauri-driver to open app, click "Open Workspace", assert file tree visible
    });
    it('creates a task and shows it on the board', async () => { /* ... */ });
});
```

Add to CI after build step.

---

## 9. Release & Distribution

### 9.1 Code Signing

**macOS:** Add to `tauri.conf.json`:

```json
"macOS": {
    "minimumSystemVersion": "14.0",
    "signingIdentity": "Developer ID Application: YOUR NAME (TEAM_ID)",
    "notarize": {
        "teamId": "TEAM_ID"
    }
}
```

Set `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID` as GitHub Actions secrets.

**Windows:** Add `"certificateThumbprint": "${{ secrets.WINDOWS_CERT_THUMBPRINT }}"` to `tauri.conf.json` bundle config.

---

### 9.2 Auto-Updater

Add `tauri-plugin-updater` to enable silent background update checking:

```toml
tauri-plugin-updater = "2"
```

```json
// tauri.conf.json
"updater": {
    "active": true,
    "endpoints": ["https://github.com/pallab-js/SDE-KIT/releases/latest/download/latest.json"],
    "dialog": false,
    "pubkey": "YOUR_PUBKEY"
}
```

In frontend, show a non-intrusive "Update available — restart to apply" toast. Never force-restart.

---

### 9.3 CHANGELOG and Semantic Versioning

Install `conventional-changelog-cli`:

```bash
npm install -D conventional-changelog-cli
```

Add script:

```json
"changelog": "conventional-changelog -p angular -i CHANGELOG.md -s"
```

Use `release-please` GitHub Action to auto-bump versions and generate CHANGELOG entries from conventional commits.

---

### 9.4 Binary Size Budget

The release binary should stay under 50MB. Add to `scripts/validate.sh`:

```bash
RELEASE_BIN="apps/desktop/src-tauri/target/release/sde-kit"
if [ -f "$RELEASE_BIN" ]; then
  size=$(stat -c%s "$RELEASE_BIN" 2>/dev/null || stat -f%z "$RELEASE_BIN")
  limit=$((50 * 1024 * 1024))
  if [ "$size" -gt "$limit" ]; then
    echo "❌ Binary too large: $((size / 1024 / 1024))MB (limit 50MB)"
    errors=$((errors + 1))
  fi
fi
```

---

## 10. Observability & Crash Reporting

Since cloud services are prohibited by AGENTS.md, all observability must be local.

### 10.1 Local Crash Log Viewer

Add a "Diagnostics" screen in the app (accessible via Command Palette → "View Crash Logs") that reads log files from the app log directory using a new Tauri command:

```rust
#[tauri::command]
pub fn get_crash_logs(app: AppHandle) -> Result<Vec<String>, AppError> {
    let log_dir = app.path().app_log_dir()?;
    // read crash-*.log files, return content sorted by date desc
}
```

---

### 10.2 Performance Metrics Store

Add to `src/lib/stores/perf.ts`:

```typescript
interface PerfEntry {
    cmd: string;
    duration_ms: number;
    timestamp: string;
}

// wrap invoke() to measure duration
export async function timedInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
    const t0 = performance.now();
    try {
        return await invoke<T>(cmd, args);
    } finally {
        const dur = performance.now() - t0;
        if (dur > 100) { // log anything over 100ms target
            console.warn(`[perf] ${cmd} took ${dur.toFixed(1)}ms`);
        }
    }
}
```

---

## 11. Code Organization Refactor

### 11.1 Rust File Layout (Target)

```
apps/desktop/src-tauri/src/
  main.rs
  lib.rs                    ← app setup only
  error.rs                  ← AppError
  state.rs                  ← AppState struct aggregating all managed state
  commands/
    mod.rs                  ← re-exports
    projects.rs
    tasks.rs
    milestones.rs
    notes.rs
    workspace.rs
    export.rs
    fs.rs
    graph.rs
  models/
    mod.rs
    project.rs
    task.rs
    milestone.rs
    note.rs
  persistence/
    mod.rs                  ← Database struct, pool setup
    migrations.rs           ← Migrations list
    sql/
      001_initial.sql
      002_add_milestone_id.sql
      003_notes_table.sql
    graph.rs                ← save_graph, load_graph
  watcher.rs
```

### 11.2 Frontend File Layout (Target)

```
src/
  lib/
    components/
      editor/
        CodeEditor.svelte
        DiffViewer.svelte
        SplitEditor.svelte      ← extract from MainContent
      panels/
        TasksPanel.svelte
        TaskCard.svelte         ← new
        MilestonesPanel.svelte
        MilestoneCard.svelte    ← new
        NotesPanel.svelte
        GraphPanel.svelte
        GitPanel.svelte
        SearchPanel.svelte
        ProjectsPanel.svelte
        PomodoroPanel.svelte
      shell/
        ActivityBar.svelte
        CommandBar.svelte
        CommandPalette.svelte
        Sidebar.svelte
        BottomPanel.svelte
        ErrorBoundary.svelte
        KeyboardShortcuts.svelte
    services/
      api.ts                    ← core invoke wrapper
      api/
        projects.ts
        tasks.ts
        milestones.ts
        notes.ts
        fs.ts
        graph.ts
        export.ts
      gitService.ts
      keyboard.ts
      undoManager.ts
    stores/
      workspace.ts
      editor.ts
      commands.ts
      notifications.ts
      pomodoro.ts
      theme.ts
      layout.ts
      app.ts                    ← new: globalError, isLoading
    types/
      index.ts
      api.ts                    ← ApiError, DBResult
    utils/
      assertions.ts
    design/
      tokens.ts
      css-variables.ts
```

---

## 12. Performance

### 12.1 Virtual Scrolling for Large Lists

TasksPanel and FileTree must use virtual scrolling when item count > 100.

Install `svelte-virtual-list` or implement with `IntersectionObserver`:

```svelte
<!-- TasksPanel.svelte -->
<script>
  import VirtualList from 'svelte-virtual-list'; // or roll own
</script>
<VirtualList items={tasks} let:item>
  <TaskCard task={item} />
</VirtualList>
```

---

### 12.2 Debounce `search_in_files`

Frontend search input must debounce at 300ms before invoking `search_in_files`. Add to `SearchPanel.svelte`:

```typescript
let searchTimeout: ReturnType<typeof setTimeout>;
function onSearchInput(query: string) {
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(() => doSearch(query), 300);
}
```

---

### 12.3 Lazy-Load Heavy Panels

CodeMirror, graph canvas, and git diff viewer are heavy. Only import/mount them when their panel is first activated:

```svelte
{#if activePanel === 'editor'}
  {#await import('$lib/components/editor/CodeEditor.svelte') then { default: CodeEditor }}
    <svelte:component this={CodeEditor} />
  {/await}
{/if}
```

---

### 12.4 Graph Layout — Off-main-thread

The force-directed layout runs on the Rust side (correct!) but the IPC round-trip blocks the UI for large graphs. After computing, stream positions incrementally:

```rust
// Instead of returning all positions at once, emit Tauri events per batch:
app.emit("graph-positions-batch", &batch)?;
```

On the frontend, update the canvas incrementally on each event.

---

## 13. Documentation Standards

### 13.1 Create `CONTRIBUTING.md`

```markdown
# Contributing to SDE-KIT

## Prerequisites
- Node.js 22+
- Rust 1.77+
- Tauri CLI 2.x

## Setup
npm install && npm run dev

## Principles (from AGENTS.md — read before coding)
- Local-first. No cloud. No network. Airplane-mode test.
- All DB ops via Tauri commands (never frontend localStorage).
- Performance target: <100ms for all core actions.

## Commit Convention
Use Conventional Commits: feat:, fix:, refactor:, docs:, test:, chore:

## PR Checklist
- [ ] cargo clippy passes with no warnings
- [ ] cargo test passes
- [ ] svelte-check passes
- [ ] ESLint + Prettier pass
- [ ] New feature has unit tests
- [ ] AGENTS.md principles not violated (no cloud, no auth, no docker)
```

---

### 13.2 JSDoc on All Public API Functions

Every function exported from `api.ts` modules must have JSDoc:

```typescript
/**
 * Create a new task in the local database.
 * @param title - Required. Max 500 chars.
 * @param projectId - Optional. If provided, task belongs to this project.
 * @throws {ApiError} with code 'VALIDATION_ERROR' if title is empty.
 */
export function createTask(title: string, ...): Promise<Task> { ... }
```

---

### 13.3 Rust `doc` Comments on All Public Items

```rust
/// Compute and return a force-directed layout for the current graph.
///
/// # Errors
/// Returns `AppError::LockPoisoned` if the graph state mutex is poisoned.
#[tauri::command]
pub fn compute_graph_layout(...) -> Result<Vec<NodePosition>, AppError> { ... }
```

Run `cargo doc --no-deps --open` to verify docs compile and render.

---

## 14. Implementation Order

Execute in this sequence to avoid regressions. Each phase is a discrete PR.

| Phase | Title | Files Changed | Priority |
|-------|-------|--------------|----------|
| 1 | **Fix critical bugs** | `commands/mod.rs`, `fs.rs`, `watcher.rs`, `persistence/mod.rs`, `local-db.ts`, `editor.ts` | 🔴 Blocker |
| 2 | **Unified error type** | `error.rs` (new), all commands, `api.ts` | 🔴 Blocker |
| 3 | **Migration system** | `persistence/migrations.rs`, `persistence/sql/*.sql`, `Cargo.toml` | 🔴 Blocker |
| 4 | **Connection pool** | `persistence/mod.rs`, all commands | 🟠 High |
| 5 | **Command module split** | `commands/**` | 🟠 High |
| 6 | **Security: CSP + fs guards** | `tauri.conf.json`, `commands/fs.rs`, capabilities | 🟠 High |
| 7 | **Structured logging** | `lib.rs` | 🟠 High |
| 8 | **CI pipeline** | `.github/workflows/ci.yml` | 🟠 High |
| 9 | **ESLint + Prettier + lefthook** | `apps/desktop/package.json`, configs | 🟡 Medium |
| 10 | **Pagination** | `commands/tasks.rs`, `commands/projects.rs`, `api.ts` | 🟡 Medium |
| 11 | **Notes table migration** | `persistence/sql/003_notes_table.sql`, `commands/notes.rs` | 🟡 Medium |
| 12 | **Export commands** | `commands/export.rs`, `local-db.ts` rewrite | 🟡 Medium |
| 13 | **Frontend stores + error UX** | `stores/app.ts`, components | 🟡 Medium |
| 14 | **TypeScript strict mode** | `tsconfig.json`, all `.ts`/`.svelte` | 🟡 Medium |
| 15 | **Component split** | `TaskCard.svelte`, `MilestoneCard.svelte`, etc. | 🟡 Medium |
| 16 | **Rust tests** | `src-tauri/src/tests/` | 🟡 Medium |
| 17 | **Frontend component tests** | `src/lib/tests/` | 🟡 Medium |
| 18 | **Virtual scrolling** | `TasksPanel.svelte`, `FileTree.svelte` | 🟢 Low |
| 19 | **Code signing + updater** | `tauri.conf.json`, `.github/workflows/release.yml` | 🟢 Low |
| 20 | **CONTRIBUTING.md + JSDoc + rustdoc** | `CONTRIBUTING.md`, all public APIs | 🟢 Low |
| 21 | **E2E tests** | `e2e/` | 🟢 Low |

---

## Appendix: Key Invariants to Preserve

The following AGENTS.md rules must be enforced at every phase:

1. Every feature must work with airplane mode ON.
2. No network dependencies introduced (no `reqwest`, `hyper`, `tokio-tungstenite`).
3. No authentication layer (no JWT, OAuth, sessions).
4. All data stored only in local SQLite or on the local filesystem.
5. Single executable — no daemon, no sidecar server.
6. Performance target: <100ms for all core user actions on M1 8GB.

Do not implement any feature that violates these invariants, even if the implementation is otherwise technically sound.
