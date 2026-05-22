<script lang="ts">
  import { activePanel, togglePanel } from '$lib/stores/workspace';
  import type { PanelId } from '$lib/types';

  const ICONS: Record<string, string> = {
    explorer:
      'M2 6a2 2 0 012-2h5l2 2h5a2 2 0 012 2v8a2 2 0 01-2 2H4a2 2 0 01-2-2V6z',
    search: 'M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z',
    projects:
      'M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4A2 2 0 002 8v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4A2 2 0 0021 16z',
    tasks: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z',
    milestones: 'M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z',
    graphs: 'M13 3h7v7M10 21H3v-7M21 3l-7 7M3 21l7-7',
    notes: 'M14.5 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V7.5L14.5 2z',
    inspector: 'M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z',
    dashboard: 'M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z',
    git: 'M2.6 10.6a2 2 0 010-3.2l5-3.6a2 2 0 012.8.4l5.2 7a2 2 0 01-.4 2.8l-5 3.6a2 2 0 01-2.8-.4l-5.2-7z',
    activity:
      'M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2zm0 18c-4.4 0-8-3.6-8-8s3.6-8 8-8 8 3.6 8 8-3.6 8-8 8zm.5-13H11v6l5.2 3.2.8-1.3-4.5-2.7V7z',
  };

  const items: { id: PanelId; label: string }[] = [
    { id: 'explorer', label: 'Explorer' },
    { id: 'search', label: 'Search' },
    { id: 'projects', label: 'Projects' },
    { id: 'tasks', label: 'Tasks' },
    { id: 'milestones', label: 'Milestones' },
    { id: 'graphs', label: 'Graphs' },
    { id: 'notes', label: 'Notes' },
    { id: 'activity', label: 'Pomodoro' },
    { id: 'dashboard', label: 'Dashboard' },
    { id: 'git', label: 'Git' },
  ];
</script>

<aside class="activity-bar">
  {#each items as item}
    <button
      class="activity-bar-item"
      class:active={$activePanel === item.id}
      onclick={() => togglePanel(item.id)}
      title={item.label}
    >
      <svg
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"><path d={ICONS[item.id]} /></svg
      >
    </button>
  {/each}
</aside>

<style>
  .activity-bar {
    display: flex;
    flex-direction: column;
    width: 48px;
    flex-shrink: 0;
    background: var(--color-surface-dark);
    border-right: 1px solid var(--color-surface-dark-border);
    padding: var(--spacing-1) 0;
    align-items: center;
    gap: 2px;
  }

  .activity-bar-item {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    padding: 0;
    border: none;
    background: none;
    color: var(--color-on-dark-soft);
    cursor: pointer;
    position: relative;
  }

  .activity-bar-item svg {
    width: 24px;
    height: 24px;
  }

  .activity-bar-item:hover {
    color: var(--color-on-dark);
  }

  .activity-bar-item.active {
    color: var(--color-on-dark);
  }

  .activity-bar-item.active::before {
    content: '';
    position: absolute;
    left: 0;
    top: 6px;
    bottom: 6px;
    width: 2px;
    background: var(--color-primary);
    border-radius: 0 1px 1px 0;
  }
</style>
