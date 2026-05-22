<script lang="ts">
  import { onMount } from 'svelte';
  import {
    getTasks,
    getMilestones,
    createTask,
    updateTaskStatus,
    deleteTask,
  } from '$lib/services/api';
  import type { Task, TaskStatus, Milestone } from '$lib/types';
  import TaskCard from './TaskCard.svelte';

  let tasks = $state<Task[]>([]);
  let milestones = $state<Milestone[]>([]);
  let selectedMilestoneId = $state<string | null>(null);
  let loading = $state(true);
  let view = $state<'list' | 'kanban'>('kanban');
  let newTitle = $state('');
  let newPriority = $state<'low' | 'medium' | 'high'>('medium');
  let dragging: string | null = $state(null);

  const columns: { id: TaskStatus; label: string }[] = [
    { id: 'todo', label: 'Todo' },
    { id: 'doing', label: 'In Progress' },
    { id: 'done', label: 'Done' },
  ];

  onMount(() => {
    load();
    loadMilestones();
  });

  async function load() {
    loading = true;
    try {
      tasks = await getTasks();
    } catch {
      tasks = [];
    }
    loading = false;
  }

  async function loadMilestones() {
    try {
      milestones = await getMilestones();
    } catch {
      milestones = [];
    }
  }

  function filteredTasks() {
    if (!selectedMilestoneId) return tasks;
    return tasks.filter((t) => t.milestoneId === selectedMilestoneId);
  }

  function getByStatus(status: TaskStatus) {
    return filteredTasks().filter((t) => t.status === status);
  }

  async function add() {
    if (!newTitle.trim()) return;
    try {
      await createTask(newTitle.trim(), undefined, newPriority);
      newTitle = '';
      load();
    } catch {}
  }

  async function remove(id: string) {
    try {
      await deleteTask(id);
      load();
    } catch {}
  }

  async function moveTo(id: string, status: TaskStatus) {
    try {
      await updateTaskStatus(id, status);
      load();
    } catch {}
  }

  function onDragStart(e: DragEvent, id: string) {
    dragging = id;
    e.dataTransfer?.setData('text/plain', id);
  }

  function onDragOver(e: DragEvent) {
    e.preventDefault();
  }

  function onDrop(e: DragEvent, status: TaskStatus) {
    e.preventDefault();
    const id = e.dataTransfer?.getData('text/plain');
    if (id) moveTo(id, status);
    dragging = null;
  }

  let showCount = $state(50);

  function viewport(node: HTMLElement) {
    const observer = new IntersectionObserver((entries) => {
      if (entries[0].isIntersecting) {
        showCount += 50;
      }
    });
    observer.observe(node);
    return {
      destroy() {
        observer.disconnect();
      },
    };
  }

  const priorityColors: Record<string, string> = {
    low: '#6c6a64',
    medium: '#d4a017',
    high: '#c64545',
  };
</script>

