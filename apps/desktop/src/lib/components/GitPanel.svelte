<script lang="ts">
  import { onMount } from 'svelte';
  import { workspaceRoot } from '$lib/stores/workspace';
  import {
    getStatus,
    getBranches,
    getLog,
    stage,
    unstage,
    stageAll,
    commit,
    checkout,
    createBranch,
    isRepo,
  } from '$lib/services/gitService';
  import type {
    GitStatusEntry,
    GitBranch,
    GitLogEntry,
  } from '$lib/services/gitService';
  import { notify } from '$lib/stores/notifications';

  let repoExists = $state(false);
  let status = $state<GitStatusEntry[]>([]);
  let branchInfo = $state<GitBranch>({ current: '', branches: [] });
  let log = $state<GitLogEntry[]>([]);
  let loading = $state(true);
  let commitMsg = $state('');
  let showNewBranch = $state(false);
  let newBranchName = $state('');
  let committing = $state(false);

  onMount(() => load());

  async function load() {
    loading = true;
    repoExists = await isRepo();
    if (repoExists) {
      status = await getStatus();
      branchInfo = await getBranches();
      log = await getLog();
    }
    loading = false;
  }

  async function toggleStage(entry: GitStatusEntry) {
    try {
      if (entry.staged) {
        await unstage(entry.path);
      } else {
        await stage(entry.path);
      }
      status = await getStatus();
    } catch (e) {
      notify(`Git error: ${e}`, 'error');
    }
  }

  async function doStageAll() {
    try {
      await stageAll();
      status = await getStatus();
    } catch (e) {
      notify(`Git error: ${e}`, 'error');
    }
  }

  async function doCommit() {
    if (!commitMsg.trim()) return;
    committing = true;
    try {
      await commit(commitMsg.trim());
      commitMsg = '';
      status = await getStatus();
      log = await getLog();
      notify('Committed successfully', 'info');
    } catch (e) {
      notify(`Commit failed: ${e}`, 'error');
    }
    committing = false;
  }

  async function switchBranch(name: string) {
    try {
      await checkout(name);
      branchInfo = await getBranches();
    } catch (e) {
      notify(`Checkout failed: ${e}`, 'error');
    }
  }

  async function doCreateBranch() {
    if (!newBranchName.trim()) return;
    try {
      await createBranch(newBranchName.trim());
      newBranchName = '';
      showNewBranch = false;
      branchInfo = await getBranches();
      notify(`Branch "${newBranchName}" created`, 'info');
    } catch (e) {
      notify(`Failed to create branch: ${e}`, 'error');
    }
  }

  const stagedCount = $derived(status.filter((e) => e.staged).length);
  const unstagedCount = $derived(status.filter((e) => !e.staged).length);
</script>

