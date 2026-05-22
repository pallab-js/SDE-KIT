<script lang="ts">
  import { onMount } from 'svelte';
  import CommandBar from './CommandBar.svelte';
  import ActivityBar from './ActivityBar.svelte';
  import Sidebar from './Sidebar.svelte';
  import MainContent from './MainContent.svelte';
  import BottomPanel from './BottomPanel.svelte';
  import CommandPalette from './CommandPalette.svelte';
  import KeyboardShortcuts from './KeyboardShortcuts.svelte';
  import {
    activePanel,
    sidebarWidth,
    bottomPanelOpen,
    togglePanel,
    toggleSidebar,
    toggleBottom,
    openTab,
  } from '$lib/stores/workspace';
  import { splitMode } from '$lib/stores/editor';
  import { registerCommand } from '$lib/stores/commands';
  import { restoreLayout, subscribeAndPersist } from '$lib/stores/layout';
  import { undoManager } from '$lib/services/undoManager';
  import { theme } from '$lib/stores/theme';
  import type { PanelId } from '$lib/types';

  let paletteOpen = $state(false);

  onMount(() => {
    bottomPanelOpen.set(false);
    restoreLayout();
    subscribeAndPersist();

    registerCommand({
      id: 'undo',
      label: 'Undo',
      shortcut: 'Cmd+Z',
      category: 'general',
      icon: '↩',
      action: () => {
        undoManager.undo();
      },
    });
    registerCommand({
      id: 'redo',
      label: 'Redo',
      shortcut: 'Cmd+Shift+Z',
      category: 'general',
      icon: '↪',
      action: () => {
        undoManager.redo();
      },
    });
    registerCommand({
      id: 'toggle-theme',
      label: 'Toggle Theme',
      shortcut: 'Cmd+Shift+T',
      category: 'view',
      icon: '☀',
      action: () => {
        theme.toggle();
      },
    });
    registerCommand({
      id: 'toggle-git',
      label: 'Show Git',
      category: 'view',
      icon: '⎇',
      action: () => {
        togglePanel('git' as PanelId);
      },
    });

    registerCommand({
      id: 'toggle-sidebar',
      label: 'Toggle Sidebar',
      shortcut: 'Cmd+B',
      category: 'view',
      icon: '⊞',
      action: () => {
        toggleSidebar();
      },
    });
    registerCommand({
      id: 'toggle-console',
      label: 'Toggle Console',
      shortcut: 'Cmd+J',
      category: 'view',
      icon: '_',
      action: () => {
        toggleBottom();
      },
    });
    registerCommand({
      id: 'explorer',
      label: 'Show Explorer',
      category: 'view',
      icon: '📁',
      action: () => {
        togglePanel('explorer' as PanelId);
      },
    });
    registerCommand({
      id: 'tasks',
      label: 'Show Tasks',
      category: 'view',
      icon: '✓',
      action: () => {
        togglePanel('tasks' as PanelId);
      },
    });
    registerCommand({
      id: 'milestones',
      label: 'Show Milestones',
      category: 'view',
      icon: '🏁',
      action: () => {
        togglePanel('milestones' as PanelId);
      },
    });
    registerCommand({
      id: 'projects',
      label: 'Show Projects',
      category: 'view',
      icon: '📦',
      action: () => {
        togglePanel('projects' as PanelId);
      },
    });
    registerCommand({
      id: 'search',
      label: 'Global Search',
      shortcut: 'Cmd+Shift+F',
      category: 'general',
      icon: '🔍',
      action: () => {
        togglePanel('search' as PanelId);
      },
    });
    registerCommand({
      id: 'new-tab',
      label: 'New Tab',
      shortcut: 'Cmd+T',
      category: 'general',
      icon: '+',
      action: () => {
        openTab({ id: `tab-${Date.now()}`, title: 'Untitled', closable: true });
      },
    });
    registerCommand({
      id: 'split-editor',
      label: 'Toggle Split Editor',
      category: 'editor',
      icon: '⧉',
      action: () => {
        splitMode.update((v) => !v);
      },
    });
  });
</script>

<KeyboardShortcuts onCommandPalette={() => (paletteOpen = true)} />
<CommandPalette open={paletteOpen} onClose={() => (paletteOpen = false)} />

<div class="workspace">
  <CommandBar onPalette={() => (paletteOpen = true)} />
  <div class="workspace-body">
    <ActivityBar />
    {#if $activePanel}
      <Sidebar />
    {/if}
    <MainContent />
  </div>
  <BottomPanel />
</div>

<style>
  .workspace {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
  }

  .workspace-body {
    display: flex;
    flex: 1;
    overflow: hidden;
  }
</style>
