<script lang="ts">
  import { onMount } from 'svelte';
  import {
    openTab,
    togglePanel,
    toggleSidebar,
    toggleBottom,
  } from '$lib/stores/workspace';
  import { commands } from '$lib/stores/commands';
  import { splitMode } from '$lib/stores/editor';
  import type { Command, PanelId } from '$lib/types';

  let { open = false, onClose }: { open: boolean; onClose: () => void } =
    $props();

  let query = $state('');
  let selectedIndex = $state(0);
  let inputEl: HTMLInputElement | undefined = $state();

  let filtered = $derived(
    query
      ? $commands.filter((c) =>
          c.label.toLowerCase().includes(query.toLowerCase())
        )
      : $commands.slice(0, 20)
  );

  $effect(() => {
    if (open && inputEl) {
      inputEl.focus();
      query = '';
      selectedIndex = 0;
    }
  });

  function onKeyDown(e: KeyboardEvent) {
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      selectedIndex = Math.min(selectedIndex + 1, filtered.length - 1);
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      selectedIndex = Math.max(selectedIndex - 1, 0);
    } else if (e.key === 'Enter' && filtered[selectedIndex]) {
      e.preventDefault();
      filtered[selectedIndex].action();
    } else if (e.key === 'Escape') {
      onClose();
    }
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="overlay"
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    onclick={onClose}
    onkeydown={(e) => e.key === 'Escape' && onClose()}
  >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="palette"
      onclick={(e) => e.stopPropagation()}
      onkeydown={onKeyDown}
    >
      <div class="search-box">
        <span class="search-icon typo-title">⌘</span>
        <input
          bind:this={inputEl}
          type="text"
          class="search-input typo-body"
          placeholder="Type a command..."
          bind:value={query}
        />
      </div>
      <div class="results" role="listbox">
        {#each filtered as cmd, i (cmd.id)}
          <button
            class="command-item typo-body"
            class:selected={i === selectedIndex}
            role="option"
            aria-selected={i === selectedIndex}
            onclick={() => {
              cmd.action();
            }}
            onmouseenter={() => (selectedIndex = i)}
          >
            <span class="cmd-icon typo-body">{cmd.icon ?? '○'}</span>
            <span class="cmd-label">{cmd.label}</span>
            <span class="cmd-category">{cmd.category}</span>
            {#if cmd.shortcut}
              <span class="cmd-shortcut typo-mono">{cmd.shortcut}</span>
            {/if}
          </button>
        {/each}
        {#if filtered.length === 0}
          <div class="no-results typo-body">No matching commands</div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: color-mix(in srgb, var(--color-surface-dark) 85%, transparent);
    display: flex;
    align-items: flex-start;
    justify-content: center;
    z-index: 1000;
    padding-top: 80px;
  }

  .palette {
    width: 520px;
    max-height: 400px;
    background: var(--color-surface-dark-elevated);
    border: 1px solid var(--color-surface-dark-border);
    border-radius: var(--radius-lg);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.6);
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: var(--spacing-2);
    padding: var(--spacing-3);
    border-bottom: 1px solid var(--color-surface-dark-border);
  }

  .search-icon {
    color: var(--color-muted);
  }

  .search-input {
    flex: 1;
    border: none;
    background: none;
    color: var(--color-on-dark);
    outline: none;
    font-family: inherit;
  }

  .search-input::placeholder {
    color: var(--color-muted-soft);
  }

  .results {
    flex: 1;
    overflow-y: auto;
    padding: var(--spacing-1);
  }

  .command-item {
    display: flex;
    align-items: center;
    gap: var(--spacing-2);
    width: 100%;
    padding: 8px var(--spacing-3);
    border: none;
    background: none;
    color: var(--color-on-dark);
    cursor: pointer;
    text-align: left;
    border-radius: var(--radius-sm);
  }

  .command-item.selected {
    background: var(--color-surface-dark);
    border-left: 2px solid var(--color-primary);
  }

  .command-item:hover {
    background: var(--color-surface-dark);
  }

  .cmd-icon {
    width: 20px;
    text-align: center;
    flex-shrink: 0;
  }

  .cmd-label {
    flex: 1;
  }

  .cmd-category {
    font-size: 11px;
    color: var(--color-muted);
    margin-right: var(--spacing-2);
    font-family: var(--font-sans);
  }

  .cmd-shortcut {
    color: var(--color-muted);
    background: var(--color-surface-dark);
    padding: 1px 6px;
    border-radius: 3px;
  }

  .no-results {
    padding: var(--spacing-6);
    text-align: center;
    color: var(--color-muted);
  }
</style>
