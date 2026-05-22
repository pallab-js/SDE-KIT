<script lang="ts">
  import type { FileEntry } from '$lib/types';
  import FileTreeNode from './FileTreeNode.svelte';

  let { root, pageSize = 50 }: { root: string; pageSize?: number } = $props();

  let rootEntry = $derived<FileEntry>({
    name: root.split('/').pop() || root,
    path: root,
    is_dir: true,
    size: 0,
    modified: '',
  });
</script>

<FileTreeNode entry={rootEntry} {pageSize} />
