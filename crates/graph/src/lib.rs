pub mod layout;
pub mod types;

pub use layout::ForceDirectedLayout;
pub use types::{Graph, GraphEdge, GraphNode, GraphSnapshot, NodePosition};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_graph_is_empty() {
        let g = Graph::new();
        assert_eq!(g.node_count(), 0);
        assert_eq!(g.edge_count(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut g = Graph::new();
        let node = GraphNode::new("test", "hello");
        let id = node.id.clone();
        g.add_node(node);
        assert_eq!(g.node_count(), 1);
        assert!(g.get_node(&id).is_some());
    }

    #[test]
    fn test_remove_node_removes_edges() {
        let mut g = Graph::new();
        let a = GraphNode::new("test", "A");
        let b = GraphNode::new("test", "B");
        let a_id = a.id.clone();
        let b_id = b.id.clone();
        g.add_node(a);
        g.add_node(b);
        let edge = GraphEdge::new(&a_id, &b_id, "related");
        g.add_edge(edge);
        assert_eq!(g.edge_count(), 1);
        g.remove_node(&a_id).unwrap();
        assert_eq!(g.node_count(), 1);
        assert_eq!(g.edge_count(), 0);
    }

    #[test]
    fn test_add_edge() {
        let mut g = Graph::new();
        let a = GraphNode::new("test", "A");
        let b = GraphNode::new("test", "B");
        let a_id = a.id.clone();
        let b_id = b.id.clone();
        g.add_node(a);
        g.add_node(b);
        let edge = GraphEdge::new(&a_id, &b_id, "related");
        g.add_edge(edge);
        assert_eq!(g.edge_count(), 1);
    }

    #[test]
    fn test_remove_edge() {
        let mut g = Graph::new();
        let a = GraphNode::new("test", "A");
        let b = GraphNode::new("test", "B");
        let a_id = a.id.clone();
        let b_id = b.id.clone();
        g.add_node(a);
        g.add_node(b);
        let edge = GraphEdge::new(&a_id, &b_id, "related");
        let e_id = edge.id.clone();
        g.add_edge(edge);
        assert_eq!(g.edge_count(), 1);
        g.remove_edge(&e_id).unwrap();
        assert_eq!(g.edge_count(), 0);
    }

    #[test]
    fn test_remove_nonexistent_node() {
        let mut g = Graph::new();
        assert!(g.remove_node("nonexistent").is_none());
    }

    #[test]
    fn test_remove_nonexistent_edge() {
        let mut g = Graph::new();
        assert!(g.remove_edge("nonexistent").is_none());
    }

    #[test]
    fn test_snapshot() {
        let mut g = Graph::new();
        g.add_node(GraphNode::new("test", "A"));
        g.add_node(GraphNode::new("test", "B"));
        let a = GraphNode::new("test", "C");
        let b = GraphNode::new("test", "D");
        let a_id = a.id.clone();
        let b_id = b.id.clone();
        g.add_node(a);
        g.add_node(b);
        let edge = GraphEdge::new(&a_id, &b_id, "edge");
        g.add_edge(edge);
        let snap = g.snapshot();
        assert_eq!(snap.nodes.len(), 4);
        assert_eq!(snap.edges.len(), 1);
    }

    #[test]
    fn test_clear() {
        let mut g = Graph::new();
        g.add_node(GraphNode::new("test", "A"));
        g.add_node(GraphNode::new("test", "B"));
        assert_eq!(g.node_count(), 2);
        g.clear();
        assert_eq!(g.node_count(), 0);
        assert_eq!(g.edge_count(), 0);
    }

    #[test]
    fn test_force_layout_runs() {
        let nodes = vec![
            GraphNode::new("test", "A"),
            GraphNode::new("test", "B"),
            GraphNode::new("test", "C"),
        ];
        let edges = vec![
            GraphEdge::new(&nodes[0].id, &nodes[1].id, "e1"),
            GraphEdge::new(&nodes[1].id, &nodes[2].id, "e2"),
        ];
        let positions = layout::force_directed(&nodes, &edges, 800.0, 600.0);
        assert_eq!(positions.len(), 3);
        for pos in &positions {
            assert!(pos.x >= 20.0 && pos.x <= 780.0);
            assert!(pos.y >= 20.0 && pos.y <= 580.0);
        }
    }

    #[test]
    fn test_empty_layout() {
        let positions = layout::force_directed(&[], &[], 800.0, 600.0);
        assert!(positions.is_empty());
    }

    #[test]
    fn test_force_directed_layout_struct() {
        let nodes = vec![
            GraphNode::new("task", "A"),
            GraphNode::new("task", "B"),
            GraphNode::new("milestone", "C"),
            GraphNode::new("task", "D"),
        ];
        let edges = vec![
            GraphEdge::new(&nodes[0].id, &nodes[1].id, "depends"),
            GraphEdge::new(&nodes[1].id, &nodes[2].id, "blocks"),
            GraphEdge::new(&nodes[2].id, &nodes[3].id, "related"),
        ];

        let layout = layout::ForceDirectedLayout::new(42);
        let positions = layout.simulate(&nodes, &edges, 100, 0.01);

        assert_eq!(positions.len(), 4);
        for pos in &positions {
            assert!(pos.x >= -500.0 && pos.x <= 500.0);
            assert!(pos.y >= -500.0 && pos.y <= 500.0);
        }
    }

    #[test]
    fn test_force_directed_layout_deterministic() {
        let nodes = vec![GraphNode::new("task", "X"), GraphNode::new("task", "Y")];
        let edges = vec![GraphEdge::new(&nodes[0].id, &nodes[1].id, "depends")];

        let layout = layout::ForceDirectedLayout::new(1234);
        let pos_a = layout.simulate(&nodes, &edges, 50, 0.1);
        let pos_b = layout.simulate(&nodes, &edges, 50, 0.1);

        assert_eq!(pos_a.len(), pos_b.len());
        for (a, b) in pos_a.iter().zip(pos_b.iter()) {
            assert!((a.x - b.x).abs() < 1e-10);
            assert!((a.y - b.y).abs() < 1e-10);
        }
    }

    #[test]
    fn test_force_directed_layout_empty() {
        let layout = layout::ForceDirectedLayout::new(0);
        let positions = layout.simulate(&[], &[], 100, 0.01);
        assert!(positions.is_empty());
    }

    #[test]
    fn test_node_metadata() {
        let mut node = GraphNode::new("test", "meta-node");
        node.metadata = Some(serde_json::json!({"key": "value"}));
        assert_eq!(node.node_type, "test");
        assert_eq!(node.label, "meta-node");
        assert!(node.metadata.is_some());
    }
}
