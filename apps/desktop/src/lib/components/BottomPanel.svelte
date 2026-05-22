<script lang="ts">
  import { bottomPanelOpen, toggleBottom } from '$lib/stores/workspace';
  import { onMount } from 'svelte';
  import TerminalPanel from './TerminalPanel.svelte';

  let logs = $state([
    {
      level: 'info',
      message: 'SDE Kit initialized',
      time: new Date().toLocaleTimeString(),
    },
    {
      level: 'info',
      message: 'SQLite database connected',
      time: new Date().toLocaleTimeString(),
    },
    {
      level: 'info',
      message: 'Workspace ready',
      time: new Date().toLocaleTimeString(),
    },
  ]);

  let selectedTab = $state<'console' | 'problems' | 'output' | 'terminal'>(
    'console'
  );

  function addLog(level: string, message: string) {
    logs = [...logs, { level, message, time: new Date().toLocaleTimeString() }];
  }
</script>

{#if $bottomPanelOpen}
  <div class="bottom-panel">
    <div class="panel-header">
      <div class="header-tabs">
        <button
          class="header-tab typo-overline"
          class:active={selectedTab === 'console'}
          onclick={() => (selectedTab = 'console')}
        >
          Console
        </button>
        <button
          class="header-tab typo-overline"
          class:active={selectedTab === 'problems'}
          onclick={() => (selectedTab = 'problems')}
        >
          Problems
        </button>
        <button
          class="header-tab typo-overline"
          class:active={selectedTab === 'output'}
          onclick={() => (selectedTab = 'output')}
        >
          Output
        </button>
        <button
          class="header-tab typo-overline"
          class:active={selectedTab === 'terminal'}
          onclick={() => (selectedTab = 'terminal')}
        >
          Terminal
        </button>
      </div>
      <button class="close-btn typo-body" onclick={toggleBottom}>×</button>
    </div>
    <div class="panel-body typo-mono">
      {#if selectedTab === 'console'}
        {#each logs as log}
          <div class="log-entry {log.level}">
            <span class="log-time">{log.time}</span>
            <span class="log-msg">{log.message}</span>
          </div>
        {/each}
      {:else if selectedTab === 'problems'}
        <div class="empty-state typo-caption">No problems detected</div>
      {:else if selectedTab === 'terminal'}
        <TerminalPanel />
      {:else}
        <div class="empty-state typo-caption">No output</div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .bottom-panel {
    display: flex;
    flex-direction: column;
    height: 160px;
    background: var(--color-surface-dark-soft);
    border-top: 1px solid var(--color-surface-dark-border);
    flex-shrink: 0;
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 var(--spacing-3);
    background: var(--color-surface-dark-soft);
    border-bottom: 1px solid var(--color-surface-dark-border);
    height: 30px;
  }

  .header-tabs {
    display: flex;
    gap: 0;
    height: 100%;
  }

  .header-tab {
    padding: 0 14px;
    border: none;
    background: none;
    color: var(--color-muted);
    cursor: pointer;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    border-bottom: 2px solid transparent;
  }

  .header-tab.active {
    color: var(--color-on-dark);
    border-bottom-color: var(--color-primary);
  }

  .header-tab:hover {
    color: var(--color-on-dark-soft);
  }

  .close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    border: none;
    background: none;
    color: var(--color-muted);
    cursor: pointer;
    border-radius: var(--radius-xs);
  }

  .close-btn:hover {
    background: var(--color-surface-dark-elevated);
    color: var(--color-on-dark);
  }

  .panel-body {
    flex: 1;
    overflow-y: auto;
    padding: var(--spacing-2);
  }

  .log-entry {
    display: flex;
    gap: var(--spacing-3);
    padding: 1px 0;
  }

  .log-time {
    color: var(--color-muted-soft);
    flex-shrink: 0;
  }

  .log-msg {
    color: var(--color-on-dark-soft);
  }

  .log-entry::before {
    content: '>';
    margin-right: 4px;
    opacity: 0.5;
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--color-muted-soft);
  }
</style>
