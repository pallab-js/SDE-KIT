<script lang="ts">
  import { workspaceRoot } from '$lib/stores/workspace';
  import FileTree from './FileTree.svelte';

  async function openFolder() {
    try {
      const { open } = await import('@tauri-apps/plugin-dialog');
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Open Workspace Folder',
      });
      if (selected) {
        workspaceRoot.set(selected);
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('set_workspace_root', { path: selected });
      }
    } catch {
      // fallback for non-Tauri environments
    }
  }
</script>

<div class="explorer">
  {#if $workspaceRoot}
    <div class="explorer-header typo-overline">
      <span class="path-label">{$workspaceRoot.split('/').pop()}</span>
    </div>
    <div class="tree-container">
      <FileTree root={$workspaceRoot} />
    </div>
  {:else}
    <div class="empty-state">
      <div class="empty-icon">📂</div>
      <div class="empty-text typo-body">No folder open</div>
      <button class="open-btn typo-caption" onclick={openFolder}
        >Open Folder</button
      >
    </div>
  {/if}
</div>

<style>
  .explorer {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .explorer-header {
    padding: 6px var(--spacing-3);
    color: var(--color-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    border-bottom: 1px solid var(--color-surface-dark-border);
  }

  .path-label {
    font-weight: 500;
  }

  .tree-container {
    flex: 1;
    overflow-y: auto;
    padding: var(--spacing-1) 0;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-3);
    padding: var(--spacing-10) var(--spacing-3);
    text-align: center;
  }

  .empty-icon {
    font-size: 24px;
    opacity: 0.2;
  }

  .empty-text {
    color: var(--color-muted);
  }

  .open-btn {
    padding: 4px 16px;
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark-soft);
    color: var(--color-on-dark);
    border-radius: var(--radius-xs);
    cursor: pointer;
  }

  .open-btn:hover {
    background: var(--color-surface-dark-elevated);
    border-color: var(--color-muted);
  }
</style>
