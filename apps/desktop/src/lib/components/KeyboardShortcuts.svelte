<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import {
    openTab,
    closeTab,
    toggleSidebar,
    toggleBottom,
    activePanel,
    activeTabId,
    openTabs,
  } from '$lib/stores/workspace';
  import { splitMode } from '$lib/stores/editor';
  import { get } from 'svelte/store';
  import type { Tab, PanelId } from '$lib/types';

  let { onCommandPalette }: { onCommandPalette: () => void } = $props();

  let tabs: Tab[] = [];
  let panel: PanelId | null = null;
  let currentTab: string | null = null;

  onMount(() => {
    const unsub1 = openTabs.subscribe((v) => (tabs = v));
    const unsub2 = activePanel.subscribe((v) => (panel = v));
    const unsub3 = activeTabId.subscribe((v) => (currentTab = v));
    return () => {
      unsub1();
      unsub2();
      unsub3();
    };
  });

  function handleKey(e: KeyboardEvent) {
    const meta = e.metaKey || e.ctrlKey;

    if (meta && e.key === 'p') {
      e.preventDefault();
      onCommandPalette();
      return;
    }

    if (meta && e.key === 'b') {
      e.preventDefault();
      toggleSidebar();
      return;
    }

    if (meta && e.key === 'j') {
      e.preventDefault();
      toggleBottom();
      return;
    }

    if (meta && e.key === 't') {
      e.preventDefault();
      openTab({ id: `tab-${Date.now()}`, title: 'Untitled', closable: true });
      return;
    }

    if (meta && e.key === 'w') {
      e.preventDefault();
      if (currentTab) closeTab(currentTab);
      return;
    }

    if (meta && e.key === 'k') {
      e.preventDefault();
      onCommandPalette();
      return;
    }

    if (meta && e.shiftKey && e.key === 'F') {
      e.preventDefault();
      activePanel.set('search');
      return;
    }

    if (meta && e.key === "'") {
      e.preventDefault();
      splitMode.update((v) => !v);
      return;
    }

    if (meta && e.key === 'Tab') {
      e.preventDefault();
      const tabs = get(openTabs);
      const current = get(activeTabId);
      const idx = tabs.findIndex((t) => t.id === current);
      if (tabs.length > 1) {
        const next = e.shiftKey
          ? tabs[(idx - 1 + tabs.length) % tabs.length]
          : tabs[(idx + 1) % tabs.length];
        activeTabId.set(next.id);
      }
      return;
    }

    if (meta && e.key === '1') {
      e.preventDefault();
      activePanel.set('explorer');
    } else if (meta && e.key === '2') {
      e.preventDefault();
      activePanel.set('tasks');
    } else if (meta && e.key === '3') {
      e.preventDefault();
      activePanel.set('projects');
    } else if (meta && e.key === '4') {
      e.preventDefault();
      activePanel.set('search');
    }
  }

  onMount(() => {
    document.addEventListener('keydown', handleKey);
    return () => document.removeEventListener('keydown', handleKey);
  });
</script>
