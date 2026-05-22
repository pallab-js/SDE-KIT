<script lang="ts">
  import { activePanel, sidebarWidth } from '$lib/stores/workspace';
  import type { PanelId } from '$lib/types';
  import { onMount } from 'svelte';
  import FileExplorer from './FileExplorer.svelte';
  import TasksPanel from './TasksPanel.svelte';
  import ProjectsPanel from './ProjectsPanel.svelte';
  import MilestonesPanel from './MilestonesPanel.svelte';
  import GraphPanel from './GraphPanel.svelte';
  import SearchPanel from './SearchPanel.svelte';
  import NotesPanel from './NotesPanel.svelte';
  import Dashboard from './Dashboard.svelte';
  import PomodoroPanel from './PomodoroPanel.svelte';
  import GitPanel from './GitPanel.svelte';

  const PANEL_LABELS: Record<PanelId, string> = {
    explorer: 'Explorer',
    projects: 'Projects',
    tasks: 'Tasks',
    milestones: 'Milestones',
    graphs: 'Graphs',
    search: 'Search',
    notes: 'Notes',
    inspector: 'Inspector',
    activity: 'Activity',
    layouts: 'Layouts',
    dashboard: 'Dashboard',
    git: 'Git',
  };

  const ICONS: Record<string, string> = {
    explorer:
      'M2 6a2 2 0 012-2h5l2 2h5a2 2 0 012 2v8a2 2 0 01-2 2H4a2 2 0 01-2-2V6z',
    projects:
      'M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4A2 2 0 002 8v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4A2 2 0 0021 16z',
    tasks: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z',
    milestones: 'M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z',
    graphs: 'M13 3h7v7M10 21H3v-7M21 3l-7 7M3 21l7-7',
    search: 'M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z',
    notes: 'M14.5 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V7.5L14.5 2z',
    inspector: 'M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z',
    activity: 'M13 10V3L4 14h7v7l9-11h-7z',
    layouts:
      'M4 5a1 1 0 011-1h14a1 1 0 011 1v2a1 1 0 01-1 1H5a1 1 0 01-1-1V5zm0 8a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H5a1 1 0 01-1-1v-6zm12 0a1 1 0 011-1h2a1 1 0 011 1v6a1 1 0 01-1 1h-2a1 1 0 01-1-1v-6z',
    dashboard: 'M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z',
    git: 'M2.6 10.6a2 2 0 010-3.2l5-3.6a2 2 0 012.8.4l5.2 7a2 2 0 01-.4 2.8l-5 3.6a2 2 0 01-2.8-.4l-5.2-7z',
  };

  let width = $state(240);
  let panelEl = $state<HTMLDivElement>();

  onMount(() => {
    sidebarWidth.subscribe((v) => (width = v));
  });

  function startResize(e: MouseEvent) {
    const startX = e.clientX;
    const startW = width;
    function onMouseMove(ev: MouseEvent) {
      const newWidth = Math.max(
        180,
        Math.min(800, startW + (ev.clientX - startX))
      );
      width = newWidth;
      sidebarWidth.set(newWidth);
    }
    function onMouseUp() {
      document.removeEventListener('mousemove', onMouseMove);
      document.removeEventListener('mouseup', onMouseUp);
    }
    document.addEventListener('mousemove', onMouseMove);
    document.addEventListener('mouseup', onMouseUp);
  }

  function autoFit() {
    if (!panelEl) return;
    const contentWidth = panelEl.scrollWidth;
    const newWidth = Math.max(180, Math.min(800, contentWidth + 16));
    width = newWidth;
    sidebarWidth.set(newWidth);
  }
</script>

<aside class="sidebar" style="width: {width}px">
  <div class="panel-header">
    <span class="panel-title typo-overline"
      >{PANEL_LABELS[$activePanel ?? 'explorer']}</span
    >
  </div>
  <div class="panel-content" bind:this={panelEl}>
    {#if $activePanel === 'explorer'}
      <FileExplorer />
    {:else if $activePanel === 'tasks'}
      <TasksPanel />
    {:else if $activePanel === 'projects'}
      <ProjectsPanel />
    {:else if $activePanel === 'milestones'}
      <MilestonesPanel />
    {:else if $activePanel === 'graphs'}
      <GraphPanel />
    {:else if $activePanel === 'search'}
      <SearchPanel />
    {:else if $activePanel === 'notes'}
      <NotesPanel />
    {:else if $activePanel === 'dashboard'}
      <Dashboard />
    {:else if $activePanel === 'activity'}
      <PomodoroPanel />
    {:else if $activePanel === 'git'}
      <GitPanel />
    {:else}
      <div class="placeholder-panel">
        <svg
          class="placeholder-icon"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          ><path d={ICONS[$activePanel ?? 'explorer']} /></svg
        >
        <span class="placeholder-text typo-body"
          >{PANEL_LABELS[$activePanel ?? 'explorer']}</span
        >
      </div>
    {/if}
  </div>
  <div
    class="resize-handle"
    aria-hidden="true"
    onmousedown={startResize}
    ondblclick={autoFit}
  ></div>
</aside>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    background: var(--color-surface-dark-elevated);
    border-right: 1px solid var(--color-surface-dark-border);
    position: relative;
    min-width: 180px;
  }

  .panel-header {
    display: flex;
    align-items: center;
    padding: 0 var(--spacing-3);
    height: 36px;
    border-bottom: 1px solid var(--color-surface-dark-border);
    flex-shrink: 0;
  }

  .panel-title {
    font-weight: 500;
    color: var(--color-on-dark-soft);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .panel-content {
    flex: 1;
    overflow-y: auto;
  }

  .placeholder-panel {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-2);
    padding: var(--spacing-8) var(--spacing-3);
    text-align: center;
  }
  .placeholder-icon {
    width: 32px;
    height: 32px;
    opacity: 0.3;
  }
  .placeholder-text {
    color: var(--color-muted);
  }

  .resize-handle {
    position: absolute;
    top: 0;
    right: -2px;
    width: 4px;
    height: 100%;
    cursor: col-resize;
    z-index: 10;
  }
  .resize-handle:hover {
    background: var(--color-primary);
    opacity: 0.3;
  }
</style>
