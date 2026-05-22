# Specification: SDE-KIT Workspace Sidepanel Sizing & Reactivity Redesign

## 1. Goal
Resolve layout resizing anomalies, visual double-render shifts, and potential memory leaks in the workspace sidepanel to ensure a clean, minimalist, high-end visual workspace.

---

## 2. Background & Problem Statement
* **Double-Render Layout Shift**: On launch, the sidebar width was explicitly set to `window.innerWidth / 2` (half of the screen) during `onMount` in `Workspace.svelte`. This overrode the restored user state, causing a visual flash where the sidebar opened extremely wide and then snapped back to its standard width when the layout was asynchronously loaded.
* **Store Subscription Memory Leaks**: `Sidebar.svelte` and `MainContent.svelte` used manual `.subscribe` statements in `onMount` and `$effect` blocks without retaining the returned unsubscribe handles. Toggling panels and tabs created duplicate subscriptions, causing memory leaks and UI lag over time.
* **Layout Sizing Anomalies**: Resizing bounds allowed the sidebar to reach up to `800px` wide, which could cover the code editor completely on medium screens, ruining visual hierarchy.

---

## 3. Requirements

* **Flicker-Free Load**: Initialize the workspace sidepanel at a standard default width of `240px` and apply the restored layout configuration from Tauri / LocalStorage asynchronously without intermediate wide-state flashing.
* **Auto-Subscription Migration**: Leverage Svelte 5's native auto-subscription (`$store`) framework. Eliminate all manual store subscription subscriptions in components to prevent memory leaks during component unmounting/remounting.
* **Sensible Resize Bounds**: Restrict the sidebar resizing width strictly between `180px` (minimum) and `500px` (maximum) to preserve clear focus on the code editor.
* **Micro-interactions**: Incorporate smooth CSS transitions on hover over the `.resize-handle` to provide a premium design feeling.
