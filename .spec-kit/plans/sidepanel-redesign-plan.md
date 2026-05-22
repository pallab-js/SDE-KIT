# Technical Implementation Plan: SDE-KIT Workspace Sidepanel Sizing & Reactivity Redesign

## 1. Proposed Code Changes

### [Workspace.svelte](file:///Users/pallabpc/desktop/SDE-KIT/apps/desktop/src/lib/components/Workspace.svelte)
* **Action**: Remove the `sidebarWidth.set(window.innerWidth / 2);` statement from the `onMount` callback.
* **Rationale**: Prevents intermediate wide-layout assignments before `restoreLayout()` completes. The sidebar width will naturally start at its default store value of `240px` and then transition smoothly to the saved state.

### [Sidebar.svelte](file:///Users/pallabpc/desktop/SDE-KIT/apps/desktop/src/lib/components/Sidebar.svelte)
* **Action**:
  1. Remove local state `let width = $state(240);` and manual `onMount` subscription hook.
  2. Map the `<aside>` element's width directly to Svelte's auto-subscription `$sidebarWidth`.
  3. Update `startResize` and `autoFit` functions to write directly to `sidebarWidth.set(newWidth)`.
  4. Restrict resizing bounds in `Math.min(500, ...)` instead of `800`.
  5. Add CSS transitions (`transition: background 0.2s ease, opacity 0.2s ease`) and set hover opacity to `0.5` on the `.resize-handle` style block.
* **Rationale**: Simplifies the reactive flow, eliminates manual subscription leaks, enforces visual bounds, and provides a polished interactive feel.

### [MainContent.svelte](file:///Users/pallabpc/desktop/SDE-KIT/apps/desktop/src/lib/components/MainContent.svelte)
* **Action**: Remove local state `activeId` and the manual `.subscribe` block inside `$effect`. Replace `activeId` with `$activeTabId` directly in the markup.
* **Rationale**: Eradicates redundant subscription leakage when tab states shift.

---

## 2. Verification Plan

### Automated Tests
* Run Vitest checks to confirm workspace state management continues to pass:
  ```bash
  bun run test
  ```
* Run `svelte-check` to assert that TypeScript type-checking passes successfully without any new errors in our components:
  ```bash
  bun run check
  ```
* Run Cargo check to ensure Tauri build checks remain successful:
  ```bash
  cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml
  ```

### Manual Verification
* Validate that workspace boots up seamlessly with zero wide-panel visual flashing.
* Double-click the resize handle to auto-fit and drag the handle, ensuring bounds strictly constrain between 180px and 500px.
* Verify that toggling side panels and tabs does not degrade UI rendering performance.
