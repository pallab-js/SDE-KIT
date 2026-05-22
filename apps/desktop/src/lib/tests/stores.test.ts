import { describe, it, expect, beforeEach, vi } from 'vitest';
import { get } from 'svelte/store';
import {
  openTabs,
  activeTabId,
  activePanel,
  bottomPanelOpen,
  sidebarWidth,
  openTab,
  closeTab,
  togglePanel,
  toggleSidebar,
  toggleBottom,
  workspaceRoot,
} from '$lib/stores/workspace';
import { theme } from '$lib/stores/theme';
import { invoke } from '@tauri-apps/api/core';

describe('Workspace Store Operations', () => {
  beforeEach(() => {
    // Reset stores before each test
    openTabs.set([]);
    activeTabId.set(null);
    activePanel.set('explorer');
    bottomPanelOpen.set(false);
    sidebarWidth.set(240);
    workspaceRoot.set(null);
  });

  it('should successfully open and activate a new tab', () => {
    const testTab = {
      id: 'file:test.js',
      title: 'test.js',
      icon: '📄',
      closable: true,
      filePath: '/some/path/test.js',
    };

    openTab(testTab);

    const tabs = get(openTabs);
    expect(tabs.length).toBe(1);
    expect(tabs[0]).toEqual(testTab);
    expect(get(activeTabId)).toBe('file:test.js');
  });

  it('should not add duplicate tabs and should activate on duplicate open', () => {
    const tab1 = { id: 't1', title: 'T1', icon: '📄', closable: true };
    const tab2 = {
      id: 't1',
      title: 'T1 Duplicate',
      icon: '📄',
      closable: true,
    };

    openTab(tab1);
    openTab(tab2);

    const tabs = get(openTabs);
    expect(tabs.length).toBe(1);
    expect(tabs[0].title).toBe('T1'); // Keeps original tab details
    expect(get(activeTabId)).toBe('t1');
  });

  it('should close a tab and handle active tab shift', () => {
    const tab1 = { id: 't1', title: 'T1', icon: '📄', closable: true };
    const tab2 = { id: 't2', title: 'T2', icon: '📄', closable: true };
    const tab3 = { id: 't3', title: 'T3', icon: '📄', closable: true };

    openTab(tab1);
    openTab(tab2);
    openTab(tab3);

    // Closing active tab (t3) should switch active tab to t2
    expect(get(activeTabId)).toBe('t3');
    closeTab('t3');

    expect(get(openTabs).length).toBe(2);
    expect(get(activeTabId)).toBe('t2');

    // Closing non-active tab should not switch active tab
    closeTab('t1');
    expect(get(openTabs).length).toBe(1);
    expect(get(activeTabId)).toBe('t2');

    // Closing final remaining tab sets active tab to null
    closeTab('t2');
    expect(get(openTabs).length).toBe(0);
    expect(get(activeTabId)).toBeNull();
  });

  it('should toggle sidebar panel states', () => {
    // Current is 'explorer'
    expect(get(activePanel)).toBe('explorer');

    // Toggle same panel closes it (sets to null)
    togglePanel('explorer');
    expect(get(activePanel)).toBeNull();

    // Toggle other panel opens it
    togglePanel('git');
    expect(get(activePanel)).toBe('git');

    // Toggle sidebar should toggle activePanel from non-null to null and vice versa
    toggleSidebar();
    expect(get(activePanel)).toBeNull();

    toggleSidebar();
    expect(get(activePanel)).toBe('git'); // Should restore the last panel
  });

  it('should toggle bottom panel visibility', () => {
    expect(get(bottomPanelOpen)).toBe(false);
    toggleBottom();
    expect(get(bottomPanelOpen)).toBe(true);
    toggleBottom();
    expect(get(bottomPanelOpen)).toBe(false);
  });
});

describe('Theme Store Operations', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should toggle the theme state and call tauri state persistence', async () => {
    // Initial state setup
    theme.apply('dark');
    expect(get(theme)).toBe('dark');

    // Toggle theme to light
    theme.toggle();
    expect(get(theme)).toBe('light');
    expect(invoke).toHaveBeenCalledWith('set_workspace_state', {
      key: 'theme_mode',
      value: 'light',
    });

    // Toggle theme back to dark
    theme.toggle();
    expect(get(theme)).toBe('dark');
    expect(invoke).toHaveBeenCalledWith('set_workspace_state', {
      key: 'theme_mode',
      value: 'dark',
    });
  });

  it('should correctly initialize theme from stored workspace settings', async () => {
    // Mock invoke to return 'light' for workspace state request
    vi.mocked(invoke).mockResolvedValueOnce('light');

    await theme.init();
    expect(get(theme)).toBe('light');
  });
});
