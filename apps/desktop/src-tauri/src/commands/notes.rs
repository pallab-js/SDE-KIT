use crate::error::AppError;
use crate::models::Note;
use crate::persistence::Database;
use chrono::Utc;
use rusqlite::params;
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
pub fn get_notes(
    project_id: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
    db: State<Database>,
) -> Result<Vec<Note>, AppError> {
    let limit = limit.unwrap_or(200).min(500);
    let offset = offset.unwrap_or(0);
    let conn = db.pool.get()?;

    let mut notes = Vec::new();
    if let Some(pid) = project_id {
        let mut stmt = conn.prepare(
            "SELECT id, title, content, project_id, created_at, updated_at FROM notes WHERE project_id = ?1 ORDER BY updated_at DESC LIMIT ?2 OFFSET ?3"
        )?;
        let rows = stmt.query_map(params![pid, limit, offset], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                project_id: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;
        for res in rows {
            match res {
                Ok(n) => notes.push(n),
                Err(e) => log::warn!("Skipping malformed note row: {e}"),
            }
        }
    } else {
        let mut stmt = conn.prepare(
            "SELECT id, title, content, project_id, created_at, updated_at FROM notes ORDER BY updated_at DESC LIMIT ?1 OFFSET ?2"
        )?;
        let rows = stmt.query_map(params![limit, offset], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                project_id: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;
        for res in rows {
            match res {
                Ok(n) => notes.push(n),
                Err(e) => log::warn!("Skipping malformed note row: {e}"),
            }
        }
    }
    Ok(notes)
}

#[tauri::command]
pub fn get_note(id: String, db: State<Database>) -> Result<Option<Note>, AppError> {
    let conn = db.pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT id, title, content, project_id, created_at, updated_at FROM notes WHERE id = ?1",
    )?;
    let result = stmt.query_row(params![id], |row| {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            project_id: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    });
    match result {
        Ok(note) => Ok(Some(note)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(AppError::Database(e)),
    }
}

#[tauri::command]
pub fn create_note(
    title: String,
    content: String,
    project_id: Option<String>,
    db: State<Database>,
) -> Result<Note, AppError> {
    validate_title(&title)?;
    let conn = db.pool.get()?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO notes (id, title, content, project_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id, title, content, project_id, now, now],
    )?;
    Ok(Note {
        id,
        title,
        content,
        project_id,
        created_at: now.clone(),
        updated_at: now,
    })
}

#[tauri::command]
pub fn update_note(
    id: String,
    title: Option<String>,
    content: Option<String>,
    project_id: Option<Option<String>>,
    db: State<Database>,
) -> Result<(), AppError> {
    if let Some(ref t) = title {
        validate_title(t)?;
    }
    let conn = db.pool.get()?;
    let now = Utc::now().to_rfc3339();

    // We perform individual updates or coalesce based on whether project_id updates are provided
    if let Some(pid_opt) = project_id {
        conn.execute(
            "UPDATE notes SET
                title      = COALESCE(?1, title),
                content    = COALESCE(?2, content),
                project_id = ?3,
                updated_at = ?4
             WHERE id = ?5",
            params![title, content, pid_opt, now, id],
        )?;
    } else {
        conn.execute(
            "UPDATE notes SET
                title      = COALESCE(?1, title),
                content    = COALESCE(?2, content),
                updated_at = ?3
             WHERE id = ?4",
            params![title, content, now, id],
        )?;
    }
    Ok(())
}

#[tauri::command]
pub fn delete_note(id: String, db: State<Database>) -> Result<(), AppError> {
    let conn = db.pool.get()?;
    conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
    Ok(())
}
