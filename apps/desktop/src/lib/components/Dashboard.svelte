<script lang="ts">
  import { onMount } from 'svelte';
  import { workspaceRoot } from '$lib/stores/workspace';
  import { getTasks } from '$lib/services/api';
  import { invoke } from '@tauri-apps/api/core';
  import type { Task } from '$lib/types';

  let tasks = $state<Task[]>([]);
  let todoCount = $state(0);
  let doingCount = $state(0);
  let doneCount = $state(0);
  let fileCount = $state(0);
  let dirCount = $state(0);
  let todoInFiles = $state(0);
  let fixmeInFiles = $state(0);
  let loading = $state(true);

  onMount(async () => {
    loading = true;
    try {
      tasks = await getTasks();
      todoCount = tasks.filter((t) => t.status === 'todo').length;
      doingCount = tasks.filter((t) => t.status === 'doing').length;
      doneCount = tasks.filter((t) => t.status === 'done').length;

      if ($workspaceRoot) {
        const entries = await invoke<any[]>('list_directory', {
          path: $workspaceRoot,
        });
        dirCount = entries.filter((e) => e.is_dir).length;
        fileCount = entries.filter((e) => !e.is_dir).length;

        const search = await invoke<any[]>('search_in_files', {
          query: 'TODO',
          caseSensitive: true,
        });
        todoInFiles = search.length;

        const fixme = await invoke<any[]>('search_in_files', {
          query: 'FIXME',
          caseSensitive: true,
        });
        fixmeInFiles = fixme.length;
      }
    } catch {}
    loading = false;
  });
</script>

<div class="dashboard">
  {#if loading}
    <div class="loading typo-body">Loading...</div>
  {:else}
    <div class="grid">
      <div class="card">
        <span class="card-value">{tasks.length}</span>
        <span class="card-label typo-caption">Total Tasks</span>
      </div>
      <div class="card accent-todo">
        <span class="card-value">{todoCount}</span>
        <span class="card-label typo-caption">Todo</span>
      </div>
      <div class="card accent-doing">
        <span class="card-value">{doingCount}</span>
        <span class="card-label typo-caption">In Progress</span>
      </div>
      <div class="card accent-done">
        <span class="card-value">{doneCount}</span>
        <span class="card-label typo-caption">Done</span>
      </div>
    </div>

    {#if $workspaceRoot}
      <div class="section">
        <h3 class="typo-overline">Workspace</h3>
        <div class="grid compact">
          <div class="card small">
            <span class="card-value">{fileCount}</span>
            <span class="card-label typo-small">Files</span>
          </div>
          <div class="card small">
            <span class="card-value">{dirCount}</span>
            <span class="card-label typo-small">Directories</span>
          </div>
          <div class="card small warn">
            <span class="card-value">{todoInFiles}</span>
            <span class="card-label typo-small">TODOs in Code</span>
          </div>
          <div class="card small warn">
            <span class="card-value">{fixmeInFiles}</span>
            <span class="card-label typo-small">FIXMEs in Code</span>
          </div>
        </div>
      </div>
    {:else}
      <div class="no-workspace">
        <p class="typo-body">Open a folder to see workspace stats</p>
      </div>
    {/if}
  {/if}
</div>

<style>
  .dashboard {
    padding: var(--spacing-4);
    overflow-y: auto;
    height: 100%;
  }

  .loading {
    color: var(--color-muted);
    text-align: center;
    padding: var(--spacing-8);
  }

  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--spacing-2);
    margin: var(--spacing-3) 0;
  }

  .grid.compact {
    gap: var(--spacing-1);
  }

  .card {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: var(--spacing-3);
    background: var(--color-surface-dark-soft);
    border: 1px solid var(--color-surface-dark-border);
    border-radius: var(--radius-md);
    border-left: 3px solid var(--color-primary);
  }

  .card.accent-todo {
    border-left-color: var(--color-muted-soft);
  }
  .card.accent-doing {
    border-left-color: var(--color-warning);
  }
  .card.accent-done {
    border-left-color: var(--color-success);
  }
  .card.small {
    padding: var(--spacing-2);
  }
  .card.warn {
    border-left-color: var(--color-warning);
  }

  .card-value {
    font-family: var(--font-display);
    font-size: 24px;
    color: var(--color-on-dark);
  }

  .card.small .card-value {
    font-size: 18px;
  }

  .card-label {
    color: var(--color-muted);
  }

  .section {
    margin-top: var(--spacing-4);
  }

  .section h3 {
    margin-bottom: var(--spacing-2);
  }

  .no-workspace {
    text-align: center;
    color: var(--color-muted);
    padding: var(--spacing-6);
  }
</style>
