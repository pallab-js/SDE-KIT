import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export type ThemeMode = 'dark' | 'light';

const STORAGE_KEY = 'theme_mode';

function createTheme() {
  const store = writable<ThemeMode>('dark');

  async function init() {
    try {
      const stored = await invoke<string | null>('get_workspace_state', {
        key: STORAGE_KEY,
      });
      if (stored === 'light' || stored === 'dark') {
        apply(stored);
      }
    } catch {
      const local = localStorage.getItem(STORAGE_KEY);
      if (local === 'light' || local === 'dark') apply(local);
    }
  }

  function apply(mode: ThemeMode) {
    document.documentElement.setAttribute('data-theme', mode);
    store.set(mode);
    try {
      invoke('set_workspace_state', { key: STORAGE_KEY, value: mode });
    } catch {
      localStorage.setItem(STORAGE_KEY, mode);
    }
  }

  function toggle() {
    const current = getCurrent();
    const next: ThemeMode = current === 'dark' ? 'light' : 'dark';
    apply(next);
  }

  function getCurrent(): ThemeMode {
    let val: ThemeMode = 'dark';
    store.subscribe((v) => {
      val = v;
    })();
    return val;
  }

  return { subscribe: store.subscribe, init, toggle, apply };
}

export const theme = createTheme();
