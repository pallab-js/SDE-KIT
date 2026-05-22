<script lang="ts">
  import {
    openTabs,
    activeTabId,
    closeTab,
    openTab,
  } from '$lib/stores/workspace';
  import { splitMode, splitRatio, dirtyPaths } from '$lib/stores/editor';
  import CodeEditor from './CodeEditor.svelte';
  import type { Tab } from '$lib/types';

  let activeId = $state<string | null>(null);

  $effect(() => {
    activeTabId.subscribe((v) => (activeId = v));
  });

  function onMiddleClick(e: MouseEvent, id: string) {
    if (e.button === 1) closeTab(id);
  }

  function onTabKeydown(e: KeyboardEvent, id: string) {
    if (e.key === 'Enter' || e.key === ' ') activeTabId.set(id);
    else if (e.key === 'Delete' || e.key === 'Backspace') closeTab(id);
  }

  function openFileTab() {
    openTab({
      id: `file:new-${Date.now()}`,
      title: 'Untitled',
      icon: '📄',
      closable: true,
    });
  }

  function isFileTab(tab: Tab): string | null {
    if (tab.id.startsWith('file:')) return tab.id.slice(5);
    return null;
  }
</script>

<main class="main-content">
  {#if $openTabs.length > 0}
    <div class="tab-bar" role="tablist">
      {#each $openTabs as tab (tab.id)}
        <button
          class="tab typo-caption"
          class:active={activeId === tab.id}
          class:dirty={tab.dirty}
          role="tab"
          aria-selected={activeId === tab.id}
          onclick={() => activeTabId.set(tab.id)}
          onmousedown={(e) => onMiddleClick(e, tab.id)}
          onkeydown={(e) => onTabKeydown(e, tab.id)}
        >
          <span class="tab-icon">{tab.icon ?? '📄'}</span>
          <span class="tab-title">{tab.title}</span>
          {#if tab.filePath && $dirtyPaths.has(tab.filePath)}<span
              class="tab-dirty">●</span
            >{/if}
          {#if tab.closable}
            <span
              class="tab-close"
              role="button"
              tabindex="-1"
              onclick={(e) => {
                e.stopPropagation();
                closeTab(tab.id);
              }}
              onkeydown={(e) => {
                if (e.key === 'Enter') closeTab(tab.id);
              }}>×</span
            >
          {/if}
        </button>
      {/each}
      <div class="tab-spacer"></div>
      <button
        class="split-btn typo-body"
        onclick={() => splitMode.update((v) => !v)}
        title="Toggle Split"
      >
        {$splitMode ? '◧' : '◫'}
      </button>
    </div>

    <div class="tab-content" role="tabpanel">
      {#if activeId?.startsWith('file:')}
        {#if $splitMode}
          {@const files = $openTabs.filter(
            (t) => t.id.startsWith('file:') && t.id !== activeId
          )}
          <div class="split-container">
            <div class="split-pane" style="flex: {$splitRatio}">
              <CodeEditor path={activeId.slice(5)} />
            </div>
            <div
              class="split-divider"
              aria-hidden="true"
              onmousedown={(e) => {
                const start = e.clientX;
                const startRatio = $splitRatio;
                function onMove(ev: MouseEvent) {
                  const rect = document
                    .querySelector('.split-container')
                    ?.getBoundingClientRect();
                  if (rect)
                    splitRatio.set(
                      Math.max(
                        0.2,
                        Math.min(
                          0.8,
                          startRatio + (ev.clientX - start) / rect.width
                        )
                      )
                    );
                }
                function onUp() {
                  document.removeEventListener('mousemove', onMove);
                  document.removeEventListener('mouseup', onUp);
                }
                document.addEventListener('mousemove', onMove);
                document.addEventListener('mouseup', onUp);
              }}
            ></div>
            <div class="split-pane" style="flex: {1 - $splitRatio}">
              {#if files.length > 0}
                <CodeEditor path={files[0].id.slice(5)} />
              {:else}
                <div class="placeholder-editor">
                  <span class="editor-text">Open another file to split</span>
                </div>
              {/if}
            </div>
          </div>
        {:else}
          <CodeEditor path={activeId.slice(5)} />
        {/if}
      {:else}
        <div class="placeholder-editor">
          <span class="editor-icon">📄</span>
          <span class="editor-text typo-body">Select a file to edit</span>
        </div>
      {/if}
    </div>
  {:else}
    <div class="welcome">
      <div class="welcome-content">
        <h1 class="welcome-title typo-display-xl">SDE Kit</h1>
        <p class="welcome-subtitle typo-body">
          Local-First SDLC Desktop Platform
        </p>
        <div class="welcome-actions">
          <button class="action-card" onclick={openFileTab}>
            <span class="action-icon typo-title">📄</span>
            <span class="action-text typo-body">New File</span>
            <span class="action-hint typo-mono">Cmd+T</span>
          </button>
          <div
            class="action-card"
            role="button"
            tabindex="0"
            onclick={() => activeTabId.set('quick-open')}
            onkeydown={(e) => {
              if (e.key === 'Enter') activeTabId.set('quick-open');
            }}
          >
            <span class="action-icon typo-title">⌘</span>
            <span class="action-text typo-body">Command Palette</span>
            <span class="action-hint typo-mono">Cmd+P</span>
          </div>
        </div>
      </div>
    </div>
  {/if}
</main>

<style>
  .main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background: var(--color-surface-dark);
  }

  .tab-bar {
    display: flex;
    align-items: center;
    height: 34px;
    background: var(--color-surface-dark-soft);
    border-bottom: 1px solid var(--color-surface-dark-border);
    overflow-x: auto;
    gap: 1px;
    padding: 0;
    flex-shrink: 0;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 5px 12px;
    border: none;
    background: var(--color-surface-dark-soft);
    color: var(--color-on-dark-soft);
    cursor: pointer;
    white-space: nowrap;
    border-right: 1px solid var(--color-surface-dark-border);
    flex-shrink: 0;
  }

  .tab.active {
    background: var(--color-surface-dark);
    color: var(--color-on-dark);
    border-bottom: 1px solid var(--color-primary);
  }

  .tab:hover:not(.active) {
    background: var(--color-surface-dark-elevated);
  }
  .tab-icon {
    flex-shrink: 0;
  }

  .tab-close {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    border: none;
    background: none;
    color: var(--color-muted);
    cursor: pointer;
    border-radius: 2px;
    padding: 0;
    line-height: 1;
  }

  .tab-close:hover {
    background: var(--color-surface-dark-elevated);
    color: var(--color-on-dark);
  }
  .tab-dirty {
    color: var(--color-warning);
    font-size: 12px;
    margin-left: -2px;
  }
  .tab-spacer {
    flex: 1;
  }

  .split-btn {
    padding: 2px 8px;
    border: none;
    background: none;
    color: var(--color-muted);
    cursor: pointer;
  }
  .split-btn:hover {
    color: var(--color-on-dark);
  }

  .tab-content {
    flex: 1;
    overflow: hidden;
    position: relative;
  }

  .split-container {
    display: flex;
    height: 100%;
    width: 100%;
  }

  .split-pane {
    overflow: hidden;
    min-width: 100px;
  }

  .split-divider {
    width: 4px;
    cursor: col-resize;
    background: var(--color-surface-dark-soft);
    flex-shrink: 0;
  }

  .split-divider:hover {
    background: var(--color-primary);
    opacity: 0.5;
  }

  .placeholder-editor {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    gap: var(--spacing-3);
    color: var(--color-muted-soft);
  }

  .editor-icon {
    font-size: 24px;
    opacity: 0.2;
  }

  .welcome {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
  }

  .welcome-content {
    text-align: center;
    max-width: 400px;
  }
  .welcome-title {
    font-family: var(--font-display);
    letter-spacing: -0.3px;
    color: var(--color-on-dark);
    margin: 0 0 var(--spacing-2);
  }
  .welcome-subtitle {
    color: var(--color-on-dark-soft);
    margin: 0 0 var(--spacing-8);
  }
  .welcome-actions {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-2);
  }

  .action-card {
    display: flex;
    align-items: center;
    gap: var(--spacing-3);
    padding: var(--spacing-3) var(--spacing-4);
    background: var(--color-surface-dark-soft);
    border: 1px solid var(--color-surface-dark-border);
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: border-color 0.15s;
    width: 100%;
    text-align: left;
    font-family: inherit;
    font-size: inherit;
    color: inherit;
  }
  .action-card:hover {
    border-color: var(--color-primary);
  }
  .action-icon {
    width: 24px;
    text-align: center;
    flex-shrink: 0;
  }
  .action-text {
    flex: 1;
    color: var(--color-on-dark);
  }
  .action-hint {
    color: var(--color-muted);
    font-family: var(--font-mono);
  }
</style>
