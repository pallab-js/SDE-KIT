import { writable, get } from 'svelte/store';
import { evictFileContent } from './editor';
import type { Tab, PanelId } from '$lib/types';

export const activePanel = writable<PanelId | null>('explorer');
export const openTabs = writable<Tab[]>([]);
export const activeTabId = writable<string | null>(null);
export const bottomPanelOpen = writable(false);
export const sidebarWidth = writable(240);
export const leftSidebarVisible = writable(false);
export const workspaceRoot = writable<string | null>(null);

export function openTab(tab: Tab) {
  openTabs.update((tabs) => {
    if (tabs.some((t) => t.id === tab.id)) return tabs;
    const updated = [...tabs, tab];
    activeTabId.set(tab.id);
    return updated;
  });
}

export function closeTab(id: string) {
  openTabs.update((tabs) => {
    const tab = tabs.find((t) => t.id === id);
    if (tab?.filePath) evictFileContent(tab.filePath);
    const idx = tabs.findIndex((t) => t.id === id);
    const updated = tabs.filter((t) => t.id !== id);
    const current = get(activeTabId);
    if (current === id && updated.length > 0) {
      const next = Math.min(idx, updated.length - 1);
      activeTabId.set(updated[next].id);
    } else if (updated.length === 0) {
      activeTabId.set(null);
    }
    return updated;
  });
}

let _lastPanel: PanelId = 'explorer';

export function togglePanel(id: PanelId) {
  activePanel.update((current) => {
    if (current === id) {
      _lastPanel = id;
      return null;
    }
    _lastPanel = current ?? _lastPanel;
    return id;
  });
}

export function toggleSidebar() {
  activePanel.update((current) => {
    if (current === null) return _lastPanel;
    _lastPanel = current;
    return null;
  });
}

export function toggleBottom() {
  bottomPanelOpen.update((v) => !v);
}
