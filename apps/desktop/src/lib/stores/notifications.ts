import { writable } from 'svelte/store';

export type NotifLevel = 'info' | 'warn' | 'error';
export interface Notification {
  id: string;
  message: string;
  level: NotifLevel;
}

const store = writable<Notification[]>([]);
export const notifications = { subscribe: store.subscribe };

export function notify(message: string, level: NotifLevel = 'info') {
  const id = crypto.randomUUID();
  store.update((n) => [...n, { id, message, level }]);
  setTimeout(() => dismiss(id), level === 'error' ? 8000 : 3000);
}

export function dismiss(id: string) {
  store.update((n) => n.filter((x) => x.id !== id));
}
