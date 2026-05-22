<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import {
    getGraphSnapshot,
    computeGraphLayout,
    addGraphNode,
    removeGraphNode,
    addGraphEdge,
    removeGraphEdge,
    clearGraph,
  } from '$lib/services/graph';
  import { invoke } from '@tauri-apps/api/core';
  import type { GraphNode, GraphEdge, NodePosition } from '$lib/services/graph';

  let nodes = $state<GraphNode[]>([]);
  let edges = $state<GraphEdge[]>([]);
  let positions = $state<Map<string, { x: number; y: number }>>(new Map());
  let loading = $state(true);
  let selectedId = $state<string | null>(null);
  let showNewNode = $state(false);
  let newNodeLabel = $state('');
  let newNodeType = $state('default');
  let showNewEdge = $state(false);
  let newEdgeSource = $state('');
  let newEdgeTarget = $state('');
  let newEdgeType = $state('related');

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null = null;
  let animFrame: number | null = null;
  let panX = $state(0);
  let panY = $state(0);
  let zoom = $state(1);
  let isPanning = false;
  let panStartX = 0;
  let panStartY = 0;
  let dragNode: string | null = null;

  const NODE_RADIUS = 20;
  const COLORS: Record<string, string> = {
    default: '#cc785c',
    project: '#5db872',
    task: '#d4a017',
    file: '#5db8a6',
    milestone: '#e8a55a',
  };

  function getColor(type: string): string {
    return COLORS[type] ?? COLORS.default;
  }

  function toScreen(x: number, y: number): { sx: number; sy: number } {
    return { sx: x * zoom + panX, sy: y * zoom + panY };
  }

  function toWorld(sx: number, sy: number): { x: number; y: number } {
    return { x: (sx - panX) / zoom, y: (sy - panY) / zoom };
  }

  async function load() {
    loading = true;
    try {
      const snap = await getGraphSnapshot();
      const prevPositions = new Map(positions);
      nodes = snap.nodes;
      edges = snap.edges;

      const unpositioned = nodes.filter((n) => !prevPositions.has(n.id));
      if (unpositioned.length > 0) {
        const w = canvas?.width || 600;
        const h = canvas?.height || 400;
        const pos = await computeGraphLayout(w, h);
        const newPos = new Map(
          pos.map((p: NodePosition) => [p.id, { x: p.x, y: p.y }])
        );
        positions = new Map([...newPos, ...prevPositions]);
      } else {
        positions = prevPositions;
      }
    } catch {
      nodes = [];
      edges = [];
    }
    loading = false;
    draw();
  }

  function draw() {
    if (!canvas || !ctx) return;
    const rect = canvas.getBoundingClientRect();
    canvas.width = rect.width * devicePixelRatio;
    canvas.height = rect.height * devicePixelRatio;
    ctx.scale(devicePixelRatio, devicePixelRatio);
    const w = rect.width;
    const h = rect.height;

    ctx.clearRect(0, 0, w, h);
    ctx.fillStyle = '#181715';
    ctx.fillRect(0, 0, w, h);

    const margin = 50;
    const topLeft = toWorld(-margin, -margin);
    const botRight = toWorld(w + margin, h + margin);
    const inView = (x: number, y: number) =>
      x >= topLeft.x && x <= botRight.x && y >= topLeft.y && y <= botRight.y;

    const visibleNodes = new Set<string>();
    for (const node of nodes) {
      const pos = positions.get(node.id);
      if (!pos) continue;
      if (inView(pos.x, pos.y)) visibleNodes.add(node.id);
    }

    for (const edge of edges) {
      const src = positions.get(edge.source_id);
      const tgt = positions.get(edge.target_id);
      if (!src || !tgt) continue;
      if (
        !visibleNodes.has(edge.source_id) &&
        !visibleNodes.has(edge.target_id)
      )
        continue;
      const p1 = toScreen(src.x, src.y);
      const p2 = toScreen(tgt.x, tgt.y);
      ctx.strokeStyle = '#4d4a45';
      ctx.lineWidth = 1.5;
      ctx.beginPath();
      ctx.moveTo(p1.sx, p1.sy);
      ctx.lineTo(p2.sx, p2.sy);
      ctx.stroke();
    }

    for (const node of nodes) {
      if (!visibleNodes.has(node.id)) continue;
      const pos = positions.get(node.id);
      if (!pos) continue;
      const p = toScreen(pos.x, pos.y);
      const isSelected = node.id === selectedId;
      const color = getColor(node.node_type);

      ctx.beginPath();
      ctx.arc(p.sx, p.sy, NODE_RADIUS * zoom, 0, Math.PI * 2);
      ctx.fillStyle = color;
      ctx.globalAlpha = 0.2;
      ctx.fill();
      ctx.globalAlpha = 1;
      ctx.strokeStyle = isSelected ? '#faf9f5' : color;
      ctx.lineWidth = isSelected ? 2.5 : 1.5;
      ctx.stroke();

      ctx.fillStyle = '#c4c0b8';
      ctx.font = `${Math.max(10, 12 * zoom)}px sans-serif`;
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      const label =
        node.label.length > 12 ? node.label.slice(0, 12) + '…' : node.label;
      ctx.fillText(label, p.sx, p.sy + NODE_RADIUS * zoom + 14 * zoom);
    }
  }

  function onMouseDown(e: MouseEvent) {
    const rect = canvas.getBoundingClientRect();
    const mx = e.clientX - rect.left;
    const my = e.clientY - rect.top;

    for (const node of nodes) {
      const pos = positions.get(node.id);
      if (!pos) continue;
      const p = toScreen(pos.x, pos.y);
      const dx = mx - p.sx;
      const dy = my - p.sy;
      if (dx * dx + dy * dy < (NODE_RADIUS * zoom + 5) ** 2) {
        selectedId = node.id;
        dragNode = node.id;
        draw();
        return;
      }
    }

    selectedId = null;
    isPanning = true;
    panStartX = mx - panX;
    panStartY = my - panY;
    draw();
  }

  let tooltip = $state<{ x: number; y: number; node: GraphNode } | null>(null);

  function onMouseMove(e: MouseEvent) {
    if (dragNode || isPanning) {
      if (dragNode) {
        const rect = canvas.getBoundingClientRect();
        const mx = e.clientX - rect.left;
        const my = e.clientY - rect.top;
        const world = toWorld(mx, my);
        positions.set(dragNode, { x: world.x, y: world.y });
        draw();
      }
      if (isPanning) {
        const rect = canvas.getBoundingClientRect();
        const mx = e.clientX - rect.left;
        const my = e.clientY - rect.top;
        panX = mx - panStartX;
        panY = my - panStartY;
        draw();
      }
      tooltip = null;
      return;
    }
    const rect = canvas.getBoundingClientRect();
    const mx = e.clientX - rect.left;
    const my = e.clientY - rect.top;
    let hovered: typeof tooltip = null;
    for (const node of nodes) {
      const pos = positions.get(node.id);
      if (!pos) continue;
      const p = toScreen(pos.x, pos.y);
      if ((mx - p.sx) ** 2 + (my - p.sy) ** 2 < (NODE_RADIUS * zoom + 5) ** 2) {
        hovered = { x: e.clientX, y: e.clientY, node };
        break;
      }
    }
    tooltip = hovered;
  }

  function onMouseUp() {
    isPanning = false;
    dragNode = null;
  }

  function onWheel(e: WheelEvent) {
    e.preventDefault();
    const delta = e.deltaY > 0 ? 0.9 : 1.1;
    zoom = Math.max(0.2, Math.min(3, zoom * delta));
    draw();
  }

  async function addNode() {
    if (!newNodeLabel.trim()) return;
    try {
      await addGraphNode(newNodeType, newNodeLabel.trim());
      newNodeLabel = '';
      showNewNode = false;
      await load();
    } catch {}
  }

  async function removeSelected() {
    if (!selectedId) return;
    try {
      await removeGraphNode(selectedId);
      selectedId = null;
      await load();
    } catch {}
  }

  async function addEdge() {
    if (!newEdgeSource || !newEdgeTarget) return;
    try {
      await addGraphEdge(newEdgeSource, newEdgeTarget, newEdgeType);
      newEdgeSource = '';
      newEdgeTarget = '';
      showNewEdge = false;
      await load();
    } catch {}
  }

  async function doClear() {
    try {
      await clearGraph();
      await load();
    } catch {}
  }

  async function refresh() {
    await load();
  }

  onMount(() => {
    if (!canvas) return;
    ctx = canvas.getContext('2d');
    load();
  });

  const selectedNode = $derived(nodes.find((n) => n.id === selectedId));

  async function syncFromSdlc() {
    try {
      await invoke('sync_graph_from_sdlc');
      await load();
    } catch (e) {
      console.error('Sync failed:', e);
    }
  }
