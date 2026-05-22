import { writable, derived, get } from 'svelte/store';

export interface EditorState {
  path: string;
  content: string;
  dirty: boolean;
}

export const fileContents = writable<Map<string, EditorState>>(new Map());
export const splitMode = writable<boolean>(false);
export const splitRatio = writable<number>(0.5);

export const dirtyPaths = derived(fileContents, ($fc) => {
  return new Set(
    [...$fc.entries()].filter(([_, v]) => v.dirty).map(([k]) => k)
  );
});

export function setFileContent(path: string, content: string) {
  fileContents.update((map) => {
    const existing = map.get(path);
    map.set(path, {
      path,
      content,
      dirty: existing?.dirty ?? false,
    });
    return new Map(map);
  });
  evictLruIfNeeded();
}

export function markDirty(path: string, dirty: boolean) {
  fileContents.update((map) => {
    const entry = map.get(path);
    if (entry) {
      entry.dirty = dirty;
      return new Map(map);
    }
    return map;
  });
}

export function updateContent(path: string, content: string) {
  fileContents.update((map) => {
    const existing = map.get(path);
    if (existing) {
      map.set(path, { ...existing, content, dirty: true });
      return new Map(map);
    }
    return map;
  });
}

export function getFileState(path: string): EditorState | undefined {
  return get(fileContents).get(path);
}

const MAX_CACHED_FILES = 50;

export function evictFileContent(path: string) {
  fileContents.update((map) => {
    map.delete(path);
    return new Map(map);
  });
}

export function evictLruIfNeeded() {
  fileContents.update((map) => {
    if (map.size > MAX_CACHED_FILES) {
      const toDelete = [...map.keys()].slice(0, map.size - MAX_CACHED_FILES);
      for (const k of toDelete) map.delete(k);
    }
    return new Map(map);
  });
}
