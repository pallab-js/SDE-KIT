# Actionable Tasks: SDE-KIT Workspace Sidepanel Sizing & Reactivity Redesign

- `[x]` **Phase 1: Workspace Double-Render Flash Fix**
  - `[x]` Locate `apps/desktop/src/lib/components/Workspace.svelte`
  - `[x]` Remove `sidebarWidth.set(window.innerWidth / 2)` from the `onMount` routine
  - `[x]` Assert that saved layout states load asynchronously without flashes

- `[x]` **Phase 2: Migrate Sidebar to Auto-Subscriptions**
  - `[x]` Locate `apps/desktop/src/lib/components/Sidebar.svelte`
  - `[x]` Strip out the manual `onMount` subscription callback and unused imports
  - `[x]` Replace the local `width` state variable with `$sidebarWidth` auto-subscription in script and markup
  - `[x]` Update drag-resizing boundaries from `800px` to a standard cap of `500px` in `startResize` and `autoFit`

- `[x]` **Phase 3: Migrate Tab Active ID to Auto-Subscriptions**
  - `[x]` Locate `apps/desktop/src/lib/components/MainContent.svelte`
  - `[x]` Strip out the manual `.subscribe` hook from the `$effect` block
  - `[x]` Re-map all `activeId` variables in the HTML template to `$activeTabId` auto-subscription

- `[x]` **Phase 4: Aesthetic Resizing Handle Polish**
  - `[x]` Add smooth CSS transition transitions for `background` and `opacity` properties on `.resize-handle` style
  - `[x]` Set hover opacity to `0.5` on `.resize-handle:hover` for superior micro-interaction visual feedback

- `[x]` **Phase 5: Technical Verification**
  - `[x]` Run Rust compiler checking (`cargo check`)
  - `[x]` Run Vitest suite tests (`bun run test`)
  - `[x]` Run Svelte and TypeScript checker (`bun run check`)
  - `[x]` Verify that all code changes are staged, formatted, and committed successfully under git
