<script lang="ts">
  import { onMount } from 'svelte';
  import { theme } from '$lib/stores/theme';
  import { undoManager } from '$lib/services/undoManager';
  import ErrorBoundary from '$lib/components/ErrorBoundary.svelte';
  import { isLoading, globalError } from '$lib/stores/app';
  import '../app.css';

  let { children } = $props();

  onMount(() => {
    theme.init();

    function handleKey(e: KeyboardEvent) {
      const meta = e.metaKey || e.ctrlKey;
      if (meta && e.shiftKey && e.key === 'z') {
        e.preventDefault();
        undoManager.redo();
      } else if (meta && e.key === 'z') {
        e.preventDefault();
        undoManager.undo();
      }
    }

    document.addEventListener('keydown', handleKey);
    return () => document.removeEventListener('keydown', handleKey);
  });
</script>

<ErrorBoundary>
  <div class="app-shell">
    {@render children()}

    {#if $isLoading}
      <div class="loading-overlay">
        <div class="spinner"></div>
        <span class="loading-text typo-body">Processing Offline Request...</span
        >
      </div>
    {/if}

    {#if $globalError}
      <div class="toast-error">
        <div class="toast-content">
          <span class="toast-icon">⚠️</span>
          <div class="toast-details">
            <span class="toast-title typo-caption">{$globalError.code}</span>
            <span class="toast-message">{$globalError.message}</span>
          </div>
        </div>
        <button class="toast-close" onclick={() => globalError.set(null)}
          >×</button
        >
      </div>
    {/if}
  </div>
</ErrorBoundary>

<style>
  .app-shell {
    display: contents;
  }

  .loading-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(10, 10, 12, 0.6);
    backdrop-filter: blur(4px);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    z-index: 9999;
    gap: 16px;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid rgba(255, 255, 255, 0.1);
    border-top: 3px solid var(--color-primary, #6366f1);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }

  .loading-text {
    color: var(--color-on-dark, #ffffff);
    font-size: 14px;
    font-weight: 500;
  }

  .toast-error {
    position: fixed;
    bottom: 24px;
    right: 24px;
    background: var(--color-surface-dark-elevated, #1e1e24);
    border: 1px solid var(--color-error, #ef4444);
    border-radius: var(--radius-sm, 8px);
    padding: 12px 16px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    z-index: 10000;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
    max-width: 380px;
    animation: slideIn 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes slideIn {
    from {
      transform: translateY(20px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  .toast-content {
    display: flex;
    align-items: flex-start;
    gap: 12px;
  }

  .toast-icon {
    font-size: 18px;
    margin-top: 2px;
  }

  .toast-details {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .toast-title {
    color: var(--color-error, #ef4444);
    font-weight: 600;
    font-size: 12px;
  }

  .toast-message {
    color: var(--color-on-dark-soft, #a1a1aa);
    font-size: 12px;
  }

  .toast-close {
    background: none;
    border: none;
    color: var(--color-muted-soft, #71717a);
    font-size: 20px;
    cursor: pointer;
    line-height: 1;
    padding: 0 4px;
  }

  .toast-close:hover {
    color: var(--color-on-dark, #ffffff);
  }
</style>
