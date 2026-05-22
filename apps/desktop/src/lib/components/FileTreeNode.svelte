<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { listDirectory } from '$lib/services/fs';
  import { openTab } from '$lib/stores/workspace';
  import { invoke } from '@tauri-apps/api/core';
  import type { FileEntry } from '$lib/types';
  import FileTreeNode from './FileTreeNode.svelte';

  interface Props {
    entry: FileEntry;
    pageSize?: number;
  }

  let { entry, pageSize = 50 }: Props = $props();

  let children = $state<FileEntry[]>([]);
  let loading = $state(true);
  let expanded = $state(false);
  let showCount = $state(50);
  let unlisten: (() => void) | undefined;
  let contextMenu = $state<{
    x: number;
    y: number;
    open: boolean;
  }>({ x: 0, y: 0, open: false });

  function openContextMenu(e: MouseEvent) {
    e.preventDefault();
    contextMenu = { x: e.clientX, y: e.clientY, open: true };
  }

  async function newFile() {
    const name = prompt('New file name:');
    if (!name) return;
    const path = entry.is_dir
      ? entry.path + '/' + name
      : entry.path.split('/').slice(0, -1).join('/') + '/' + name;
    await invoke('write_file', { path, content: '' });
    contextMenu.open = false;
    if (entry.is_dir) {
      if (!expanded) {
        expanded = true;
      }
      load();
    }
  }

  async function renameEntry() {
    const newName = prompt('New name:', entry.name);
    if (!newName || newName === entry.name) return;
    const dir = entry.path.split('/').slice(0, -1).join('/');
    await invoke('rename_file', {
      oldPath: entry.path,
      newPath: dir + '/' + newName,
    });
    contextMenu.open = false;
  }

  async function deleteEntry() {
    if (!confirm(`Delete "${entry.name}"?`)) return;
    await invoke('delete_file', { path: entry.path });
    contextMenu.open = false;
  }

  onMount(async () => {
    if (entry.is_dir) {
      unlisten = await listen('fs-event', () => {
        if (expanded) load();
      });
    }
  });

  onDestroy(() => {
    unlisten?.();
  });

  async function load() {
    loading = true;
    try {
      children = await listDirectory(entry.path);
    } catch {
      children = [];
    }
    loading = false;
  }

  function toggle() {
    expanded = !expanded;
    if (expanded && children.length === 0) load();
  }

  function openFile() {
    openTab({
      id: `file:${entry.path}`,
      title: entry.name,
      icon: '📄',
      closable: true,
      filePath: entry.path,
    });
  }
</script>

{#if entry.is_dir}
  <div class="folder-row" class:expanded>
    <button
      class="folder-toggle typo-caption"
      onclick={toggle}
      oncontextmenu={openContextMenu}
      aria-label={expanded ? 'Collapse' : 'Expand'}
    >
      <span class="chevron">{expanded ? '▼' : '▶'}</span>
      <span class="folder-icon">{expanded ? '📂' : '📁'}</span>
      <span class="folder-name">{entry.name}</span>
    </button>
  </div>

  {#if expanded}
    <div class="folder-children">
      {#if loading}
        <div class="loading typo-small">Loading...</div>
      {:else}
        {#each children.filter((c) => c.is_dir) as dir (dir.path)}
          <div class="child">
            <FileTreeNode entry={dir} {pageSize} />
          </div>
        {/each}
        {@const files = children.filter((c) => !c.is_dir)}
        {#each files.slice(0, showCount) as file (file.path)}
          <FileTreeNode entry={file} {pageSize} />
        {/each}
        {#if files.length > showCount}
          <button
            class="show-more typo-small"
            onclick={() => (showCount += pageSize)}
          >
            Show {files.length - showCount} more…
          </button>
        {/if}
      {/if}
    </div>
  {/if}
{:else}
  <button
    class="file-row typo-caption"
    onclick={openFile}
    oncontextmenu={openContextMenu}
  >
    <span class="file-icon">📄</span>
    <span class="file-name">{entry.name}</span>
  </button>
{/if}

{#if contextMenu.open}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="ctx-backdrop"
    onclick={() => (contextMenu.open = false)}
    onkeydown={() => {}}
  ></div>
  <div
    class="ctx-menu"
    style="left: {contextMenu.x}px; top: {contextMenu.y}px;"
  >
    <button onclick={newFile}>New File Here</button>
    <button onclick={renameEntry}>Rename</button>
    <button class="danger" onclick={deleteEntry}>Delete</button>
  </div>
{/if}

<style>
  .folder-row {
    user-select: none;
  }

  .folder-toggle {
    display: flex;
    align-items: center;
    gap: 4px;
    width: 100%;
    padding: 3px var(--spacing-3);
    border: none;
    background: none;
    color: var(--color-on-dark-soft);
    cursor: pointer;
    text-align: left;
  }

  .folder-toggle:hover {
    background: var(--color-surface-dark);
    color: var(--color-on-dark);
  }

  .chevron {
    font-size: 8px;
    width: 12px;
    flex-shrink: 0;
    opacity: 0.6;
  }

  .folder-icon {
    width: 16px;
    text-align: center;
    flex-shrink: 0;
  }

  .folder-name {
    font-weight: 500;
    color: var(--color-on-dark-soft);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .folder-children {
    padding-left: var(--spacing-3);
  }

  .child {
    margin: 0;
  }

  .file-row {
    display: flex;
    align-items: center;
    gap: 4px;
    width: 100%;
    padding: 3px var(--spacing-3);
    padding-left: 28px;
    border: none;
    background: none;
    color: var(--color-on-dark-soft);
    cursor: pointer;
    text-align: left;
  }

  .file-row:hover {
    background: var(--color-surface-dark);
    color: var(--color-on-dark);
  }

  .file-icon {
    width: 16px;
    text-align: center;
    flex-shrink: 0;
  }

  .file-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .loading {
    padding: 4px var(--spacing-3);
    color: var(--color-muted-soft);
    font-style: italic;
  }

  .show-more {
    display: block;
    width: 100%;
    padding: 3px var(--spacing-3);
    padding-left: 28px;
    border: none;
    background: none;
    color: var(--color-muted);
    cursor: pointer;
    font-style: italic;
    text-align: left;
  }

  .show-more:hover {
    color: var(--color-on-dark);
    background: var(--color-surface-dark);
  }

  .ctx-backdrop {
    position: fixed;
    inset: 0;
    z-index: 99;
  }
  .ctx-menu {
    position: fixed;
    z-index: 100;
    background: var(--color-surface-dark-elevated);
    border: 1px solid var(--color-surface-dark-border);
    border-radius: var(--radius-md);
    padding: 4px;
    min-width: 160px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
  }
  .ctx-menu button {
    display: block;
    width: 100%;
    padding: 6px 12px;
    border: none;
    background: none;
    color: var(--color-on-dark);
    text-align: left;
    cursor: pointer;
    border-radius: var(--radius-xs);
    font-size: 13px;
  }
  .ctx-menu button:hover {
    background: var(--color-surface-dark);
  }
  .ctx-menu button.danger:hover {
    color: var(--color-error);
  }
</style>
