use crate::error::AppError;
use crate::persistence::Database;
use rusqlite::params;
use tauri::State;

#[tauri::command]
pub fn get_workspace_state(key: String, db: State<Database>) -> Result<Option<String>, AppError> {
    let conn = db.pool.get()?;
    let mut stmt = conn.prepare("SELECT value FROM workspace_state WHERE key = ?1")?;
    let result = stmt.query_row(params![key], |row| row.get::<_, String>(0));
    match result {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(AppError::Database(e)),
    }
}

#[tauri::command]
pub fn set_workspace_state(
    key: String,
    value: String,
    db: State<Database>,
) -> Result<(), AppError> {
    let conn = db.pool.get()?;
    conn.execute(
        "INSERT INTO workspace_state (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
        params![key, value],
    )?;
    Ok(())
}
