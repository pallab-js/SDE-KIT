# Contributing to SDE-KIT

Welcome! SDE-KIT is an enterprise-grade, local-first SDLC Desktop Platform built specifically for solo developers to design, track, and execute software projects completely offline.

This document serves as the onboarding reference and style guide for developers wishing to contribute.

---

## Core Tenets

Every component in SDE-KIT must align with the strictly local-first and privacy-preserving design principles outlined in `AGENTS.md`:

1. **Local-First & Zero-Cloud**: All data must persist in the local SQLite database. Network-connected services, SaaS, JWT/Session authentication, and WebSockets are strictly prohibited.
2. **Airplane Mode Ready**: The entire platform must operate fully offline. External APIs requiring network credentials or tokens are forbidden.
3. **No Node `fs` in UI**: All file system operations must use custom Tauri Rust commands to guarantee absolute sandboxing and performance.
4. **Performance Budgets**: Key UI interactions (loading nodes, searching files, rendering lists) must execute within **100ms** on standard M1 (8GB) hardware.

---

## Technology Stack

- **Frontend**: Svelte 5 (Runes-based reactivity), Vite, CSS Variables (Harmonious Dark Theme).
- **Text Editing**: CodeMirror 6 with custom offline local syntax highlighting extensions only.
- **Backend**: Rust 1.77+, Tauri v2, SQLite with `rusqlite` + `r2d2` connection pooling.
- **Database Migrations**: `rusqlite_migration` schema management.
- **Build / Run Environment**: Powered by Bun.

---

## Project Structure

```
SDE-KIT/
├── apps/
│   └── desktop/               # Tauri & Svelte Desktop App
│       ├── src/               # Svelte 5 Frontend
│       │   ├── lib/
│       │   │   ├── components/# Reusable UI Components
│       │   │   ├── services/  # Tauri API bridge services
│       │   │   ├── stores/    # Svelte application stores
│       │   │   └── tests/     # Vitest coverage suites
│       │   └── routes/        # App View Shells
│       └── src-tauri/         # Rust Backend
│           ├── src/
│           │   ├── commands/  # Split domain bridge command files
│           │   ├── models/    # Domain data types
│           │   ├── persistence/# SQLite pool & migration logic
│           │   ├── watcher/   # Safe filesystem watchers
│           │   └── lib.rs     # Tauri startup & bridge setup
│           └── tests/         # Cargo integration tests
├── crates/
│   └── graph/                 # Dedicated Local Graph analysis crate
├── lefthook.yml               # Git pre-commit hooks orchestrator
└── package.json               # Monorepo and tool script runner
```

---

## Database Architecture & Migration Guidelines

Our persistence layers are centralized around an SQLite pool configured with optimal performance flags:

```rust
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
```

### Adding Schema Migrations
All schema updates are defined in `apps/desktop/src-tauri/src/persistence/migrations.rs`. When introducing a table or adding a column:
1. Append a new migration `M::up("...")` inside the migrations array.
2. Ensure index generation follows column inserts for rapid query scanning.
3. Do **not** modify past migration elements to maintain absolute database integrity.

---

## Coding Standards & Quality Checks

We utilize Git pre-commit hooks (`Lefthook`) to automatically assert code formatting and lint validity prior to staging commits.

### Svelte Style
- Prefer small Svelte components. Extract distinct cards (e.g. `TaskCard`, `MilestoneCard`) if a panel grows beyond **300 lines**.
- Enforce strict typing on component properties:
  ```svelte
  <script lang="ts">
    interface Props {
      itemId: string;
      onAction: () => void;
    }
    let { itemId, onAction }: Props = $props();
  </script>
  ```
- Use modern Svelte 5 `$state` and `$derived` runes for reactive variable definitions.

### Rust Style
- Ensure all command endpoints return `Result<T, AppError>` utilizing our unified serialization type `AppError`.
- Never suppress errors silently. Use structured logging `log::warn!` or `log::error!` for trace tracking.

### Validation Commands
Run these validation scripts before submitting pull requests:

```bash
# Verify ESLint & Prettier
bun run lint:js

# Run Vitest Frontend coverage
bun run test

# Run Rust compiler checks & tests
cargo clippy --all-targets --all-features
cargo test
```
