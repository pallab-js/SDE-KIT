<script lang="ts">
  import { getNote, createNote, updateNote } from '$lib/services/api';

  const NOTE_ID = 'scratch';
  let content = $state('');
  let status = $state<'saved' | 'saving' | 'unsaved'>('saved');
  let saveTimer: ReturnType<typeof setTimeout> | undefined;

  async function load() {
    try {
      const note = await getNote(NOTE_ID);
      if (note) {
        content = note.content;
      } else {
        // Try creating the scratch pad note in the dedicated notes table if it does not exist yet
        await createNote('Scratch Pad', '', undefined);
        // In our backend create_note, the ID is a generated UUID. If the scratch pad needs a fixed ID 'scratch',
        // we can query and if it doesn't exist, create it. Since our backend uses Uuid for create_note,
        // we can handle loading it or creating a new note.
        // Let's make sure the notes table has a way to handle this.
      }
    } catch {}
  }

  // We can update the load logic to list notes, and if there are no notes, create one.
  // Or we can just use a fixed ID. Since the notes table primary key is text, let's look at get_notes.
  // If we get the first note, or search for a note with title 'Scratch Pad', we can load it.
  // Let's write a robust scratch pad loader:
  let activeNoteId: string | null = null;

  async function loadScratchNote() {
    try {
      const notes = await getNotes(undefined, 1);
      if (notes && notes.length > 0) {
        const scratchNote = notes[0];
        if (scratchNote) {
          activeNoteId = scratchNote.id;
          content = scratchNote.content;
        }
      } else {
        const newNote = await createNote('Scratch Pad', '');
        activeNoteId = newNote.id;
        content = '';
      }
    } catch (err) {
      console.error('Failed to load scratch note', err);
    }
  }

  import { getNotes } from '$lib/services/api';

  async function save() {
    if (!activeNoteId) return;
    status = 'saving';
    try {
      await updateNote(activeNoteId, 'Scratch Pad', content, null);
      status = 'saved';
    } catch {
      status = 'unsaved';
    }
  }

  function onInput() {
    status = 'unsaved';
    clearTimeout(saveTimer);
    saveTimer = setTimeout(save, 800);
  }

  loadScratchNote();
</script>

<div class="notes-panel">
  <div class="notes-header">
    <span class="notes-title typo-overline">SCRATCH PAD</span>
    <span class="notes-status typo-small"
      >{status === 'saving' ? '⟳ Saving' : status === 'saved' ? '✓' : '●'}</span
    >
  </div>
  <textarea
    class="notes-body typo-body"
    placeholder="Write notes, ideas, snippets..."
    bind:value={content}
    oninput={onInput}
  ></textarea>
</div>

<style>
  .notes-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  .notes-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 6px var(--spacing-3);
    border-bottom: 1px solid var(--color-surface-dark-border);
  }
  .notes-title {
    color: var(--color-muted);
  }
  .notes-status {
    color: var(--color-muted);
  }
  .notes-body {
    flex: 1;
    background: var(--color-surface-dark);
    color: var(--color-on-dark);
    border: none;
    outline: none;
    resize: none;
    padding: var(--spacing-3);
    font-family: var(--font-mono);
    font-size: 13px;
    line-height: 1.6;
  }
</style>
