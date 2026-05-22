import { vi } from 'vitest';

// Mock Tauri API core
vi.mock('@tauri-apps/api/core', () => {
  return {
    invoke: vi.fn(async (cmd: string, args?: Record<string, unknown>) => {
      if (cmd === 'get_workspace_state') {
        if (args?.key === 'theme_mode') {
          return 'dark';
        }
      }
      return null;
    }),
  };
});

// Mock Tauri API event
vi.mock('@tauri-apps/api/event', () => {
  return {
    listen: vi.fn(async (_event: string, _callback: () => void) => {
      return () => {};
    }),
  };
});

// Mock browser globals that might be missing in a pure Node test environment
if (typeof globalThis.document === 'undefined') {
  const doc = {
    documentElement: {
      setAttribute: vi.fn(),
      getAttribute: vi.fn(),
    },
  };
  globalThis.document = doc as unknown as Document;
}

if (typeof globalThis.localStorage === 'undefined') {
  const store: Record<string, string> = {};
  globalThis.localStorage = {
    getItem: vi.fn((key: string) => store[key] || null),
    setItem: vi.fn((key: string, value: string) => {
      store[key] = value;
    }),
    removeItem: vi.fn((key: string) => {
      delete store[key];
    }),
    clear: vi.fn(() => {
      for (const k in store) delete store[k];
    }),
    length: 0,
    key: vi.fn(() => null),
  };
}
