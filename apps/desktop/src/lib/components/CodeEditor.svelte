<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import {
    EditorView,
    keymap,
    lineNumbers,
    highlightActiveLine,
    highlightSpecialChars,
    drawSelection,
    rectangularSelection,
  } from '@codemirror/view';
  import { EditorState, Compartment } from '@codemirror/state';
  import {
    defaultKeymap,
    history,
    historyKeymap,
    indentWithTab,
  } from '@codemirror/commands';
  import { searchKeymap, highlightSelectionMatches } from '@codemirror/search';
  import { oneDark } from '@codemirror/theme-one-dark';
  import { readFile, writeFile } from '$lib/services/fs';
  import { setFileContent, updateContent, markDirty } from '$lib/stores/editor';
  import { writable } from 'svelte/store';
  const saveStatus = writable<'saved' | 'saving' | 'error' | 'dirty'>('saved');

  let { path: filePath }: { path: string } = $props();

  let container: HTMLDivElement;
  let view: EditorView;
  let langCompartment = new Compartment();
  let saveTimer: ReturnType<typeof setTimeout> | undefined;

  const langLoaders: Record<
    string,
    () => Promise<import('@codemirror/language').LanguageSupport>
  > = {
    '.js': () =>
      import('@codemirror/lang-javascript').then((m) => m.javascript()),
    '.jsx': () =>
      import('@codemirror/lang-javascript').then((m) =>
        m.javascript({ jsx: true })
      ),
    '.ts': () =>
      import('@codemirror/lang-javascript').then((m) =>
        m.javascript({ typescript: true })
      ),
    '.tsx': () =>
      import('@codemirror/lang-javascript').then((m) =>
        m.javascript({ jsx: true, typescript: true })
      ),
    '.css': () => import('@codemirror/lang-css').then((m) => m.css()),
    '.html': () => import('@codemirror/lang-html').then((m) => m.html()),
    '.svelte': () => import('@codemirror/lang-html').then((m) => m.html()),
    '.json': () => import('@codemirror/lang-json').then((m) => m.json()),
    '.md': () => import('@codemirror/lang-markdown').then((m) => m.markdown()),
    '.mdx': () => import('@codemirror/lang-markdown').then((m) => m.markdown()),
    '.rs': () => import('@codemirror/lang-rust').then((m) => m.rust()),
    '.py': () => import('@codemirror/lang-python').then((m) => m.python()),
    '.yaml': () => import('@codemirror/lang-yaml').then((m) => m.yaml()),
    '.yml': () => import('@codemirror/lang-yaml').then((m) => m.yaml()),
    // .toml: unsupported by CodeMirror — falls back to plain text
    '.xml': () => import('@codemirror/lang-xml').then((m) => m.xml()),
    '.sql': () => import('@codemirror/lang-sql').then((m) => m.sql()),
    '.java': () => import('@codemirror/lang-java').then((m) => m.java()),
    '.cpp': () => import('@codemirror/lang-cpp').then((m) => m.cpp()),
    '.c': () => import('@codemirror/lang-cpp').then((m) => m.cpp()),
    '.go': () => import('@codemirror/lang-go').then((m) => m.go()),
    '.php': () => import('@codemirror/lang-php').then((m) => m.php()),
  };

  async function loadLang(path: string) {
    const ext = '.' + path.split('.').pop()?.toLowerCase();
    const loader = langLoaders[ext];
    if (loader) {
      try {
        return await loader();
      } catch {}
    }
    return [] as any;
  }

  onMount(async () => {
    try {
      const content = await readFile(filePath);
      setFileContent(filePath, content);
      const lang = await loadLang(filePath);

      const startState = EditorState.create({
        doc: content,
        extensions: [
          lineNumbers(),
          highlightActiveLine(),
          highlightSpecialChars(),
          drawSelection(),
          rectangularSelection(),
          highlightSelectionMatches(),
          oneDark,
          EditorView.theme({
            '&': { height: '100%', fontSize: '13px' },
            '.cm-scroller': {
              fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
            },
          }),
          langCompartment.of(lang),
          keymap.of([
            ...defaultKeymap,
            ...historyKeymap,
            ...searchKeymap,
            indentWithTab,
            {
              key: 'Mod-s',
              run: () => {
                save();
                return true;
              },
            },
          ]),
          history(),
          EditorView.updateListener.of((update) => {
            if (update.docChanged) {
              saveStatus.set('dirty');
              const newContent = update.state.doc.toString();
              updateContent(filePath, newContent);
              debounceSave(newContent);
            }
          }),
        ],
      });

      view = new EditorView({
        state: startState,
        parent: container,
      });
    } catch {
      container.textContent = 'Failed to load file';
    }
  });

  function debounceSave(content: string) {
    clearTimeout(saveTimer);
    saveTimer = setTimeout(() => save(content), 1000);
  }

  async function save(content?: string) {
    saveStatus.set('saving');
    try {
      const current = content ?? view?.state.doc.toString();
      if (current !== undefined) {
        await writeFile(filePath, current);
        markDirty(filePath, false);
        saveStatus.set('saved');
      }
    } catch (e) {
      saveStatus.set('error');
      console.error('Save failed:', e);
    }
  }

  onDestroy(() => {
    clearTimeout(saveTimer);
    view?.destroy();
  });
</script>

<div class="editor-wrapper" style="position: relative; height: 100%;">
  <div bind:this={container} class="editor-container"></div>
  <div class="editor-status" class:error={$saveStatus === 'error'}>
    {#if $saveStatus === 'saving'}⟳ Saving{:else if $saveStatus === 'error'}⚠
      Save failed{:else if $saveStatus === 'dirty'}● Unsaved{:else}✓{/if}
  </div>
</div>

<style>
  .editor-container {
    height: 100%;
    overflow: hidden;
  }

  .editor-container :global(.cm-editor) {
    height: 100%;
  }

  .editor-container :global(.cm-scroller) {
    overflow: auto;
  }

  .editor-status {
    position: absolute;
    bottom: 4px;
    right: 8px;
    font-size: 11px;
    color: var(--color-muted);
    pointer-events: none;
    font-family: var(--font-mono);
  }
  .editor-status.error {
    color: var(--color-error);
  }
</style>