</script>

<div class="graph-panel">
  <div class="panel-header">
    <div class="header-actions">
      <button
        class="tool-btn typo-caption"
        onclick={() => (showNewNode = !showNewNode)}
        title="Add Node">+N</button
      >
      <button
        class="tool-btn typo-caption"
        onclick={() => (showNewEdge = !showNewEdge)}
        title="Add Edge">+E</button
      >
      <button
        class="tool-btn typo-caption"
        onclick={removeSelected}
        title="Delete Selected (click node)">−</button
      >
      <button
        class="tool-btn typo-caption"
        onclick={refresh}
        title="Refresh Layout">⟳</button
      >
      <button
        class="tool-btn typo-caption"
        onclick={syncFromSdlc}
        title="Sync from Projects/Tasks/Milestones">⟲SDLC</button
      >
      <button class="tool-btn typo-caption" onclick={doClear} title="Clear All"
        >✕</button
      >
    </div>
  </div>

  {#if showNewNode}
    <div class="form-overlay">
      <div class="form-row">
        <input
          type="text"
          class="typo-caption"
          placeholder="Node label"
          bind:value={newNodeLabel}
        />
        <select class="typo-caption" bind:value={newNodeType}>
          <option value="default">Default</option>
          <option value="project">Project</option>
          <option value="task">Task</option>
          <option value="file">File</option>
          <option value="milestone">Milestone</option>
        </select>
        <button class="btn-primary typo-caption" onclick={addNode}>Add</button>
        <button
          class="btn-secondary typo-caption"
          onclick={() => (showNewNode = false)}>X</button
        >
      </div>
    </div>
  {/if}

  {#if showNewEdge}
    <div class="form-overlay">
      <div class="form-row">
        <select class="typo-caption" bind:value={newEdgeSource}>
          <option value="">— Source node —</option>
          {#each nodes as n (n.id)}
            <option value={n.id}>{n.label} ({n.node_type})</option>
          {/each}
        </select>
        <select class="typo-caption" bind:value={newEdgeTarget}>
          <option value="">— Target node —</option>
          {#each nodes as n (n.id)}
            <option value={n.id}>{n.label} ({n.node_type})</option>
          {/each}
        </select>
        <select class="typo-caption" bind:value={newEdgeType}>
          <option value="related">Related</option>
          <option value="depends">Depends On</option>
          <option value="contains">Contains</option>
          <option value="blocks">Blocks</option>
          <option value="implements">Implements</option>
        </select>
        <button class="btn-primary typo-caption" onclick={addEdge}>Add</button>
        <button
          class="btn-secondary typo-caption"
          onclick={() => (showNewEdge = false)}>✕</button
        >
      </div>
    </div>
  {/if}

  <div class="canvas-container">
    {#if loading}
      <div class="loading-overlay typo-body">Loading...</div>
    {/if}
    <canvas
      bind:this={canvas}
      class="graph-canvas"
      onmousedown={onMouseDown}
      onmousemove={onMouseMove}
      onmouseup={onMouseUp}
      onmouseleave={onMouseUp}
      onwheel={onWheel}
    ></canvas>
    {#if selectedNode}
      <div class="node-info typo-small">
        <span class="info-label">{selectedNode.label}</span>
        <span
          class="info-type typo-overline"
          style="color: {getColor(selectedNode.node_type)}"
          >{selectedNode.node_type}</span
        >
        <span class="info-id typo-mono">{selectedNode.id.slice(0, 8)}…</span>
      </div>
    {/if}
  </div>

  {#if tooltip}
    <div
      class="node-tooltip typo-small"
      style="left: {tooltip.x + 12}px; top: {tooltip.y - 8}px;"
    >
      <strong>{tooltip.node.label}</strong>
      <span>{tooltip.node.node_type}</span>
    </div>
  {/if}

  <div class="panel-footer typo-small">
    <span class="stats typo-mono"
      >{nodes.length} nodes, {edges.length} edges</span
    >
    <span class="hint typo-small"
      >Pan: drag · Zoom: scroll · Select: click · Drag node: move</span
    >
  </div>
</div>

<style>
  .graph-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px var(--spacing-3);
    border-bottom: 1px solid var(--color-surface-dark-border);
    flex-shrink: 0;
  }
  .header-actions {
    display: flex;
    gap: 2px;
  }
  .tool-btn {
    padding: 2px 6px;
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark);
    color: var(--color-on-dark-soft);
    cursor: pointer;
    border-radius: var(--radius-xs);
  }
  .tool-btn:hover {
    background: var(--color-surface-dark-elevated);
    color: var(--color-on-dark);
  }

  .form-overlay {
    padding: var(--spacing-2);
    border-bottom: 1px solid var(--color-surface-dark-border);
  }
  .form-row {
    display: flex;
    gap: var(--spacing-1);
    align-items: center;
  }
  .form-row input {
    flex: 1;
    padding: 4px var(--spacing-2);
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark);
    color: var(--color-on-dark);
    border-radius: var(--radius-xs);
    outline: none;
    min-width: 0;
  }
  .form-row input:focus {
    border-color: var(--color-primary);
  }
  .form-row select {
    padding: 3px 4px;
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark-soft);
    color: var(--color-on-dark-soft);
    border-radius: var(--radius-xs);
  }
  .btn-primary,
  .btn-secondary {
    padding: 3px 8px;
    border: 1px solid var(--color-surface-dark-border);
    border-radius: var(--radius-xs);
    cursor: pointer;
    white-space: nowrap;
  }
  .btn-primary {
    background: var(--color-primary);
    color: var(--color-on-primary);
    border-color: var(--color-primary);
  }
  .btn-secondary {
    background: var(--color-surface-dark-elevated);
    color: var(--color-on-dark-soft);
  }

  .canvas-container {
    flex: 1;
    position: relative;
    overflow: hidden;
  }
  .graph-canvas {
    width: 100%;
    height: 100%;
    display: block;
    cursor: grab;
  }
  .graph-canvas:active {
    cursor: grabbing;
  }

  .loading-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: color-mix(in srgb, var(--color-surface-dark) 90%, transparent);
    color: var(--color-on-dark-soft);
    z-index: 10;
  }

  .node-info {
    position: absolute;
    bottom: var(--spacing-2);
    left: var(--spacing-2);
    background: var(--color-surface-dark-soft);
    border: 1px solid var(--color-surface-dark-border);
    border-radius: var(--radius-md);
    padding: 6px 10px;
    display: flex;
    gap: var(--spacing-2);
    align-items: center;
  }
  .info-label {
    color: var(--color-on-dark);
    font-weight: 500;
  }
  .info-type {
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  .info-id {
    color: var(--color-muted);
    font-family: var(--font-mono);
  }

  .panel-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 4px var(--spacing-3);
    border-top: 1px solid var(--color-surface-dark-border);
    color: var(--color-muted);
    flex-shrink: 0;
  }
  .stats {
    font-family: var(--font-mono);
  }
  .hint {
    text-align: right;
  }
  .node-tooltip {
    position: fixed;
    z-index: 200;
    background: var(--color-surface-dark-elevated);
    border: 1px solid var(--color-surface-dark-border);
    border-radius: var(--radius-sm);
    padding: 4px 8px;
    color: var(--color-on-dark);
    pointer-events: none;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
</style>
