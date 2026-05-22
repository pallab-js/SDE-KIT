export interface GraphNode {
  id: string;
  node_type: string;
  label: string;
  metadata?: Record<string, unknown>;
}

export interface GraphEdge {
  id: string;
  source_id: string;
  target_id: string;
  edge_type: string;
  label?: string;
}

export interface NodePosition {
  id: string;
  x: number;
  y: number;
}

export interface GraphSnapshot {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

async function invoke<T>(
  cmd: string,
  args?: Record<string, unknown>
): Promise<T> {
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    return await invoke<T>(cmd, args);
  } catch {
    throw new Error('Tauri API not available');
  }
}

export function addGraphNode(
  nodeType: string,
  label: string,
  metadata?: Record<string, unknown>
): Promise<GraphNode> {
  return invoke('add_graph_node', { nodeType, label, metadata });
}

export function removeGraphNode(id: string): Promise<void> {
  return invoke('remove_graph_node', { id });
}

export function addGraphEdge(
  sourceId: string,
  targetId: string,
  edgeType: string
): Promise<GraphEdge> {
  return invoke('add_graph_edge', { sourceId, targetId, edgeType });
}

export function removeGraphEdge(id: string): Promise<void> {
  return invoke('remove_graph_edge', { id });
}

export function getGraphSnapshot(): Promise<GraphSnapshot> {
  return invoke('get_graph_snapshot');
}

export function clearGraph(): Promise<void> {
  return invoke('clear_graph');
}

export function computeGraphLayout(
  width: number,
  height: number
): Promise<NodePosition[]> {
  return invoke('compute_graph_layout', { width, height });
}
