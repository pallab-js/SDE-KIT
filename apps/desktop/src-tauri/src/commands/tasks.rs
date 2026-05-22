use crate::error::AppError;
use crate::models::{Task, TaskPriority, TaskStatus};
use crate::persistence::Database;
use chrono::Utc;
use rusqlite::params;
use std::str::FromStr;
use tauri::State;
use uuid::Uuid;

fn validate_title(title: &str) -> Result<(), AppError> {
    if title.trim().is_empty() {
        return Err(AppError::Validation("Title cannot be empty".into()));
    }
    if title.len() > 500 {
        return Err(AppError::Validation(
            "Title too long (max 500 chars)".into(),
        ));
    }
    Ok(())
}

#[tauri::command]
pub fn get_tasks(
    limit: Option<i64>,
    offset: Option<i64>,
    db: State<Database>,
) -> Result<Vec<Task>, AppError> {
    let limit = limit.unwrap_or(200).min(500);
    let offset = offset.unwrap_or(0);
    let conn = db.pool.get()?;
    let mut stmt = conn
        .prepare("SELECT id, title, description, status, priority, created_at, updated_at, project_id, milestone_id FROM tasks ORDER BY created_at DESC LIMIT ?1 OFFSET ?2")?;

    let mut tasks = Vec::new();
    let rows = stmt.query_map(params![limit, offset], |row| {
        let s: String = row.get(3)?;
        let p: String = row.get(4)?;
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            status: TaskStatus::from_str(&s).unwrap_or(TaskStatus::Todo),
            priority: TaskPriority::from_str(&p).unwrap_or(TaskPriority::Medium),
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
            project_id: row.get(7)?,
            milestone_id: row.get(8)?,
        })
    })?;

    for res in rows {
        match res {
            Ok(t) => tasks.push(t),
            Err(e) => log::warn!("Skipping malformed task row: {e}"),
        }
    }
    Ok(tasks)
}

#[tauri::command]
pub fn get_tasks_by_project(
    project_id: String,
    limit: Option<i64>,
    offset: Option<i64>,
    db: State<Database>,
) -> Result<Vec<Task>, AppError> {
    let limit = limit.unwrap_or(200).min(500);
    let offset = offset.unwrap_or(0);
    let conn = db.pool.get()?;
    let mut stmt = conn
        .prepare("SELECT id, title, description, status, priority, created_at, updated_at, project_id, milestone_id FROM tasks WHERE project_id = ?1 ORDER BY created_at DESC LIMIT ?2 OFFSET ?3")?;

    let mut tasks = Vec::new();
    let rows = stmt.query_map(params![project_id, limit, offset], |row| {
        let s: String = row.get(3)?;
        let p: String = row.get(4)?;
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            status: TaskStatus::from_str(&s).unwrap_or(TaskStatus::Todo),
            priority: TaskPriority::from_str(&p).unwrap_or(TaskPriority::Medium),
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
            project_id: row.get(7)?,
            milestone_id: row.get(8)?,
        })
    })?;

    for res in rows {
        match res {
            Ok(t) => tasks.push(t),
            Err(e) => log::warn!("Skipping malformed task row: {e}"),
        }
    }
    Ok(tasks)
}

#[tauri::command]
pub fn create_task(
    title: String,
    description: Option<String>,
    priority: Option<String>,
    project_id: Option<String>,
    milestone_id: Option<String>,
    db: State<Database>,
) -> Result<Task, AppError> {
    validate_title(&title)?;
    let conn = db.pool.get()?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let p = priority.unwrap_or_else(|| "medium".to_string());
    conn.execute(
        "INSERT INTO tasks (id, title, description, status, priority, created_at, updated_at, project_id, milestone_id) VALUES (?1, ?2, ?3, 'todo', ?4, ?5, ?6, ?7, ?8)",
        params![id, title, description, p, now, now, project_id, milestone_id],
    )?;
    Ok(Task {
        id,
        title,
        description,
        status: TaskStatus::Todo,
        priority: TaskPriority::from_str(&p).unwrap_or(TaskPriority::Medium),
        created_at: now.clone(),
        updated_at: now,
        project_id,
        milestone_id,
    })
}

#[tauri::command]
pub fn update_task(
    id: String,
    title: Option<String>,
    description: Option<String>,
    priority: Option<String>,
    db: State<Database>,
) -> Result<(), AppError> {
    if let Some(ref t) = title {
        validate_title(t)?;
    }
    let conn = db.pool.get()?;
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE tasks SET
            title       = COALESCE(?1, title),
            description = COALESCE(?2, description),
            priority    = COALESCE(?3, priority),
            updated_at  = ?4
         WHERE id = ?5",
        params![title, description, priority, now, id],
    )?;
    Ok(())
}

#[tauri::command]
pub fn update_task_status(id: String, status: String, db: State<Database>) -> Result<(), AppError> {
    let conn = db.pool.get()?;
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE tasks SET status = ?1, updated_at = ?2 WHERE id = ?3",
        params![status, now, id],
    )?;
    Ok(())
}

#[tauri::command]
pub fn delete_task(id: String, db: State<Database>) -> Result<(), AppError> {
    let conn = db.pool.get()?;
    conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
    Ok(())
}
