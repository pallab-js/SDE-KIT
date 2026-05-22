import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
  plugins: [sveltekit()],
  test: {
    environment: 'node',
    include: ['src/**/*.{test,spec}.{js,ts}'],
    setupFiles: ['src/lib/tests/setup.ts'],
    coverage: {
      provider: 'v8',
      include: ['src/lib/services/**', 'src/lib/stores/**', 'src/lib/utils/**'],
      thresholds: { lines: 80 },
    },
  },
});
