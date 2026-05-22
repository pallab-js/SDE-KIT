import { describe, it, expect, beforeEach } from 'vitest';
import { undoManager } from '../services/undoManager';
import { get } from 'svelte/store';

describe('undoManager', () => {
  beforeEach(() => {
    undoManager.clear();
  });

  it('starts with no undo/redo', () => {
    const state = get(undoManager.state);
    expect(state.canUndo).toBe(false);
    expect(state.canRedo).toBe(false);
  });

  it('can push and undo an action', () => {
    let value = 0;
    undoManager.push({
      id: 'test',
      label: 'Increment',
      undo: () => {
        value--;
      },
      redo: () => {
        value++;
      },
    });

    expect(get(undoManager.state).canUndo).toBe(true);
    undoManager.undo();
    expect(value).toBe(-1);
    expect(get(undoManager.state).canUndo).toBe(false);
  });

  it('can redo after undo', () => {
    let value = 0;
    undoManager.push({
      id: 'test',
      label: 'Increment',
      undo: () => {
        value--;
      },
      redo: () => {
        value++;
      },
    });

    value = 1;
    undoManager.undo();
    expect(value).toBe(0);
    expect(get(undoManager.state).canRedo).toBe(true);

    undoManager.redo();
    expect(value).toBe(1);
    expect(get(undoManager.state).canRedo).toBe(false);
  });

  it('clears future on new push', () => {
    undoManager.push({ id: '1', label: 'A', undo: () => {}, redo: () => {} });
    undoManager.undo();
    expect(get(undoManager.state).canRedo).toBe(true);

    undoManager.push({ id: '2', label: 'B', undo: () => {}, redo: () => {} });
    expect(get(undoManager.state).canRedo).toBe(false);
  });

  it('clear resets state', () => {
    undoManager.push({ id: '1', label: 'A', undo: () => {}, redo: () => {} });
    undoManager.clear();
    const state = get(undoManager.state);
    expect(state.canUndo).toBe(false);
    expect(state.canRedo).toBe(false);
  });
});
