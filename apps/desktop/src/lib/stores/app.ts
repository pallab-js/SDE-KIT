import { writable } from 'svelte/store';
import type { ApiError } from '$lib/services/api';

export const globalError = writable<ApiError | null>(null);
export const isLoading = writable<boolean>(false);

export function withLoading<T>(fn: () => Promise<T>): Promise<T> {
  isLoading.set(true);
  return fn().finally(() => isLoading.set(false));
}
