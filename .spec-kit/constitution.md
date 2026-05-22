# SDE-KIT Development Constitution

This document governs the development, architecture, code style, and validation principles of the SDE-KIT desktop application. Every new feature, bug fix, or refactor must strictly adhere to these rules.

---

## 1. Core Principles

* **Local-First**: All application data must persist exclusively in local storage or the local SQLite database.
* **Offline-Capable**: Full functionality must remain active and correct with airplane mode strictly ON.
* **Standalone**: Packaged as a single standalone executable using Tauri, requiring zero external services or system-level runtimes.
* **Privacy-by-Default**: No user data, files, workspace directories, or project telemetry may ever leave the local machine.
* **Solo-Developer Focused**: Streamlined, zero-configuration local workspace workflow to maximize solo development speed and practicality.

---

## 2. Strict Prohibitions (Never Implement)

* **No Cloud Services**: Do not integrate AWS, Azure, GCP, Firebase, Supabase, or other remote cloud offerings.
* **No Authentication Systems**: Avoid OAuth, JWT, login screens, sessions, or user account management.
* **No Containerization**: No Docker, Kubernetes, or containerization setups for running local features.
* **No AI/ML Network APIs**: No remote integrations with OpenAI, Claude, Gemini, or any LLM-powered cloud API (local Ollama is permitted only as a user-configured loopback option).
* **No Real-Time Sync**: No WebSockets, P2P networking, or external real-time data syncs.
* **No Telemetry**: No analytics, usage tracking, crash reporting services, or background network pings.

---

## 3. Technology Stack & Implementation Rules

* **Frontend**: HTML5, Vanilla CSS, TypeScript, and **Svelte 5** (utilizing runes like `$state`, `$derived`, and `$effect`).
* **Backend**: Rust 2021, Tauri v2.
* **Styling**: Tailored, modern dark/light CSS variables with high-end typography (Outfit, Inter) and glassmorphic surface elevations.
* **Editor**: CodeMirror 6 extensions only (strictly local-first; no remote LSP connections).
* **Database**: SQLite database operations via `rusqlite` and `r2d2` connection pooling. Avoid raw file manipulations for structured states; use transactional queries.
* **Reactivity**: Leverage Svelte 5 store auto-subscriptions (`$store`) in components to prevent subscription memory leaks.
* **Error Protocol**: Rust commands must return a clean `Result<T, String>` containing user-friendly errors. Frontend must handle and present these non-blockingly via global loading overlays and toasts without crashing.

---

## 4. Testing & Verification Requirements

* **Rust Tests**: `cargo test` for integration and command validation.
* **Svelte Tests**: `npm run test` (Vitest) for store reactivity, workspace updates, and layout configs.
* **Verification**: Code must compile without errors under `cargo check` and Svelte type-checking (`svelte-check`).
