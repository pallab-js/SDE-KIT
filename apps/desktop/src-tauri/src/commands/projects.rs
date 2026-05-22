use crate::error::AppError;
use crate::models::Project;
use crate::persistence::Database;
use chrono::Utc;
use rusqlite::params;
use tauri::State;
use uuid::Uuid;

pub fn validate_name(name: &str) -> Result<(), AppError> {
    if name.trim().is_empty() {
        return Err(AppError::Validation("Name cannot be empty".into()));
    }
    if name.len() > 500 {
        return Err(AppError::Validation("Name too long (max 500 chars)".into()));
    }
    Ok(())
}

#[tauri::command]
pub fn get_projects(db: State<Database>) -> Result<Vec<Project>, AppError> {
    let conn = db.pool.get()?;
    let mut stmt = conn
        .prepare("SELECT id, name, path, description, created_at, updated_at, tags FROM projects ORDER BY updated_at DESC")?;

    let mut projects = Vec::new();
    let rows = stmt.query_map([], |row| {
        let tags_str: String = row.get(6)?;
        let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            path: row.get(2)?,
            description: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
            tags,
        })
    })?;

    for res in rows {
        match res {
            Ok(proj) => projects.push(proj),
            Err(e) => log::warn!("Skipping malformed project row: {e}"),
        }
    }
    Ok(projects)
}

#[tauri::command]
pub fn create_project(
    name: String,
    path: String,
    description: Option<String>,
    db: State<Database>,
) -> Result<Project, AppError> {
    validate_name(&name)?;
    let conn = db.pool.get()?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO projects (id, name, path, description, created_at, updated_at, tags) VALUES (?1, ?2, ?3, ?4, ?5, ?6, '[]')",
        params![id, name, path, description, now, now],
    )?;
    Ok(Project {
        id,
        name,
        path,
        description,
        created_at: now.clone(),
        updated_at: now,
        tags: vec![],
    })
}

#[tauri::command]
pub fn update_project(
    id: String,
    name: Option<String>,
    path: Option<String>,
    description: Option<String>,
    db: State<Database>,
) -> Result<(), AppError> {
    if let Some(ref n) = name {
        validate_name(n)?;
    }
    let conn = db.pool.get()?;
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE projects SET
            name        = COALESCE(?1, name),
            path        = COALESCE(?2, path),
            description = COALESCE(?3, description),
            updated_at  = ?4
         WHERE id = ?5",
        params![name, path, description, now, id],
    )?;
    Ok(())
}

#[tauri::command]
pub fn delete_project(id: String, db: State<Database>) -> Result<(), AppError> {
    let conn = db.pool.get()?;
    let tx = conn.unchecked_transaction()?;
    tx.execute("DELETE FROM tasks WHERE project_id = ?1", params![id])?;
    tx.execute("DELETE FROM milestones WHERE project_id = ?1", params![id])?;
    tx.execute("DELETE FROM projects WHERE id = ?1", params![id])?;
    tx.commit()?;
    Ok(())
}
