use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub node_type: String,
    pub label: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub edge_type: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePosition {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSnapshot {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

impl GraphNode {
    pub fn new(node_type: impl Into<String>, label: impl Into<String>) -> Self {
        GraphNode {
            id: uuid::Uuid::new_v4().to_string(),
            node_type: node_type.into(),
            label: label.into(),
            metadata: None,
        }
    }
}

impl GraphEdge {
    pub fn new(
        source_id: impl Into<String>,
        target_id: impl Into<String>,
        edge_type: impl Into<String>,
    ) -> Self {
        GraphEdge {
            id: uuid::Uuid::new_v4().to_string(),
            source_id: source_id.into(),
            target_id: target_id.into(),
            edge_type: edge_type.into(),
            label: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    nodes: HashMap<String, GraphNode>,
    edges: Vec<GraphEdge>,
    adjacency: HashMap<String, Vec<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: Vec::new(),
            adjacency: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: GraphNode) {
        let id = node.id.clone();
        self.nodes.insert(id.clone(), node);
        self.adjacency.entry(id).or_default();
    }

    pub fn remove_node(&mut self, id: &str) -> Option<GraphNode> {
        let node = self.nodes.remove(id)?;
        self.edges
            .retain(|e| e.source_id != id && e.target_id != id);
        self.adjacency.remove(id);
        for adj in self.adjacency.values_mut() {
            adj.retain(|n| n != id);
        }
        Some(node)
    }

    pub fn add_edge(&mut self, edge: GraphEdge) {
        let src = edge.source_id.clone();
        let tgt = edge.target_id.clone();
        self.edges.push(edge);
        self.adjacency.entry(src).or_default().push(tgt);
    }

    pub fn remove_edge(&mut self, id: &str) -> Option<GraphEdge> {
        let idx = self.edges.iter().position(|e| e.id == id)?;
        let edge = self.edges.remove(idx);
        if let Some(adj) = self.adjacency.get_mut(&edge.source_id) {
            adj.retain(|n| *n != edge.target_id);
        }
        Some(edge)
    }

    pub fn get_node(&self, id: &str) -> Option<&GraphNode> {
        self.nodes.get(id)
    }

    pub fn snapshot(&self) -> GraphSnapshot {
        GraphSnapshot {
            nodes: self.nodes.values().cloned().collect(),
            edges: self.edges.clone(),
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
        self.adjacency.clear();
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}
