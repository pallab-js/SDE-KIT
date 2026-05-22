use crate::error::AppError;
use crate::models::{Milestone, MilestoneStatus};
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
pub fn get_milestones(
    limit: Option<i64>,
    offset: Option<i64>,
    db: State<Database>,
) -> Result<Vec<Milestone>, AppError> {
    let limit = limit.unwrap_or(200).min(500);
    let offset = offset.unwrap_or(0);
    let conn = db.pool.get()?;
    let mut stmt = conn
        .prepare("SELECT id, title, description, due_date, status, project_id, created_at, updated_at FROM milestones ORDER BY created_at DESC LIMIT ?1 OFFSET ?2")?;

    let mut milestones = Vec::new();
    let rows = stmt.query_map(params![limit, offset], |row| {
        let s: String = row.get(4)?;
        Ok(Milestone {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            due_date: row.get(3)?,
            status: MilestoneStatus::from_str(&s).unwrap_or(MilestoneStatus::Open),
            project_id: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;

    for res in rows {
        match res {
            Ok(m) => milestones.push(m),
            Err(e) => log::warn!("Skipping malformed milestone row: {e}"),
        }
    }
    Ok(milestones)
}

#[tauri::command]
pub fn create_milestone(
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    project_id: Option<String>,
    db: State<Database>,
) -> Result<Milestone, AppError> {
    validate_title(&title)?;
    let conn = db.pool.get()?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO milestones (id, title, description, due_date, status, project_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, 'open', ?5, ?6, ?7)",
        params![id, title, description, due_date, project_id, now, now],
    )?;
    Ok(Milestone {
        id,
        title,
        description,
        due_date,
        status: MilestoneStatus::Open,
        project_id,
        created_at: now.clone(),
        updated_at: now,
    })
}

#[tauri::command]
pub fn update_milestone_status(
    id: String,
    status: String,
    db: State<Database>,
) -> Result<(), AppError> {
    let conn = db.pool.get()?;
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE milestones SET status = ?1, updated_at = ?2 WHERE id = ?3",
        params![status, now, id],
    )?;
    Ok(())
}

#[tauri::command]
pub fn delete_milestone(id: String, db: State<Database>) -> Result<(), AppError> {
    let conn = db.pool.get()?;
    conn.execute("DELETE FROM milestones WHERE id = ?1", params![id])?;
    Ok(())
}

#[tauri::command]
pub fn assign_task_to_milestone(
    task_id: String,
    milestone_id: Option<String>,
    db: State<Database>,
) -> Result<(), AppError> {
    let conn = db.pool.get()?;
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE tasks SET milestone_id = ?1, updated_at = ?2 WHERE id = ?3",
        params![milestone_id, now, task_id],
    )?;
    Ok(())
}
