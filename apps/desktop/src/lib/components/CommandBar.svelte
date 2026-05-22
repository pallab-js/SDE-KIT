<script lang="ts">
  import { toggleBottom, toggleSidebar } from '$lib/stores/workspace';
  import { theme } from '$lib/stores/theme';

  let { onPalette }: { onPalette: () => void } = $props();
</script>

<header class="command-bar typo-caption">
  <div class="bar-left">
    <button
      class="bar-btn typo-caption"
      onclick={toggleSidebar}
      title="Toggle Sidebar (Cmd+B)"
    >
      ☰
    </button>
    <span class="brand typo-body">SDE Kit</span>
  </div>
  <div class="bar-center">
    <button class="palette-trigger typo-caption" onclick={onPalette}>
      <span class="palette-icon">⌘</span>
      <span class="palette-text">Quick Actions...</span>
      <span class="palette-hint">Cmd+P</span>
    </button>
  </div>
  <div class="bar-right">
    <button
      class="bar-btn typo-caption"
      title="Toggle Console (Cmd+J)"
      onclick={toggleBottom}
    >
      Console
    </button>
    <button
      class="bar-btn typo-caption"
      title="Toggle Theme (Cmd+Shift+T)"
      onclick={() => theme.toggle()}
    >
      ☀
    </button>
  </div>
</header>

<style>
  .command-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 36px;
    padding: 0 var(--spacing-3);
    background: var(--color-surface-dark-elevated);
    border-bottom: 1px solid var(--color-surface-dark-border);
    -webkit-app-region: drag;
  }

  .bar-left,
  .bar-center,
  .bar-right {
    display: flex;
    align-items: center;
    gap: var(--spacing-1);
    -webkit-app-region: no-drag;
  }

  .bar-center {
    flex: 1;
    justify-content: center;
    max-width: 400px;
  }

  .palette-trigger {
    display: flex;
    align-items: center;
    gap: var(--spacing-2);
    padding: 4px 12px;
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark);
    color: var(--color-on-dark-soft);
    cursor: pointer;
    border-radius: var(--radius-sm);
    width: 100%;
    opacity: 0.8;
  }

  .palette-trigger:hover {
    border-color: var(--color-muted);
    color: var(--color-on-dark);
    opacity: 1;
  }

  .palette-icon {
    font-size: 14px;
  }

  .palette-text {
    flex: 1;
    text-align: left;
  }

  .palette-hint {
    color: var(--color-muted-soft);
    font-family: var(--font-mono);
  }

  .bar-btn {
    padding: 4px 10px;
    border: none;
    background: none;
    color: var(--color-on-dark-soft);
    cursor: pointer;
    border-radius: var(--radius-sm);
  }

  .bar-btn:hover {
    background: var(--color-surface-dark);
    color: var(--color-on-dark);
  }

  .brand {
    font-family: var(--font-display);
    font-size: 16px;
    color: var(--color-on-dark);
    margin-left: var(--spacing-1);
    letter-spacing: -0.3px;
    -webkit-app-region: no-drag;
  }
</style>