<div class="tasks-panel">
  <div class="panel-header">
    <div class="header-actions">
      <button
        class="view-btn typo-caption"
        class:active={view === 'list'}
        onclick={() => {
          view = 'list';
          showCount = 50;
        }}>☰</button
      >
      <button
        class="view-btn typo-caption"
        class:active={view === 'kanban'}
        onclick={() => {
          view = 'kanban';
          showCount = 50;
        }}>⊞</button
      >
    </div>
    <select
      class="milestone-filter typo-caption"
      bind:value={selectedMilestoneId}
      onchange={() => {
        showCount = 50;
      }}
    >
      <option value={null}>All Milestones</option>
      {#each milestones as m (m.id)}
        <option value={m.id}>{m.title}</option>
      {/each}
    </select>
  </div>

  <div class="new-task">
    <input
      type="text"
      class="typo-caption"
      placeholder="New task..."
      bind:value={newTitle}
      onkeydown={(e) => {
        if (e.key === 'Enter') add();
      }}
    />
    <select class="typo-caption" bind:value={newPriority}>
      <option value="low">Low</option>
      <option value="medium">Medium</option>
      <option value="high">High</option>
    </select>
    <button class="add-btn typo-body" onclick={add}>+</button>
  </div>

  {#if loading}
    <div class="empty typo-body">Loading...</div>
  {:else if view === 'list'}
    <div class="task-list" role="list">
      {#each filteredTasks().slice(0, showCount) as task (task.id)}
        <div
          class="task-row typo-caption"
          draggable="true"
          role="listitem"
          aria-grabbed="false"
          ondragstart={(e) => onDragStart(e, task.id)}
          style="border-left: 3px solid {priorityColors[task.priority]}"
        >
          <span
            class="status-dot"
            class:todo={task.status === 'todo'}
            class:doing={task.status === 'doing'}
            class:done={task.status === 'done'}
          ></span>
          <span class="task-title">{task.title}</span>
          <span
            class="task-priority"
            style="color: {priorityColors[task.priority]}">{task.priority}</span
          >
          <select
            value={task.status}
            onchange={(e) =>
              moveTo(
                task.id,
                (e.target as HTMLSelectElement).value as TaskStatus
              )}
          >
            <option value="todo">Todo</option>
            <option value="doing">Doing</option>
            <option value="done">Done</option>
          </select>
          <button class="delete-btn" onclick={() => remove(task.id)}>×</button>
        </div>
      {/each}
      {#if filteredTasks().length > showCount}
        <div class="scroll-trigger typo-small" use:viewport>
          Loading more tasks...
        </div>
      {/if}
    </div>
  {:else}
    <div class="kanban">
      {#each columns as col}
        <div
          class="kanban-col"
          role="region"
          aria-dropeffect="move"
          ondragover={onDragOver}
          ondrop={(e) => onDrop(e, col.id)}
        >
          <div class="col-header typo-overline">
            {col.label} ({getByStatus(col.id).length})
          </div>
          <div class="col-cards" role="list">
            {#each getByStatus(col.id).slice(0, showCount) as task (task.id)}
              <TaskCard {task} {onDragStart} onDelete={remove} />
            {/each}
            {#if getByStatus(col.id).length > showCount}
              <div class="scroll-trigger typo-small" use:viewport>
                Loading more...
              </div>
            {/if}
            {#if getByStatus(col.id).length === 0}
              <div class="card empty-card typo-caption">Drop tasks here</div>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .tasks-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px var(--spacing-3);
    border-bottom: 1px solid var(--color-surface-dark-border);
    gap: var(--spacing-2);
  }
  .milestone-filter {
    max-width: 160px;
    padding: 2px 4px;
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark-soft);
    color: var(--color-on-dark-soft);
    border-radius: var(--radius-xs);
  }
  .header-actions {
    display: flex;
    gap: 2px;
  }
  .view-btn {
    padding: 2px 6px;
    border: 1px solid transparent;
    background: none;
    color: var(--color-on-dark-soft);
    cursor: pointer;
    border-radius: var(--radius-xs);
  }
  .view-btn.active {
    border-color: var(--color-primary);
    color: var(--color-on-dark);
    background: var(--color-surface-dark);
  }

  .new-task {
    display: flex;
    gap: var(--spacing-1);
    padding: var(--spacing-2);
    border-bottom: 1px solid var(--color-surface-dark-border);
  }
  .new-task input {
    flex: 1;
    padding: 4px var(--spacing-2);
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark);
    color: var(--color-on-dark);
    border-radius: var(--radius-xs);
    outline: none;
  }
  .new-task input:focus {
    border-color: var(--color-primary);
  }
  .new-task select {
    padding: 4px;
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark-soft);
    color: var(--color-on-dark-soft);
    border-radius: var(--radius-xs);
  }
  .add-btn {
    padding: 4px 8px;
    border: none;
    background: var(--color-primary);
    color: var(--color-on-primary);
    cursor: pointer;
    border-radius: var(--radius-xs);
  }

  .task-list {
    flex: 1;
    overflow-y: auto;
    padding: var(--spacing-1) 0;
  }
  .task-row {
    display: flex;
    align-items: center;
    gap: var(--spacing-2);
    padding: 5px var(--spacing-3);
    cursor: grab;
  }
  .task-row:hover {
    background: var(--color-surface-dark);
  }
  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }
  .status-dot.todo {
    background: var(--color-muted-soft);
  }
  .status-dot.doing {
    background: var(--color-warning);
  }
  .status-dot.done {
    background: var(--color-success);
  }
  .delete-btn:hover {
    color: var(--color-error);
  }

  .kanban {
    display: flex;
    flex: 1;
    overflow-x: auto;
    gap: var(--spacing-2);
    padding: var(--spacing-2);
  }
  .kanban-col {
    flex: 1;
    min-width: 160px;
    display: flex;
    flex-direction: column;
    background: var(--color-surface-dark-soft);
    border-radius: var(--radius-md);
    border: 1px solid var(--color-surface-dark-border);
  }
  .col-header {
    padding: 6px var(--spacing-2);
    color: var(--color-on-dark-soft);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    border-bottom: 1px solid var(--color-surface-dark-border);
  }
  .col-cards {
    flex: 1;
    padding: var(--spacing-1);
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: var(--spacing-1);
  }
  .card {
    padding: 6px var(--spacing-2);
    background: var(--color-surface-dark-elevated);
    border-radius: var(--radius-xs);
    cursor: grab;
  }
  .card:hover {
    background: var(--color-surface-dark);
  }
  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: var(--spacing-1);
  }
  .card-title {
    color: var(--color-on-dark);
    word-break: break-word;
  }
  .card-priority {
    color: var(--color-muted);
    text-transform: uppercase;
  }
  .empty-card {
    color: var(--color-muted);
    font-style: italic;
    text-align: center;
    padding: var(--spacing-4);
    border: 1px dashed var(--color-surface-dark-border);
    cursor: default;
  }
  .empty {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--spacing-8);
    color: var(--color-muted);
  }
  .scroll-trigger {
    padding: var(--spacing-4);
    text-align: center;
    color: var(--color-muted);
    font-style: italic;
  }
</style>
