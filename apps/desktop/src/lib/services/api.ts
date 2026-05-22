import type { Project, Task, Milestone, Note } from '$lib/types';
import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import { globalError } from '$lib/stores/app';

export interface ApiError {
  code: string;
  message: string;
}

export function isApiError(e: unknown): e is ApiError {
  return typeof e === 'object' && e !== null && 'code' in e && 'message' in e;
}

async function invoke<T>(
  cmd: string,
  args?: Record<string, unknown>
): Promise<T> {
  try {
    return await tauriInvoke<T>(cmd, args);
  } catch (e) {
    const err = isApiError(e)
      ? e
      : ({ code: 'UNKNOWN', message: String(e) } satisfies ApiError);
    globalError.set(err);
    throw err;
  }
}

// Projects
export function getProjects(): Promise<Project[]> {
  return invoke('get_projects');
}
export function createProject(
  name: string,
  path: string,
  description?: string
): Promise<Project> {
  return invoke('create_project', { name, path, description });
}
export function updateProject(
  id: string,
  name?: string,
  path?: string,
  description?: string
): Promise<void> {
  return invoke('update_project', { id, name, path, description });
}
export function deleteProject(id: string): Promise<void> {
  return invoke('delete_project', { id });
}

// Tasks
export function getTasks(limit?: number, offset?: number): Promise<Task[]> {
  return invoke('get_tasks', { limit, offset });
}
export function getTasksByProject(
  projectId: string,
  limit?: number,
  offset?: number
): Promise<Task[]> {
  return invoke('get_tasks_by_project', { projectId, limit, offset });
}
export function createTask(
  title: string,
  description?: string,
  priority?: string,
  projectId?: string,
  milestoneId?: string
): Promise<Task> {
  return invoke('create_task', {
    title,
    description,
    priority,
    projectId,
    milestoneId,
  });
}
export function updateTask(
  id: string,
  title?: string,
  description?: string,
  priority?: string
): Promise<void> {
  return invoke('update_task', { id, title, description, priority });
}
export function updateTaskStatus(id: string, status: string): Promise<void> {
  return invoke('update_task_status', { id, status });
}
export function deleteTask(id: string): Promise<void> {
  return invoke('delete_task', { id });
}

// Milestones
export function getMilestones(
  limit?: number,
  offset?: number
): Promise<Milestone[]> {
  return invoke('get_milestones', { limit, offset });
}
export function createMilestone(
  title: string,
  description?: string,
  dueDate?: string,
  projectId?: string
): Promise<Milestone> {
  return invoke('create_milestone', { title, description, dueDate, projectId });
}
export function updateMilestoneStatus(
  id: string,
  status: string
): Promise<void> {
  return invoke('update_milestone_status', { id, status });
}
export function deleteMilestone(id: string): Promise<void> {
  return invoke('delete_milestone', { id });
}
export function assignTaskToMilestone(
  taskId: string,
  milestoneId: string | null
): Promise<void> {
  return invoke('assign_task_to_milestone', { taskId, milestoneId });
}

// Notes (Phase 11 Dedicated Notes Table)
export function getNotes(
  projectId?: string,
  limit?: number,
  offset?: number
): Promise<Note[]> {
  return invoke('get_notes', { projectId, limit, offset });
}
export function getNote(id: string): Promise<Note | null> {
  return invoke('get_note', { id });
}
export function createNote(
  title: string,
  content: string,
  projectId?: string
): Promise<Note> {
  return invoke('create_note', { title, content, projectId });
}
export function updateNote(
  id: string,
  title?: string,
  content?: string,
  projectId?: string | null
): Promise<void> {
  return invoke('update_note', { id, title, content, projectId });
}
export function deleteNote(id: string): Promise<void> {
  return invoke('delete_note', { id });
}
