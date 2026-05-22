/**
 * Runtime type guards and validation utilities
 * Prevents AI-hallucinated values and catches type errors early.
 */

export function assertNever(x: never): never {
  throw new Error(`Unexpected value: ${JSON.stringify(x)}`);
}

export function isLocalPath(path: string): boolean {
  return (
    path.startsWith('/') ||
    path.startsWith('file://') ||
    /^[A-Z]:\\/.test(path) ||
    path.startsWith('./') ||
    path.startsWith('../')
  );
}

export function clamp(value: number, min: number, max: number): number {
  return Math.min(max, Math.max(min, value));
}

export function inRange(value: number, min: number, max: number): boolean {
  return value >= min && value <= max;
}

export function isThemeMode(value: unknown): value is 'light' | 'dark' {
  return value === 'light' || value === 'dark';
}

export function isTaskStatus(
  value: unknown
): value is 'todo' | 'doing' | 'done' {
  return value === 'todo' || value === 'doing' || value === 'done';
}

export function isTaskPriority(
  value: unknown
): value is 'low' | 'medium' | 'high' {
  return value === 'low' || value === 'medium' || value === 'high';
}

export function isMilestoneStatus(value: unknown): value is 'open' | 'closed' {
  return value === 'open' || value === 'closed';
}

export function isPanelId(value: unknown): boolean {
  const valid = [
    'explorer',
    'projects',
    'tasks',
    'milestones',
    'graphs',
    'search',
    'activity',
    'layouts',
    'notes',
    'inspector',
    'dashboard',
    'git',
  ];
  return typeof value === 'string' && valid.includes(value);
}

export function deterministicId(seed: string, index: number): string {
  let hash = 0;
  for (let i = 0; i < seed.length; i++) {
    hash = (hash << 5) - hash + seed.charCodeAt(i);
    hash |= 0;
  }
  return `sdek-${(hash + index).toString(36).slice(-8)}`;
}