<div class="git-panel">
  {#if loading}
    <div class="empty typo-body">Loading...</div>
  {:else if !repoExists}
    <div class="empty">
      <span class="empty-icon">⎇</span>
      <span class="empty-text typo-body">Not a git repository</span>
      <span class="typo-small hint"
        >Open a git repo in Explorer to get started</span
      >
    </div>
  {:else}
    <div class="branch-bar typo-caption">
      <span class="branch-icon">⎇</span>
      <select
        class="branch-select"
        value={branchInfo.current}
        onchange={(e) => switchBranch((e.target as HTMLSelectElement).value)}
      >
        {#each branchInfo.branches as b}
          <option value={b} selected={b === branchInfo.current}>{b}</option>
        {/each}
      </select>
      <button
        class="new-branch-btn typo-small"
        onclick={() => (showNewBranch = !showNewBranch)}>+</button
      >
    </div>

    {#if showNewBranch}
      <div class="new-branch-form">
        <input
          type="text"
          class="typo-caption"
          placeholder="Branch name"
          bind:value={newBranchName}
          onkeydown={(e) => e.key === 'Enter' && doCreateBranch()}
        />
        <button class="action-btn typo-small" onclick={doCreateBranch}
          >Create</button
        >
      </div>
    {/if}

    <div class="changes-header typo-overline">
      <span>Changes ({unstagedCount})</span>
      <button class="stage-all-btn typo-small" onclick={doStageAll}
        >+ All</button
      >
    </div>

    <div class="changes-list">
      {#each status.filter((e) => !e.staged) as entry (entry.path)}
        <button class="change-row" onclick={() => toggleStage(entry)}>
          <span
            class="change-status"
            class:m={entry.status === 'M'}
            class:a={entry.status === 'A'}
            class:d={entry.status === 'D'}
            class:q={entry.status === '?'}
          >
            {entry.status === '?'
              ? '?'
              : entry.status === 'M'
                ? 'M'
                : entry.status === 'A'
                  ? 'A'
                  : entry.status === 'D'
                    ? 'D'
                    : '~'}
          </span>
          <span class="change-path typo-small">{entry.path}</span>
        </button>
      {:else}
        <div class="empty-changes typo-small">No changes</div>
      {/each}
    </div>

    {#if stagedCount > 0}
      <div class="changes-header typo-overline">
        <span>Staged ({stagedCount})</span>
      </div>
      <div class="changes-list">
        {#each status.filter((e) => e.staged) as entry (entry.path)}
          <button class="change-row staged" onclick={() => toggleStage(entry)}>
            <span
              class="change-status"
              class:m={entry.status === 'M'}
              class:a={entry.status === 'A'}
              class:d={entry.status === 'D'}
            >
              {entry.status === 'M'
                ? 'M'
                : entry.status === 'A'
                  ? 'A'
                  : entry.status === 'D'
                    ? 'D'
                    : '+'}
            </span>
            <span class="change-path typo-small">{entry.path}</span>
          </button>
        {/each}
      </div>
    {/if}

    <div class="commit-area">
      <input
        type="text"
        class="commit-input typo-caption"
        placeholder="Commit message..."
        bind:value={commitMsg}
        disabled={stagedCount === 0}
        onkeydown={(e) => e.key === 'Enter' && doCommit()}
      />
      <button
        class="commit-btn typo-caption"
        disabled={stagedCount === 0 || committing}
        onclick={doCommit}
      >
        {committing ? '...' : '✓'}
      </button>
    </div>

    <div class="changes-header typo-overline">Recent Commits</div>
    <div class="log-list">
      {#each log as entry}
        <div class="log-entry">
          <span class="log-hash typo-mono">{entry.hash}</span>
          <span class="log-msg typo-small">{entry.message}</span>
          <span class="log-meta typo-small">{entry.author} · {entry.date}</span>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .git-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-2);
    padding: var(--spacing-8);
    text-align: center;
    height: 100%;
  }
  .empty-icon {
    font-size: 32px;
    opacity: 0.3;
  }
  .empty-text {
    color: var(--color-muted);
  }
  .hint {
    color: var(--color-muted-soft);
  }

  .branch-bar {
    display: flex;
    align-items: center;
    gap: var(--spacing-1);
    padding: 6px var(--spacing-3);
    border-bottom: 1px solid var(--color-surface-dark-border);
  }
  .branch-icon {
    font-size: 16px;
  }
  .branch-select {
    flex: 1;
    padding: 2px 4px;
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark-soft);
    color: var(--color-on-dark-soft);
    border-radius: var(--radius-xs);
  }
  .new-branch-btn {
    padding: 1px 8px;
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark);
    color: var(--color-on-dark-soft);
    cursor: pointer;
    border-radius: var(--radius-xs);
  }
  .new-branch-btn:hover {
    background: var(--color-surface-dark-elevated);
  }

  .new-branch-form {
    display: flex;
    gap: var(--spacing-1);
    padding: var(--spacing-2);
    border-bottom: 1px solid var(--color-surface-dark-border);
  }
  .new-branch-form input {
    flex: 1;
    padding: 3px 6px;
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark);
    color: var(--color-on-dark);
    border-radius: var(--radius-xs);
    outline: none;
  }
  .action-btn {
    padding: 3px 8px;
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark-elevated);
    color: var(--color-on-dark-soft);
    cursor: pointer;
    border-radius: var(--radius-xs);
  }

  .changes-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 4px var(--spacing-3);
    border-bottom: 1px solid var(--color-surface-dark-border);
  }
  .stage-all-btn {
    background: none;
    border: none;
    color: var(--color-primary);
    cursor: pointer;
  }

  .changes-list {
    overflow-y: auto;
    max-height: 200px;
  }
  .change-row {
    display: flex;
    align-items: center;
    gap: var(--spacing-2);
    width: 100%;
    padding: 3px var(--spacing-3);
    border: none;
    background: none;
    color: var(--color-on-dark-soft);
    cursor: pointer;
    text-align: left;
  }
  .change-row:hover {
    background: var(--color-surface-dark);
  }
  .change-row.staged {
    opacity: 0.7;
  }
  .change-status {
    font-family: var(--font-mono);
    font-size: 11px;
    width: 18px;
    text-align: center;
    font-weight: 600;
  }
  .change-status.m {
    color: var(--color-warning);
  }
  .change-status.a {
    color: var(--color-success);
  }
  .change-status.d {
    color: var(--color-error);
  }
  .change-status.q {
    color: var(--color-muted);
  }
  .change-path {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .empty-changes {
    padding: var(--spacing-3);
    color: var(--color-muted-soft);
    text-align: center;
  }

  .commit-area {
    display: flex;
    gap: var(--spacing-1);
    padding: var(--spacing-2);
    border-bottom: 1px solid var(--color-surface-dark-border);
    border-top: 1px solid var(--color-surface-dark-border);
  }
  .commit-input {
    flex: 1;
    padding: 4px 8px;
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark);
    color: var(--color-on-dark);
    border-radius: var(--radius-xs);
    outline: none;
  }
  .commit-input:disabled {
    opacity: 0.4;
  }
  .commit-btn {
    padding: 4px 10px;
    border: none;
    background: var(--color-primary);
    color: var(--color-on-primary);
    cursor: pointer;
    border-radius: var(--radius-xs);
  }
  .commit-btn:disabled {
    opacity: 0.4;
  }

  .log-list {
    flex: 1;
    overflow-y: auto;
  }
  .log-entry {
    padding: 4px var(--spacing-3);
    border-bottom: 1px solid var(--color-surface-dark-border);
  }
  .log-hash {
    color: var(--color-primary);
    font-size: 11px;
  }
  .log-msg {
    display: block;
    color: var(--color-on-dark);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .log-meta {
    color: var(--color-muted-soft);
    font-size: 10px;
  }
</style>
