# SDE Kit

A **local-first SDLC desktop platform** — manage your entire software development lifecycle completely offline, with zero cloud dependencies.

SDE-KIT is engineered as an enterprise-grade, privacy-centric workspace for solo developers to organize repositories, track tasks, manage milestones, take notes, compile code relations, and write software securely in a fully offline desktop environment.

Built with **Rust + Tauri v2 + Svelte 5 (Runes) + Vanilla CSS Variables**.

---

## Key Enterprise Features

- **📂 Workspace Management** — Open and index project folders with full file tree navigation and safe filesystem sandboxing constraints.
- **💻 Integrated Editor** — High-speed, side-by-side editing utilizing CodeMirror 6 with local offline syntax highlighting extensions.
- **⊞ Virtual Kanban Board** — Drag-and-drop task boards optimized via IntersectionObserver-based virtual scrolling for smooth rendering of large datasets.
- **🏁 Milestones Tracking** — Align tasks to milestones and visually track completion percentages (via modular sub-components).
- **📝 Dedicated Local Notes** — Standalone scratchpad note cards linked directly to local SQLite projects.
- **🕸 Relationship Graphs** — Custom force-directed canvas layout engine written in Rust to visualize code relations.
- **⚡ Connection Pooling** — Concurrent thread-safe database connection pooling utilizing `r2d2` with Write-Ahead Logging (`WAL` journal) enabled.
- **🔄 Database Migrations** — Managed schemas via version-controlled `rusqlite_migration` upgrades.
- **💾 Offline Local Export** — One-click JSON and SQLite database exports generating direct object-URL downloads.
- **🚨 Unified Error & Loading UX** — Universal exception handler translating backend Rust errors into visual glassmorphic overlays and floating Toast alerts.
- **🛡 CSP Sandboxing** — Strict Content Security Policies and strict path verification guards (`check_root_strict`) to prevent unauthorized traversals.

---

## Architecture

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

## Development & Build Environment

The repository is fully optimized to run under **Bun** as a high-performance package and script execution runner.

### Prerequisites

- **Bun** (Recommended drop-in Node/NPM replacement)
- **Rust** 1.77+
- **Tauri CLI** 2.x — `cargo install tauri-cli`

### Running the App

```bash
# Install dependencies
bun install

# Run application in hot-reload development mode
bun run dev

# Run Svelte and TypeScript safety check
bun run check

# Check linting and formatting compliance
bun run lint:js
bun run format
```

### Testing Pipelines

```bash
# Run Svelte Vitest frontend suites
bun run test

# Run Rust unit and database integration tests
cargo test
```

### Production Release

```bash
# Compile the local optimized installer DMG/app bundle
bun run release
```

---

## Contributing

For coding styles, component refactoring thresholds, connection pooling rules, and database schema conventions, please consult the [CONTRIBUTING.md](CONTRIBUTING.md) onboarding reference.
