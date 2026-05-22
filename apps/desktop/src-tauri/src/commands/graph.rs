use crate::error::AppError;
use crate::persistence::{save_graph, Database};
use sde_kit_graph::layout;
use sde_kit_graph::types::{Graph, GraphEdge, GraphNode, GraphSnapshot, NodePosition};
use std::sync::Mutex;
use tauri::State;

fn persist(g: &Graph, db: &Database) -> Result<(), AppError> {
    let snap = g.snapshot();
    let mut conn = db.pool.get()?;
    save_graph(&mut conn, &snap)?;
    Ok(())
}

pub struct GraphState(pub Mutex<Graph>);

#[tauri::command]
pub fn add_graph_node(
    node_type: String,
    label: String,
    metadata: Option<serde_json::Value>,
    graph: State<GraphState>,
    db: State<Database>,
) -> Result<GraphNode, AppError> {
    let mut g = graph
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let node = GraphNode::new(node_type, label);
    let node = GraphNode { metadata, ..node };
    g.add_node(node.clone());
    persist(&g, &db)?;
    Ok(node)
}

#[tauri::command]
pub fn remove_graph_node(
    id: String,
    graph: State<GraphState>,
    db: State<Database>,
) -> Result<(), AppError> {
    let mut g = graph
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    g.remove_node(&id)
        .ok_or_else(|| AppError::Validation("node not found".into()))?;
    persist(&g, &db)?;
    Ok(())
}

#[tauri::command]
pub fn add_graph_edge(
    source_id: String,
    target_id: String,
    edge_type: String,
    graph: State<GraphState>,
    db: State<Database>,
) -> Result<GraphEdge, AppError> {
    let mut g = graph
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if g.get_node(&source_id).is_none() || g.get_node(&target_id).is_none() {
        return Err(AppError::Validation(
            "source or target node not found".into(),
        ));
    }
    let edge = GraphEdge::new(source_id, target_id, edge_type);
    g.add_edge(edge.clone());
    persist(&g, &db)?;
    Ok(edge)
}

#[tauri::command]
pub fn remove_graph_edge(
    id: String,
    graph: State<GraphState>,
    db: State<Database>,
) -> Result<(), AppError> {
    let mut g = graph
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    g.remove_edge(&id)
        .ok_or_else(|| AppError::Validation("edge not found".into()))?;
    persist(&g, &db)?;
    Ok(())
}

#[tauri::command]
pub fn get_graph_snapshot(graph: State<GraphState>) -> Result<GraphSnapshot, AppError> {
    let g = graph
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(g.snapshot())
}

#[tauri::command]
pub fn clear_graph(graph: State<GraphState>, db: State<Database>) -> Result<(), AppError> {
    let mut g = graph
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    g.clear();
    persist(&g, &db)?;
    Ok(())
}

#[tauri::command]
pub fn compute_graph_layout(
    width: f64,
    height: f64,
    graph: State<GraphState>,
) -> Result<Vec<NodePosition>, AppError> {
    let g = graph
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let snap = g.snapshot();
    if snap.nodes.is_empty() {
        return Ok(vec![]);
    }
    let positions = layout::force_directed(&snap.nodes, &snap.edges, width, height);
    Ok(positions)
}

#[tauri::command]
pub fn sync_graph_from_sdlc(graph: State<GraphState>, db: State<Database>) -> Result<(), AppError> {
    use std::str::FromStr;
    let conn = db.pool.get()?;

    let projects: Vec<crate::models::Project> = {
        let mut s = conn.prepare(
            "SELECT id, name, path, description, created_at, updated_at, tags FROM projects",
        )?;
        let x = s
            .query_map([], |r| {
                let tags_str: String = r.get(6)?;
                let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
                Ok(crate::models::Project {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    path: r.get(2)?,
                    description: r.get(3)?,
                    created_at: r.get(4)?,
                    updated_at: r.get(5)?,
                    tags,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();
        x
    };

    let tasks: Vec<crate::models::Task> = {
        let mut s = conn.prepare("SELECT id, title, description, status, priority, created_at, updated_at, project_id, milestone_id FROM tasks")?;
        let x = s
            .query_map([], |r| {
                let st: String = r.get(3)?;
                let pr: String = r.get(4)?;
                Ok(crate::models::Task {
                    id: r.get(0)?,
                    title: r.get(1)?,
                    description: r.get(2)?,
                    status: crate::models::TaskStatus::from_str(&st)
                        .unwrap_or(crate::models::TaskStatus::Todo),
                    priority: crate::models::TaskPriority::from_str(&pr)
                        .unwrap_or(crate::models::TaskPriority::Medium),
                    created_at: r.get(5)?,
                    updated_at: r.get(6)?,
                    project_id: r.get(7)?,
                    milestone_id: r.get(8)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();
        x
    };

    let milestones: Vec<crate::models::Milestone> = {
        let mut s = conn.prepare("SELECT id, title, description, due_date, status, project_id, created_at, updated_at FROM milestones")?;
        let x = s
            .query_map([], |r| {
                let st: String = r.get(4)?;
                Ok(crate::models::Milestone {
                    id: r.get(0)?,
                    title: r.get(1)?,
                    description: r.get(2)?,
                    due_date: r.get(3)?,
                    status: crate::models::MilestoneStatus::from_str(&st)
                        .unwrap_or(crate::models::MilestoneStatus::Open),
                    project_id: r.get(5)?,
                    created_at: r.get(6)?,
                    updated_at: r.get(7)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();
        x
    };
    drop(conn);

    let mut g = graph
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;

    for p in &projects {
        if g.get_node(&p.id).is_none() {
            g.add_node(sde_kit_graph::types::GraphNode {
                id: p.id.clone(),
                node_type: "project".into(),
                label: p.name.clone(),
                metadata: None,
            });
        }
    }
    for m in &milestones {
        if g.get_node(&m.id).is_none() {
            g.add_node(sde_kit_graph::types::GraphNode {
                id: m.id.clone(),
                node_type: "milestone".into(),
                label: m.title.clone(),
                metadata: None,
            });
        }
        if let Some(pid) = &m.project_id {
            if g.get_node(pid).is_some() {
                let snap = g.snapshot();
                if !snap
                    .edges
                    .iter()
                    .any(|e| e.source_id == *pid && e.target_id == m.id)
                {
                    g.add_edge(sde_kit_graph::types::GraphEdge {
                        id: format!("edge-{}-{}", pid, m.id),
                        source_id: pid.clone(),
                        target_id: m.id.clone(),
                        edge_type: "contains".into(),
                        label: None,
                    });
                }
            }
        }
    }
    for t in &tasks {
        if g.get_node(&t.id).is_none() {
            g.add_node(sde_kit_graph::types::GraphNode {
                id: t.id.clone(),
                node_type: "task".into(),
                label: t.title.clone(),
                metadata: None,
            });
        }
        if let Some(pid) = &t.project_id {
            if g.get_node(pid).is_some() {
                let snap = g.snapshot();
                if !snap
                    .edges
                    .iter()
                    .any(|e| e.source_id == *pid && e.target_id == t.id)
                {
                    g.add_edge(sde_kit_graph::types::GraphEdge {
                        id: format!("edge-{}-{}", pid, t.id),
                        source_id: pid.clone(),
                        target_id: t.id.clone(),
                        edge_type: "contains".into(),
                        label: None,
                    });
                }
            }
        }
    }

    let snap = g.snapshot();
    let mut db_conn = db.pool.get()?;
    save_graph(&mut db_conn, &snap)?;
    Ok(())
}
