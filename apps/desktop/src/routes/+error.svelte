<script lang="ts">
  import { page } from '$app/stores';
</script>

<div class="error-page">
  <h1 class="typo-display-xl">{$page.status}</h1>
  <p class="typo-body">
    {$page.error?.message ?? 'An unexpected error occurred'}
  </p>
  <a class="typo-body" href="/">Go Home</a>
</div>

<style>
  .error-page {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    gap: var(--spacing-4);
    background: var(--color-surface-dark);
    color: var(--color-on-dark);
  }

  h1 {
    font-family: var(--font-display);
    letter-spacing: -0.3px;
    margin: 0;
    color: var(--color-primary);
  }

  p {
    color: var(--color-on-dark-soft);
  }

  a {
    color: var(--color-primary);
    text-decoration: none;
  }

  a:hover {
    color: var(--color-primary-active);
  }
</style>
