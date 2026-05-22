import { writable, get } from 'svelte/store';
import type { Command } from '$lib/types';

const registry = writable<Command[]>([]);

export const commands = { subscribe: registry.subscribe };

export function registerCommand(cmd: Command) {
  registry.update((cmds) => {
    const existing = cmds.findIndex((c) => c.id === cmd.id);
    if (existing >= 0) {
      const updated = [...cmds];
      updated[existing] = cmd;
      return updated;
    }
    return [...cmds, cmd];
  });
}

export function unregisterCommand(id: string) {
  registry.update((cmds) => cmds.filter((c) => c.id !== id));
}

export function executeCommand(id: string) {
  const cmd = get(registry).find((c) => c.id === id);
  cmd?.action();
}
