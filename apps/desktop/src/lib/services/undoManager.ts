import { writable, get } from 'svelte/store';

export interface UndoAction {
  id: string;
  label: string;
  undo: () => void;
  redo: () => void;
}

export interface UndoState {
  canUndo: boolean;
  canRedo: boolean;
  currentLabel: string | null;
}

function createUndoManager() {
  const past = writable<UndoAction[]>([]);
  const future = writable<UndoAction[]>([]);
  const state = writable<UndoState>({
    canUndo: false,
    canRedo: false,
    currentLabel: null,
  });

  function updateState() {
    state.set({
      canUndo: get(past).length > 0,
      canRedo: get(future).length > 0,
      currentLabel:
        get(past).length > 0 ? get(past)[get(past).length - 1].label : null,
    });
  }

  function push(action: UndoAction) {
    past.update((p) => [...p, action]);
    future.set([]);
    updateState();
  }

  function undo() {
    const p = get(past);
    if (p.length === 0) return;
    const action = p[p.length - 1];
    action.undo();
    past.update((p) => p.slice(0, -1));
    future.update((f) => [...f, action]);
    updateState();
  }

  function redo() {
    const f = get(future);
    if (f.length === 0) return;
    const action = f[f.length - 1];
    action.redo();
    past.update((p) => [...p, action]);
    future.update((f) => f.slice(0, -1));
    updateState();
  }

  function clear() {
    past.set([]);
    future.set([]);
    updateState();
  }

  return {
    state: { subscribe: state.subscribe },
    push,
    undo,
    redo,
    clear,
  };
}

export const undoManager = createUndoManager();
