use crate::error::AppError;
use crate::persistence::Database;
use rusqlite::params;
use serde_json::json;
use tauri::Manager;
use tauri::State;

#[tauri::command]
pub fn export_project_json(project_id: String, db: State<Database>) -> Result<String, AppError> {
    let conn = db.pool.get()?;

    // 1. Query project
    let project = conn.query_row(
        "SELECT id, name, path, description, created_at, updated_at, tags FROM projects WHERE id = ?1",
        params![project_id],
        |row| {
            let tags_str: String = row.get(6)?;
            let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
            Ok(json!({
                "id": row.get::<usize, String>(0)?,
                "name": row.get::<usize, String>(1)?,
                "path": row.get::<usize, String>(2)?,
                "description": row.get::<usize, Option<String>>(3)?,
                "created_at": row.get::<usize, String>(4)?,
                "updated_at": row.get::<usize, String>(5)?,
                "tags": tags,
            }))
        }
    )?;

    // 2. Query tasks
    let mut stmt = conn.prepare(
        "SELECT id, title, description, status, priority, created_at, updated_at, project_id, milestone_id FROM tasks WHERE project_id = ?1"
    )?;
    let tasks: Vec<serde_json::Value> = stmt
        .query_map(params![project_id], |row| {
            Ok(json!({
                "id": row.get::<usize, String>(0)?,
                "title": row.get::<usize, String>(1)?,
                "description": row.get::<usize, Option<String>>(2)?,
                "status": row.get::<usize, String>(3)?,
                "priority": row.get::<usize, String>(4)?,
                "created_at": row.get::<usize, String>(5)?,
                "updated_at": row.get::<usize, String>(6)?,
                "project_id": row.get::<usize, Option<String>>(7)?,
                "milestone_id": row.get::<usize, Option<String>>(8)?,
            }))
        })?
        .filter_map(|r| r.ok())
        .collect();

    // 3. Query milestones
    let mut stmt = conn.prepare(
        "SELECT id, title, description, due_date, status, project_id, created_at, updated_at FROM milestones WHERE project_id = ?1"
    )?;
    let milestones: Vec<serde_json::Value> = stmt
        .query_map(params![project_id], |row| {
            Ok(json!({
                "id": row.get::<usize, String>(0)?,
                "title": row.get::<usize, String>(1)?,
                "description": row.get::<usize, Option<String>>(2)?,
                "due_date": row.get::<usize, Option<String>>(3)?,
                "status": row.get::<usize, String>(4)?,
                "project_id": row.get::<usize, Option<String>>(5)?,
                "created_at": row.get::<usize, String>(6)?,
                "updated_at": row.get::<usize, String>(7)?,
            }))
        })?
        .filter_map(|r| r.ok())
        .collect();

    // 4. Query graph nodes
    let mut stmt = conn.prepare("SELECT id, node_type, label, metadata FROM graph_nodes")?;
    let nodes: Vec<serde_json::Value> = stmt
        .query_map([], |row| {
            let meta_str: Option<String> = row.get(3)?;
            let metadata: Option<serde_json::Value> =
                meta_str.and_then(|s| serde_json::from_str(&s).ok());
            Ok(json!({
                "id": row.get::<usize, String>(0)?,
                "node_type": row.get::<usize, String>(1)?,
                "label": row.get::<usize, String>(2)?,
                "metadata": metadata,
            }))
        })?
        .filter_map(|r| r.ok())
        .collect();

    let mut stmt =
        conn.prepare("SELECT id, source_id, target_id, edge_type, label FROM graph_edges")?;
    let edges: Vec<serde_json::Value> = stmt
        .query_map([], |row| {
            Ok(json!({
                "id": row.get::<usize, String>(0)?,
                "source_id": row.get::<usize, String>(1)?,
                "target_id": row.get::<usize, String>(2)?,
                "edge_type": row.get::<usize, String>(3)?,
                "label": row.get::<usize, Option<String>>(4)?,
            }))
        })?
        .filter_map(|r| r.ok())
        .collect();

    let export_data = json!({
        "project": project,
        "tasks": tasks,
        "milestones": milestones,
        "graph": {
            "nodes": nodes,
            "edges": edges,
        },
        "exported_at": chrono::Utc::now().to_rfc3339(),
        "version": "1.0",
    });

    Ok(serde_json::to_string_pretty(&export_data)?)
}

#[tauri::command]
pub fn export_project_sqlite(
    _project_id: String,
    db: State<Database>,
    app: tauri::AppHandle,
) -> Result<Vec<u8>, AppError> {
    let conn = db.pool.get()?;
    // perform checkpoint to flush WAL
    let _ = conn.execute("PRAGMA wal_checkpoint(TRUNCATE);", []);
    drop(conn);

    let app_dir = app.path().app_data_dir()?;
    let db_path = app_dir.join("sde-kit.db");

    let bytes = std::fs::read(&db_path)?;
    Ok(bytes)
}
