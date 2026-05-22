<script lang="ts">
  import { notify } from '$lib/stores/notifications';

  let { children } = $props();

  let error = $state<Error | null>(null);
  let info = $state<string>('');

  function handleError(e: ErrorEvent) {
    e.preventDefault();
    const err = e.error instanceof Error ? e.error : new Error(e.message);
    error = err;
    info = `at ${e.filename ?? '?'}:${e.lineno ?? 0}:${e.colno ?? 0}`;
    notify(`Component error: ${err.message}`, 'error');
  }

  function handleRejection(e: PromiseRejectionEvent) {
    e.preventDefault();
    error = e.reason instanceof Error ? e.reason : new Error(String(e.reason));
    info = 'Unhandled promise rejection';
    notify(`Async error: ${error.message}`, 'error');
  }

  function reset() {
    error = null;
    info = '';
  }

  $effect(() => {
    window.addEventListener('error', handleError);
    window.addEventListener('unhandledrejection', handleRejection);
    return () => {
      window.removeEventListener('error', handleError);
      window.removeEventListener('unhandledrejection', handleRejection);
    };
  });
</script>

{#if error}
  <div class="error-boundary" role="alert">
    <div class="error-icon">⚠</div>
    <h2 class="typo-title">Something went wrong</h2>
    <p class="typo-body error-message">{error.message}</p>
    {#if info}
      <p class="typo-small error-info">{info}</p>
    {/if}
    <div class="error-actions">
      <button class="action-btn typo-caption" onclick={reset}>
        Try Again
      </button>
      <button
        class="action-btn typo-caption"
        onclick={() => window.location.reload()}
      >
        Reload App
      </button>
    </div>
  </div>
{:else}
  {@render children()}
{/if}

<style>
  .error-boundary {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    padding: 32px;
    text-align: center;
    background: var(--color-surface-dark);
    color: var(--color-on-dark);
  }

  .error-icon {
    font-size: 48px;
    margin-bottom: 16px;
    opacity: 0.5;
  }

  .error-message {
    color: var(--color-error);
    margin: 8px 0;
    max-width: 500px;
    word-break: break-word;
  }

  .error-info {
    color: var(--color-muted);
    margin: 4px 0 16px;
  }

  .error-actions {
    display: flex;
    gap: 8px;
    margin-top: 16px;
  }

  .action-btn {
    padding: 8px 20px;
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark-elevated);
    color: var(--color-on-dark);
    border-radius: var(--radius-sm);
    cursor: pointer;
  }

  .action-btn:hover {
    background: var(--color-surface-dark-soft);
    border-color: var(--color-muted);
  }
</style>
