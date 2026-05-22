<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { openTab } from '$lib/stores/workspace';

  interface SearchResult {
    path: string;
    line_number: number;
    line: string;
  }

  let query = $state('');
  let caseSensitive = $state(false);
  let results = $state<SearchResult[]>([]);
  let loading = $state(false);
  let searched = $state(false);

  async function search() {
    if (!query.trim()) return;
    loading = true;
    searched = true;
    try {
      results = await invoke<SearchResult[]>('search_in_files', {
        query: query.trim(),
        caseSensitive,
      });
    } catch {
      results = [];
    }
    loading = false;
  }

  function openResult(r: SearchResult) {
    openTab({
      id: r.path,
      title: r.path.split('/').pop() ?? r.path,
      closable: true,
      filePath: r.path,
    });
  }
</script>

<div class="search-panel">
  <div class="search-bar">
    <input
      class="search-input typo-caption"
      type="text"
      placeholder="Search in files..."
      bind:value={query}
      onkeydown={(e) => e.key === 'Enter' && search()}
    />
    <label class="case-label typo-small">
      <input type="checkbox" bind:checked={caseSensitive} /> Aa
    </label>
    <button class="search-btn typo-caption" onclick={search}>Search</button>
  </div>
  <div class="results">
    {#if loading}
      <div class="empty typo-body">Searching...</div>
    {:else if searched && results.length === 0}
      <div class="empty typo-body">No results</div>
    {:else}
      {#each results as r (r.path + ':' + r.line_number)}
        <button class="result-row" onclick={() => openResult(r)}>
          <span class="result-path typo-mono">{r.path}:{r.line_number}</span>
          <span class="result-line typo-small">{r.line}</span>
        </button>
      {/each}
    {/if}
  </div>
</div>

<style>
  .search-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  .search-bar {
    display: flex;
    gap: var(--spacing-1);
    padding: var(--spacing-2);
    border-bottom: 1px solid var(--color-surface-dark-border);
  }
  .search-input {
    flex: 1;
    padding: 4px var(--spacing-2);
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark);
    color: var(--color-on-dark);
    border-radius: var(--radius-xs);
    outline: none;
  }
  .search-input:focus {
    border-color: var(--color-primary);
  }
  .case-label {
    display: flex;
    align-items: center;
    gap: 2px;
    color: var(--color-on-dark-soft);
    cursor: pointer;
    font-size: 11px;
  }
  .search-btn {
    padding: 4px 10px;
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark-elevated);
    color: var(--color-on-dark);
    border-radius: var(--radius-xs);
    cursor: pointer;
  }
  .results {
    flex: 1;
    overflow-y: auto;
  }
  .result-row {
    display: flex;
    flex-direction: column;
    gap: 2px;
    width: 100%;
    padding: 5px var(--spacing-3);
    border: none;
    background: none;
    text-align: left;
    cursor: pointer;
    border-bottom: 1px solid var(--color-surface-dark-border);
  }
  .result-row:hover {
    background: var(--color-surface-dark-elevated);
  }
  .result-path {
    color: var(--color-primary);
    font-size: 11px;
  }
  .result-line {
    color: var(--color-on-dark-soft);
    font-size: 12px;
    white-space: pre;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .empty {
    padding: var(--spacing-6);
    text-align: center;
    color: var(--color-muted);
  }
</style>
