<script lang="ts">
  import type { Task } from '$lib/types';

  let { task, onDragStart, onDelete } = $props<{
    task: Task;
    onDragStart: (e: DragEvent, id: string) => void;
    onDelete: (id: string) => void;
  }>();

  const priorityColors: Record<string, string> = {
    low: '#6c6a64',
    medium: '#d4a017',
    high: '#c64545',
  };
</script>

<div
  class="card typo-caption"
  draggable="true"
  role="listitem"
  aria-grabbed="false"
  ondragstart={(e) => onDragStart(e, task.id)}
  style="border-left: 3px solid {priorityColors[task.priority]}"
>
  <div class="card-header">
    <span class="card-title">{task.title}</span>
    <button class="delete-btn" onclick={() => onDelete(task.id)}>×</button>
  </div>
  <span class="card-priority" style="color: {priorityColors[task.priority]}">
    {task.priority}
  </span>
</div>

<style>
  .card {
    padding: 6px var(--spacing-2);
    background: var(--color-surface-dark-elevated);
    border-radius: var(--radius-xs);
    cursor: grab;
    display: flex;
    flex-direction: column;
    gap: 4px;
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
    font-size: 9px;
    text-transform: uppercase;
    font-weight: 600;
  }
  .delete-btn {
    border: none;
    background: none;
    color: var(--color-muted-soft);
    cursor: pointer;
    line-height: 1;
    font-size: 16px;
    padding: 0;
  }
  .delete-btn:hover {
    color: var(--color-error);
  }
</style>
