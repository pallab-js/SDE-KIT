<script lang="ts">
  import { onMount } from 'svelte';
  import { getTasks } from '$lib/services/api';
  import type { Milestone } from '$lib/types';

  interface Props {
    ms: Milestone;
    onToggle: (id: string, current: string) => void;
    onRemove: (id: string) => void;
  }

  let { ms, onToggle, onRemove }: Props = $props();

  let progress = $state<{ done: number; total: number }>({ done: 0, total: 0 });

  onMount(() => {
    loadProgress();
  });

  // Watch for changes to ms.id or other properties to reload progress if needed
  $effect(() => {
    if (ms.id) {
      loadProgress();
    }
  });

  async function loadProgress() {
    try {
      const all = await getTasks();
      const relevant = all.filter((t) => t.milestoneId === ms.id);
      progress = {
        done: relevant.filter((t) => t.status === 'done').length,
        total: relevant.length,
      };
    } catch {
      progress = { done: 0, total: 0 };
    }
  }
</script>

<div class="milestone-card" class:closed={ms.status === 'closed'}>
  <div class="ms-header">
    <button
      class="ms-toggle typo-body"
      onclick={() => onToggle(ms.id, ms.status)}
      aria-label="Toggle milestone status"
    >
      {ms.status === 'open' ? '○' : '●'}
    </button>
    <span class="ms-title typo-caption">{ms.title}</span>
    <button class="delete-btn typo-body" onclick={() => onRemove(ms.id)}
      >×</button
    >
  </div>
  {#if ms.description}
    <div class="ms-desc typo-small">{ms.description}</div>
  {/if}
  <div class="ms-meta typo-small">
    {#if ms.dueDate}
      <span
        class="ms-due"
        class:overdue={ms.status === 'open' &&
          new Date(ms.dueDate) < new Date()}
      >
        Due: {ms.dueDate}
      </span>
    {/if}
    <span
      class="ms-status"
      class:open={ms.status === 'open'}
      class:closed={ms.status === 'closed'}
    >
      {ms.status}
    </span>
  </div>
  {#if progress.total > 0}
    <div class="ms-progress">
      <div class="ms-bar-track">
        <div
          class="ms-bar"
          style="width: {Math.round((progress.done / progress.total) * 100)}%"
        ></div>
      </div>
      <span class="ms-progress-label typo-small"
        >{progress.done}/{progress.total} tasks</span
      >
    </div>
  {/if}
</div>

<style>
  .milestone-card {
    margin: var(--spacing-1) var(--spacing-2);
    padding: var(--spacing-2);
    background: var(--color-surface-dark-soft);
    border: 1px solid var(--color-surface-dark-border);
    border-radius: var(--radius-md);
    border-left: 3px solid var(--color-primary);
  }
  .milestone-card.closed {
    opacity: 0.6;
    border-left-color: var(--color-muted);
  }
  .ms-header {
    display: flex;
    align-items: center;
    gap: var(--spacing-2);
  }
  .ms-toggle {
    border: none;
    background: none;
    cursor: pointer;
    color: var(--color-primary);
    padding: 0;
    flex-shrink: 0;
  }
  .milestone-card.closed .ms-toggle {
    color: var(--color-muted);
  }
  .ms-title {
    flex: 1;
    color: var(--color-on-dark);
    font-weight: 500;
  }
  .ms-desc {
    color: var(--color-on-dark-soft);
    margin: 4px 0 0 22px;
  }
  .ms-meta {
    display: flex;
    align-items: center;
    gap: var(--spacing-2);
    margin-top: var(--spacing-1);
    color: var(--color-muted);
    padding-left: 22px;
  }
  .ms-due.overdue {
    color: var(--color-error);
  }
  .ms-status {
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  .ms-status.open {
    color: var(--color-primary);
  }
  .ms-status.closed {
    color: var(--color-muted);
  }
  .delete-btn {
    width: 18px;
    height: 18px;
    border: none;
    background: none;
    color: var(--color-muted-soft);
    cursor: pointer;
    flex-shrink: 0;
  }
  .delete-btn:hover {
    color: var(--color-error);
  }
  .ms-progress {
    margin: 4px 0 0 22px;
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .ms-bar-track {
    height: 4px;
    flex: 1;
    background: var(--color-surface-dark-border);
    border-radius: 2px;
    overflow: hidden;
  }
  .ms-bar {
    height: 100%;
    background: var(--color-success);
    border-radius: 2px;
    transition: width 0.3s;
  }
  .ms-progress-label {
    color: var(--color-muted);
    font-size: 11px;
    white-space: nowrap;
  }
</style>
