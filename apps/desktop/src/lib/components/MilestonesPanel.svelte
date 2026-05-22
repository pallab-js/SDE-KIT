<script lang="ts">
  import { onMount } from 'svelte';
  import {
    getMilestones,
    createMilestone,
    updateMilestoneStatus,
    deleteMilestone,
  } from '$lib/services/api';
  import type { Milestone } from '$lib/types';
  import MilestoneCard from './MilestoneCard.svelte';

  let milestones = $state<Milestone[]>([]);
  let loading = $state(true);
  let showNew = $state(false);
  let newTitle = $state('');
  let newDesc = $state('');
  let newDue = $state('');

  onMount(() => load());

  async function load() {
    loading = true;
    try {
      milestones = await getMilestones();
    } catch {
      milestones = [];
    }
    loading = false;
  }

  async function add() {
    if (!newTitle.trim()) return;
    try {
      await createMilestone(
        newTitle.trim(),
        newDesc || undefined,
        newDue || undefined
      );
      newTitle = '';
      newDesc = '';
      newDue = '';
      showNew = false;
      load();
    } catch {}
  }

  async function toggle(id: string, current: string) {
    const next = current === 'open' ? 'closed' : 'open';
    try {
      await updateMilestoneStatus(id, next);
      load();
    } catch {}
  }

  async function remove(id: string) {
    try {
      await deleteMilestone(id);
      load();
    } catch {}
  }
</script>

<div class="milestones-panel">
  <div class="panel-header">
    <button class="add-btn typo-body" onclick={() => (showNew = !showNew)}
      >+</button
    >
  </div>

  {#if showNew}
    <div class="new-form">
      <input
        type="text"
        class="typo-caption"
        placeholder="Milestone title"
        bind:value={newTitle}
      />
      <input
        type="text"
        class="typo-caption"
        placeholder="Description (optional)"
        bind:value={newDesc}
      />
      <input
        type="date"
        class="typo-caption"
        placeholder="Due date"
        bind:value={newDue}
      />
      <div class="form-actions">
        <button class="btn-primary typo-caption" onclick={add}>Create</button>
        <button
          class="btn-secondary typo-caption"
          onclick={() => (showNew = false)}>Cancel</button
        >
      </div>
    </div>
  {/if}

  <div class="milestone-list">
    {#if loading}
      <div class="empty">Loading...</div>
    {:else if milestones.length === 0}
      <div class="empty">
        <span class="empty-icon">🏁</span>
        <span class="empty-text typo-body">No milestones yet</span>
      </div>
    {:else}
      {#each milestones as ms (ms.id)}
        <MilestoneCard {ms} onToggle={toggle} onRemove={remove} />
      {/each}
    {/if}
  </div>
</div>

<style>
  .milestones-panel {
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
  }
  .add-btn {
    width: 20px;
    height: 20px;
    border: none;
    background: var(--color-surface-dark);
    color: var(--color-on-dark-soft);
    cursor: pointer;
    border-radius: var(--radius-xs);
    line-height: 1;
  }
  .add-btn:hover {
    background: var(--color-surface-dark-elevated);
    color: var(--color-on-dark);
  }

  .new-form {
    padding: var(--spacing-2);
    display: flex;
    flex-direction: column;
    gap: var(--spacing-1);
    border-bottom: 1px solid var(--color-surface-dark-border);
  }
  .new-form input {
    padding: 4px var(--spacing-2);
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark);
    color: var(--color-on-dark);
    border-radius: var(--radius-xs);
    outline: none;
  }
  .new-form input:focus {
    border-color: var(--color-primary);
  }
  .form-actions {
    display: flex;
    gap: var(--spacing-1);
  }
  .btn-primary,
  .btn-secondary {
    padding: 3px 10px;
    border: 1px solid var(--color-surface-dark-border);
    border-radius: var(--radius-xs);
    cursor: pointer;
  }
  .btn-primary {
    background: var(--color-primary);
    color: var(--color-on-primary);
    border-color: var(--color-primary);
  }
  .btn-secondary {
    background: var(--color-surface-dark-elevated);
    color: var(--color-on-dark-soft);
  }
  .btn-secondary:hover {
    background: var(--color-surface-dark-soft);
  }

  .milestone-list {
    flex: 1;
    overflow-y: auto;
    padding: var(--spacing-1) 0;
  }
  .milestone-card {
    margin: var(--spacing-1) var(--spacing-2);
    padding: var(--spacing-2);
    background: var(--color-surface-dark-soft);
    border: 1px solid var(--color-surface-dark-border);
    border-radius: var(--radius-md);
    border-left: 3px solid var(--color-primary);
  }
  .milestone-card.closed {
    opacity: 0.6;
    border-left-color: var(--color-muted);
  }
  .ms-header {
    display: flex;
    align-items: center;
    gap: var(--spacing-2);
  }
  .ms-toggle {
    border: none;
    background: none;
    cursor: pointer;
    color: var(--color-primary);
    padding: 0;
    flex-shrink: 0;
  }
  .milestone-card.closed .ms-toggle {
    color: var(--color-muted);
  }
  .ms-title {
    flex: 1;
    color: var(--color-on-dark);
    font-weight: 500;
  }
  .ms-desc {
    color: var(--color-on-dark-soft);
    margin: 4px 0 0 22px;
  }
  .ms-meta {
    display: flex;
    align-items: center;
    gap: var(--spacing-2);
    margin-top: var(--spacing-1);
    color: var(--color-muted);
    padding-left: 22px;
  }
  .ms-due.overdue {
    color: var(--color-error);
  }
  .ms-status {
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  .ms-status.open {
    color: var(--color-primary);
  }
  .ms-status.closed {
    color: var(--color-muted);
  }
  .delete-btn {
    width: 18px;
    height: 18px;
    border: none;
    background: none;
    color: var(--color-muted-soft);
    cursor: pointer;
    flex-shrink: 0;
  }
  .delete-btn:hover {
    color: var(--color-error);
  }
  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--spacing-2);
    padding: var(--spacing-8);
    text-align: center;
  }
  .empty-icon {
    font-size: 24px;
    opacity: 0.3;
  }
  .empty-text {
    color: var(--color-muted);
  }
  .ms-progress {
    margin: 4px 0 0 22px;
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .ms-bar-track {
    height: 4px;
    flex: 1;
    background: var(--color-surface-dark-border);
    border-radius: 2px;
    overflow: hidden;
  }
  .ms-bar {
    height: 100%;
    background: var(--color-success);
    border-radius: 2px;
    transition: width 0.3s;
  }
  .ms-progress-label {
    color: var(--color-muted);
    font-size: 11px;
    white-space: nowrap;
  }
</style>
