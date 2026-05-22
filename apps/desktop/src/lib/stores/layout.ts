import {
  activePanel,
  sidebarWidth,
  bottomPanelOpen,
  openTabs,
  activeTabId,
  leftSidebarVisible,
} from './workspace';
import type { LayoutConfig } from '$lib/types';

const LAYOUT_KEY = 'layout_state';

async function persist(): Promise<void> {
  const config: LayoutConfig = {
    activePanel: null,
    sidebarWidth: 240,
    leftSidebarVisible: false,
    bottomPanelOpen: false,
    openTabs: [],
    activeTabId: null,
  };
  activePanel.subscribe((v) => (config.activePanel = v))();
  sidebarWidth.subscribe((v) => (config.sidebarWidth = v))();
  leftSidebarVisible.subscribe((v) => (config.leftSidebarVisible = v))();
  bottomPanelOpen.subscribe((v) => (config.bottomPanelOpen = v))();
  openTabs.subscribe((v) => (config.openTabs = v))();
  activeTabId.subscribe((v) => (config.activeTabId = v))();

  try {
    const { invoke } = await import('@tauri-apps/api/core');
    await invoke('set_workspace_state', {
      key: LAYOUT_KEY,
      value: JSON.stringify(config),
    });
  } catch {
    localStorage.setItem(LAYOUT_KEY, JSON.stringify(config));
  }
}

async function restore(): Promise<LayoutConfig | null> {
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    const result = await invoke<string | null>('get_workspace_state', {
      key: LAYOUT_KEY,
    });
    if (result) return JSON.parse(result);
  } catch {
    const stored = localStorage.getItem(LAYOUT_KEY);
    if (stored) return JSON.parse(stored);
  }
  return null;
}

let saveTimeout: ReturnType<typeof setTimeout>;
function schedulePersist() {
  clearTimeout(saveTimeout);
  saveTimeout = setTimeout(persist, 500);
}

export function subscribeAndPersist() {
  activePanel.subscribe(schedulePersist);
  sidebarWidth.subscribe(schedulePersist);
  leftSidebarVisible.subscribe(schedulePersist);
  bottomPanelOpen.subscribe(schedulePersist);
  openTabs.subscribe(schedulePersist);
  activeTabId.subscribe(schedulePersist);
}

export async function restoreLayout(): Promise<void> {
  const config = await restore();
  if (!config) return;
  if (config.activePanel) activePanel.set(config.activePanel);
  if (config.sidebarWidth) sidebarWidth.set(config.sidebarWidth);
  leftSidebarVisible.set(config.leftSidebarVisible ?? false);
  if (config.bottomPanelOpen) bottomPanelOpen.set(config.bottomPanelOpen);
  if (config.openTabs.length > 0) {
    openTabs.set(config.openTabs);
    if (config.activeTabId) activeTabId.set(config.activeTabId);
  }
}
