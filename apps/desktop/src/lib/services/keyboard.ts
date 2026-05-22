/**
 * Keyboard Navigation Service
 * - Provides a global keybinding layer for Cmd+P, panel switching, etc.
 * - Integrates with the existing commands store for all commands
 * - Uses CustomEvent dispatch for loose coupling with components
 */

export type KeyCombo = {
  key: string;
  ctrl?: boolean;
  shift?: boolean;
  alt?: boolean;
  meta?: boolean;
};

export type ShortcutAction = {
  id: string;
  label: string;
  combo: KeyCombo;
  handler: () => void;
  context?: 'global' | 'editor' | 'graph' | 'tasks';
};

function matchesKey(event: KeyboardEvent, combo: KeyCombo): boolean {
  if (event.key.toLowerCase() !== combo.key.toLowerCase()) return false;
  if (combo.ctrl && !event.ctrlKey) return false;
  if (combo.shift && !event.shiftKey) return false;
  if (combo.alt && !event.altKey) return false;
  if (combo.meta && !event.metaKey) return false;
  // When combo specifies modifiers, the modifiers should match exactly
  const hasUnwantedModifier =
    (!combo.ctrl && event.ctrlKey) ||
    (!combo.shift && event.shiftKey) ||
    (!combo.alt && event.altKey) ||
    (!combo.meta && event.metaKey);
  if (hasUnwantedModifier) return false;
  return true;
}

export class KeyboardService {
  private static instance: KeyboardService;
  private actions = new Map<string, ShortcutAction>();
  private activeContext: ShortcutAction['context'] = 'global';
  private paletteOpen = false;

  private constructor() {
    this.registerDefaults();
    this.bindGlobalListener();
  }

  static getInstance(): KeyboardService {
    if (!KeyboardService.instance) {
      KeyboardService.instance = new KeyboardService();
    }
    return KeyboardService.instance;
  }

  register(action: ShortcutAction): void {
    this.actions.set(action.id, action);
  }

  unregister(id: string): void {
    this.actions.delete(id);
  }

  setContext(ctx: ShortcutAction['context']): void {
    this.activeContext = ctx;
  }

  setPaletteOpen(open: boolean): void {
    this.paletteOpen = open;
  }

  private registerDefaults(): void {
    this.register({
      id: 'palette:toggle',
      label: 'Open Command Palette',
      combo: { key: 'p', meta: true },
      handler: () => {
        document.dispatchEvent(
          new CustomEvent('palette:toggle', { detail: { open: true } })
        );
      },
      context: 'global',
    });

    this.register({
      id: 'editor:save',
      label: 'Save File',
      combo: { key: 's', meta: true },
      handler: () => document.dispatchEvent(new CustomEvent('editor:save')),
      context: 'editor',
    });

    this.register({
      id: 'nav:next-panel',
      label: 'Focus Next Panel',
      combo: { key: 'Tab', ctrl: true },
      handler: () => dispatchNav('next'),
      context: 'global',
    });

    this.register({
      id: 'nav:prev-panel',
      label: 'Focus Previous Panel',
      combo: { key: 'Tab', ctrl: true, shift: true },
      handler: () => dispatchNav('prev'),
      context: 'global',
    });

    this.register({
      id: 'graph:recenter',
      label: 'Recenter Graph',
      combo: { key: '0', ctrl: true },
      handler: () => document.dispatchEvent(new CustomEvent('graph:recenter')),
      context: 'graph',
    });

    this.register({
      id: 'task:new',
      label: 'New Task',
      combo: { key: 'n', meta: true },
      handler: () => document.dispatchEvent(new CustomEvent('task:create')),
      context: 'tasks',
    });

    this.register({
      id: 'sidebar:toggle',
      label: 'Toggle Sidebar',
      combo: { key: 'b', meta: true },
      handler: () => document.dispatchEvent(new CustomEvent('sidebar:toggle')),
      context: 'global',
    });

    this.register({
      id: 'bottom:toggle',
      label: 'Toggle Bottom Panel',
      combo: { key: 'j', meta: true },
      handler: () => document.dispatchEvent(new CustomEvent('bottom:toggle')),
      context: 'global',
    });
  }

  private bindGlobalListener(): void {
    document.addEventListener('keydown', (e: KeyboardEvent) => {
      if (this.paletteOpen) return;

      for (const action of this.actions.values()) {
        if (
          action.context !== 'global' &&
          action.context !== this.activeContext
        )
          continue;
        if (matchesKey(e, action.combo)) {
          e.preventDefault();
          e.stopPropagation();
          action.handler();
          return;
        }
      }
    });
  }
}

function dispatchNav(dir: 'next' | 'prev'): void {
  const panels = ['file-tree', 'editor', 'tasks', 'graph', 'terminal'];
  const current =
    document.activeElement?.getAttribute('data-panel') || panels[0];
  const idx = panels.indexOf(current);
  const next =
    dir === 'next'
      ? panels[(idx + 1) % panels.length]
      : panels[(idx - 1 + panels.length) % panels.length];

  const el = document.querySelector(
    `[data-panel="${next}"]`
  ) as HTMLElement | null;
  el?.focus();

  const announcer = document.getElementById('keyboard-announcer');
  if (announcer) {
    announcer.textContent = '';
    requestAnimationFrame(() => {
      announcer.textContent = `Focused ${next}`;
    });
  }
}
