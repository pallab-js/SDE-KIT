<script lang="ts">
  import { onMount } from 'svelte';
  import { getProjects, createProject, deleteProject } from '$lib/services/api';
  import { workspaceRoot } from '$lib/stores/workspace';
  import { invoke } from '@tauri-apps/api/core';
  import type { Project } from '$lib/types';
  import { LocalDatabase } from '$lib/services/database/local-db';
  import { withLoading, globalError } from '$lib/stores/app';

  async function exportProject(project: Project, format: 'json' | 'sqlite') {
    globalError.set(null);
    try {
      await withLoading(async () => {
        const dbService = LocalDatabase.getInstance();
        const res = await dbService.exportProject(project.id, format);
        if (res.success && res.data) {
          const url = URL.createObjectURL(res.data);
          const a = document.createElement('a');
          a.href = url;
          a.download =
            format === 'json' ? `${project.name}.json` : `${project.name}.db`;
          a.click();
          URL.revokeObjectURL(url);
        } else {
          throw new Error(res.error || 'Failed to export project');
        }
      });
    } catch (err: any) {
      globalError.set({
        code: 'EXPORT_FAILED',
        message: err.message || String(err),
      });
    }
  }

  let projects = $state<Project[]>([]);
  let loading = $state(true);
  let showNew = $state(false);
  let newName = $state('');
  let newPath = $state('');

  onMount(() => load());

  async function load() {
    loading = true;
    try {
      projects = await getProjects();
    } catch {
      projects = [];
    }
    loading = false;
  }

  async function create() {
    if (!newName.trim()) return;
    try {
      await createProject(newName.trim(), newPath || '/');
      newName = '';
      newPath = '';
      showNew = false;
      load();
    } catch {}
  }

  async function remove(id: string) {
    try {
      await deleteProject(id);
      load();
    } catch {}
  }

  async function openProject(project: Project) {
    workspaceRoot.set(project.path);
    try {
      await invoke('set_workspace_root', { path: project.path });
    } catch {}
    const { togglePanel } = await import('$lib/stores/workspace');
    togglePanel('explorer');
  }
</script>

<div class="projects-panel">
  <div class="panel-header">
    <button class="add-btn typo-body" onclick={() => (showNew = !showNew)}
      >+</button
    >
  </div>

  {#if showNew}
    <div class="new-form">
      <input
        type="text"
        class="typo-caption"
        placeholder="Project name"
        bind:value={newName}
      />
      <input
        type="text"
        class="typo-caption"
        placeholder="Path (optional)"
        bind:value={newPath}
      />
      <div class="form-actions">
        <button class="btn-primary typo-caption" onclick={create}>Create</button
        >
        <button
          class="btn-secondary typo-caption"
          onclick={() => (showNew = false)}>Cancel</button
        >
      </div>
    </div>
  {/if}

  <div class="project-list">
    {#if loading}
      <div class="empty">Loading...</div>
    {:else if projects.length === 0}
      <div class="empty">
        <span class="empty-icon">📦</span>
        <span class="empty-text typo-body">No projects yet</span>
      </div>
    {:else}
      {#each projects as project (project.id)}
        <div class="project-card">
          <div class="project-info">
            <span class="project-name typo-caption">{project.name}</span>
            <span class="project-path">{project.path}</span>
          </div>
          <button
            class="open-project-btn typo-small"
            onclick={() => openProject(project)}
            title="Open in Explorer">→</button
          >
          <button
            class="export-btn typo-small"
            onclick={() => exportProject(project, 'json')}
            title="Export Project as JSON">JSON</button
          >
          <button
            class="export-btn typo-small"
            onclick={() => exportProject(project, 'sqlite')}
            title="Export Project as SQLite">SQL</button
          >
          <button
            class="delete-btn typo-body"
            onclick={() => remove(project.id)}>×</button
          >
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .projects-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px var(--spacing-3);
    border-bottom: 1px solid var(--color-surface-dark-border);
  }
  .add-btn {
    width: 20px;
    height: 20px;
    border: none;
    background: var(--color-surface-dark);
    color: var(--color-on-dark-soft);
    cursor: pointer;
    border-radius: var(--radius-xs);
    line-height: 1;
  }
  .add-btn:hover {
    background: var(--color-surface-dark-elevated);
    color: var(--color-on-dark);
  }

  .new-form {
    padding: var(--spacing-2);
    display: flex;
    flex-direction: column;
    gap: var(--spacing-1);
    border-bottom: 1px solid var(--color-surface-dark-border);
  }
  .new-form input {
    padding: 4px var(--spacing-2);
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark);
    color: var(--color-on-dark);
    border-radius: var(--radius-xs);
    outline: none;
  }
  .new-form input:focus {
    border-color: var(--color-primary);
  }
  .form-actions {
    display: flex;
    gap: var(--spacing-1);
  }
  .btn-primary,
  .btn-secondary {
    padding: 3px 10px;
    border: 1px solid var(--color-surface-dark-border);
    border-radius: var(--radius-xs);
    cursor: pointer;
  }
  .btn-primary {
    background: var(--color-primary);
    color: var(--color-on-primary);
    border-color: var(--color-primary);
  }
  .btn-secondary {
    background: var(--color-surface-dark-elevated);
    color: var(--color-on-dark-soft);
  }
  .btn-secondary:hover {
    background: var(--color-surface-dark-soft);
  }

  .project-list {
    flex: 1;
    overflow-y: auto;
    padding: var(--spacing-1) 0;
  }
  .project-card {
    display: flex;
    align-items: center;
    gap: var(--spacing-2);
    padding: 6px var(--spacing-3);
    border-bottom: 1px solid var(--color-surface-dark-border);
  }
  .project-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .project-name {
    color: var(--color-on-dark);
    font-weight: 500;
  }
  .project-path {
    font-size: 11px;
    color: var(--color-muted-soft);
  }
  .delete-btn {
    width: 18px;
    height: 18px;
    border: none;
    background: none;
    color: var(--color-muted-soft);
    cursor: pointer;
  }
  .delete-btn:hover {
    color: var(--color-error);
  }
  .export-btn {
    padding: 2px 6px;
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark);
    color: var(--color-on-dark-soft);
    cursor: pointer;
    border-radius: var(--radius-xs);
    font-size: 10px;
    transition: all 0.2s ease-in-out;
  }
  .export-btn:hover {
    background: var(--color-surface-dark-elevated);
    color: var(--color-primary);
    border-color: var(--color-primary);
  }
  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--spacing-2);
    padding: var(--spacing-8);
    text-align: center;
  }
  .empty-icon {
    font-size: 24px;
    opacity: 0.3;
  }
  .empty-text {
    color: var(--color-muted);
  }
</style>
