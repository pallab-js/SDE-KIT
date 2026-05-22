<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import {
    EditorView,
    keymap,
    lineNumbers,
    drawSelection,
    rectangularSelection,
  } from '@codemirror/view';
  import { EditorState } from '@codemirror/state';
  import { defaultKeymap, history } from '@codemirror/commands';
  import { oneDark } from '@codemirror/theme-one-dark';
  import {
    syntaxHighlighting,
    defaultHighlightStyle,
  } from '@codemirror/language';

  let {
    original = '',
    modified = '',
    language = 'typescript',
  }: { original?: string; modified?: string; language?: string } = $props();

  let container: HTMLDivElement;
  let view: EditorView;

  onMount(() => {
    if (!container) return;

    const startState = EditorState.create({
      doc: `--- Original\n${original}\n\n+++ Modified\n${modified}`,
      extensions: [
        lineNumbers(),
        drawSelection(),
        rectangularSelection(),
        oneDark,
        EditorView.editable.of(false),
        EditorView.theme({
          '&': { height: '100%', fontSize: '13px' },
          '.cm-scroller': {
            fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
          },
        }),
        keymap.of([...defaultKeymap]),
        history(),
      ],
    });

    view = new EditorView({
      state: startState,
      parent: container,
    });
  });

  onDestroy(() => {
    view?.destroy();
  });
</script>

<div class="diff-viewer">
  <div class="diff-header typo-overline">
    <span>File Diff</span>
    <span class="typo-small">{language}</span>
  </div>
  <div bind:this={container} class="diff-content"></div>
</div>

<style>
  .diff-viewer {
    display: flex;
    flex-direction: column;
    height: 100%;
    border: 1px solid var(--color-surface-dark-border);
    border-radius: var(--radius-md);
    overflow: hidden;
  }

  .diff-header {
    display: flex;
    justify-content: space-between;
    padding: 6px var(--spacing-3);
    background: var(--color-surface-dark-soft);
    border-bottom: 1px solid var(--color-surface-dark-border);
  }

  .diff-content {
    flex: 1;
    overflow: hidden;
  }

  .diff-content :global(.cm-editor) {
    height: 100%;
  }
  .diff-content :global(.cm-scroller) {
    overflow: auto;
  }
</style>
