<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { workspaceRoot } from '$lib/stores/workspace';
  import { get } from 'svelte/store';

  let output = $state<string[]>([]);
  let inputValue = $state('');
  let child: import('@tauri-apps/plugin-shell').Child | null = null;
  let outputEl: HTMLDivElement | undefined;

  async function runCommand() {
    const cmd = inputValue.trim();
    if (!cmd) return;
    inputValue = '';
    output = [...output, `$ ${cmd}`];

    try {
      const { Command } = await import('@tauri-apps/plugin-shell');
      const cwd = get(workspaceRoot) ?? undefined;
      const shell = navigator.userAgent.includes('Win')
        ? ['cmd', '/C', cmd]
        : ['sh', '-c', cmd];
      const command = Command.create(shell[0], shell.slice(1), { cwd });

      command.stdout.on('data', (line: string) => {
        output = [...output, line];
        scrollToBottom();
      });
      command.stderr.on('data', (line: string) => {
        output = [...output, `[err] ${line}`];
        scrollToBottom();
      });
      command.on('close', ({ code }) => {
        output = [...output, `[exit ${code}]`];
        child = null;
      });

      child = await command.spawn();
    } catch (e) {
      output = [...output, `Error: ${e}`];
    }
  }

  function scrollToBottom() {
    setTimeout(() => outputEl?.scrollTo({ top: outputEl.scrollHeight }), 10);
  }

  function clearOutput() {
    output = [];
  }

  onDestroy(async () => {
    await child?.kill();
  });
</script>

<div class="terminal-panel">
  <div class="term-toolbar">
    <span class="term-title typo-overline">TERMINAL</span>
    <button class="clear-btn typo-caption" onclick={clearOutput}>Clear</button>
  </div>
  <div class="term-output" bind:this={outputEl}>
    {#each output as line, i (i)}
      <div class="term-line typo-mono" class:command={line.startsWith('$')}>
        {line}
      </div>
    {/each}
  </div>
  <div class="term-input-row">
    <span class="prompt typo-mono">$</span>
    <input
      class="term-input typo-mono"
      type="text"
      placeholder="Enter command..."
      bind:value={inputValue}
      onkeydown={(e) => e.key === 'Enter' && runCommand()}
    />
  </div>
</div>

<style>
  .terminal-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--color-surface-dark);
  }
  .term-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 4px var(--spacing-3);
    border-bottom: 1px solid var(--color-surface-dark-border);
  }
  .term-title {
    color: var(--color-muted);
  }
  .clear-btn {
    background: none;
    border: none;
    color: var(--color-muted);
    cursor: pointer;
  }
  .clear-btn:hover {
    color: var(--color-on-dark);
  }
  .term-output {
    flex: 1;
    overflow-y: auto;
    padding: var(--spacing-2) var(--spacing-3);
  }
  .term-line {
    font-size: 12px;
    color: var(--color-on-dark-soft);
    white-space: pre-wrap;
    word-break: break-all;
    line-height: 1.5;
  }
  .term-line.command {
    color: var(--color-primary);
  }
  .term-input-row {
    display: flex;
    align-items: center;
    gap: var(--spacing-2);
    padding: var(--spacing-1) var(--spacing-3);
    border-top: 1px solid var(--color-surface-dark-border);
  }
  .prompt {
    color: var(--color-primary);
  }
  .term-input {
    flex: 1;
    background: none;
    border: none;
    outline: none;
    color: var(--color-on-dark);
    font-size: 13px;
  }
</style>
