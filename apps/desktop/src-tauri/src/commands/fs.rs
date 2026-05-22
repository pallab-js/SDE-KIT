use crate::error::AppError;
use crate::watcher;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Clone, serde::Serialize)]
pub struct SearchResult {
    pub path: String,
    pub line_number: usize,
    pub line: String,
}

#[tauri::command]
pub fn search_in_files(
    query: String,
    root: State<WorkspaceRoot>,
    case_sensitive: bool,
) -> Result<Vec<SearchResult>, AppError> {
    let root_guard = root
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let base = root_guard
        .clone()
        .ok_or_else(|| AppError::Workspace("no workspace root set".into()))?;
    drop(root_guard);

    let mut results = Vec::new();
    search_dir(&base, &base, &query, case_sensitive, &mut results, 0)?;
    Ok(results)
}

fn search_dir(
    dir: &std::path::Path,
    base: &std::path::Path,
    query: &str,
    case_sensitive: bool,
    results: &mut Vec<SearchResult>,
    depth: usize,
) -> Result<(), AppError> {
    if depth > 10 {
        return Ok(());
    }
    let entries = std::fs::read_dir(dir)?;
    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        if name.starts_with('.') || name == "node_modules" || name == "target" || name == "dist" {
            continue;
        }
        if path.is_dir() {
            let _ = search_dir(&path, base, query, case_sensitive, results, depth + 1);
        } else if let Ok(content) = std::fs::read_to_string(&path) {
            let relative = path.strip_prefix(base).unwrap_or(&path);
            for (i, line) in content.lines().enumerate() {
                let matches = if case_sensitive {
                    line.contains(query)
                } else {
                    line.to_lowercase().contains(&query.to_lowercase())
                };
                if matches {
                    results.push(SearchResult {
                        path: normalize_path(relative),
                        line_number: i + 1,
                        line: line.trim().to_string(),
                    });
                    if results.len() >= 500 {
                        return Ok(());
                    }
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: String,
}

pub struct WorkspaceRoot(pub Mutex<Option<PathBuf>>);

fn resolve_within_root(root: &Path, requested: &str) -> Result<PathBuf, AppError> {
    let root_canonical = root
        .canonicalize()
        .map_err(|_| AppError::Workspace("invalid workspace root".to_string()))?;
    let joined = root_canonical.join(requested);
    if joined.exists() {
        let canonical = joined
            .canonicalize()
            .map_err(|_| AppError::Workspace("path does not exist".to_string()))?;
        if canonical.starts_with(&root_canonical) {
            return Ok(canonical);
        }
        return Err(AppError::Workspace("path traversal denied".to_string()));
    }
    if !joined.starts_with(&root_canonical) {
        return Err(AppError::Workspace("path traversal denied".to_string()));
    }
    Ok(joined)
}

fn check_root(path: &str, root: &Option<PathBuf>) -> Result<PathBuf, AppError> {
    let p = Path::new(path);
    match root {
        Some(r) => resolve_within_root(r, path),
        None => {
            if !p.exists() {
                let parent = p.parent().unwrap_or(p);
                if !parent.exists() {
                    return Err(AppError::Workspace("path does not exist".to_string()));
                }
            }
            Ok(p.to_path_buf())
        }
    }
}

fn check_root_strict(path: &str, root: &Option<PathBuf>) -> Result<PathBuf, AppError> {
    let r = root
        .as_ref()
        .ok_or_else(|| AppError::Workspace("no workspace root set".to_string()))?;
    resolve_within_root(r, path)
}

fn normalize_path(raw: &Path) -> String {
    raw.to_string_lossy().replace("\\", "/")
}

fn fmt_time(meta: &std::fs::Metadata) -> String {
    let sys = meta.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH);
    let dt: chrono::DateTime<chrono::Utc> = chrono::DateTime::from(sys);
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[tauri::command]
pub fn set_workspace_root(
    path: String,
    root: State<WorkspaceRoot>,
    active_watcher: State<crate::ActiveWatcher>,
    app: tauri::AppHandle,
) -> Result<(), AppError> {
    let p = Path::new(&path)
        .canonicalize()
        .map_err(|e| AppError::Workspace(format!("invalid path: {e}")))?;
    if !p.is_dir() {
        return Err(AppError::Workspace("not a directory".to_string()));
    }
    let mut state = root
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    *state = Some(p.clone());

    // Stop old watcher if exists by dropping the handle
    let mut watcher_guard = active_watcher
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    *watcher_guard = None;

    // Start watching the new workspace root
    let handle = watcher::start_watching(app, normalize_path(&p)).map_err(AppError::Workspace)?;
    *watcher_guard = Some(handle);
    Ok(())
}

#[tauri::command]
pub fn list_directory(
    path: String,
    root: State<WorkspaceRoot>,
) -> Result<Vec<FileEntry>, AppError> {
    let root_guard = root
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let dir = check_root(&path, &root_guard)?;
    drop(root_guard);
    if !dir.is_dir() {
        return Err(AppError::Workspace("not a directory".to_string()));
    }

    let mut entries: Vec<FileEntry> = Vec::new();
    let mut read_dir = std::fs::read_dir(&dir)?;

    while let Some(entry) = read_dir.next().transpose()? {
        let meta = entry.metadata()?;
        let file_type = entry.file_type()?;
        entries.push(FileEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            path: normalize_path(&entry.path()),
            is_dir: file_type.is_dir(),
            size: meta.len(),
            modified: fmt_time(&meta),
        });
    }

    entries.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });

    Ok(entries)
}

#[tauri::command]
pub fn read_file(path: String, root: State<WorkspaceRoot>) -> Result<String, AppError> {
    let root_guard = root
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let resolved = check_root(&path, &root_guard)?;
    drop(root_guard);
    let content = std::fs::read_to_string(&resolved)?;
    Ok(content)
}

#[tauri::command]
pub fn write_file(
    path: String,
    content: String,
    root: State<WorkspaceRoot>,
) -> Result<(), AppError> {
    let root_guard = root
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let resolved = check_root_strict(&path, &root_guard)?;
    drop(root_guard);
    std::fs::write(&resolved, &content)?;
    Ok(())
}

#[tauri::command]
pub fn create_directory(path: String, root: State<WorkspaceRoot>) -> Result<(), AppError> {
    let root_guard = root
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let resolved = check_root_strict(&path, &root_guard)?;
    drop(root_guard);
    std::fs::create_dir_all(&resolved)?;
    Ok(())
}

#[tauri::command]
pub fn delete_file(path: String, root: State<WorkspaceRoot>) -> Result<(), AppError> {
    let root_guard = root
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let resolved = check_root_strict(&path, &root_guard)?;
    drop(root_guard);
    if resolved.is_dir() {
        std::fs::remove_dir_all(&resolved)?;
    } else {
        std::fs::remove_file(&resolved)?;
    }
    Ok(())
}

#[tauri::command]
pub fn rename_file(
    old_path: String,
    new_path: String,
    root: State<WorkspaceRoot>,
) -> Result<(), AppError> {
    let root_guard = root
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let old_resolved = check_root_strict(&old_path, &root_guard)?;
    let new_resolved = check_root_strict(&new_path, &root_guard)?;
    drop(root_guard);
    std::fs::rename(&old_resolved, &new_resolved)?;
    Ok(())
}

#[tauri::command]
pub fn get_file_info(path: String, root: State<WorkspaceRoot>) -> Result<FileEntry, AppError> {
    let root_guard = root
        .0
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let resolved = check_root(&path, &root_guard)?;
    drop(root_guard);
    let meta = resolved.metadata()?;
    Ok(FileEntry {
        name: resolved
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        path: normalize_path(&resolved),
        is_dir: resolved.is_dir(),
        size: meta.len(),
        modified: fmt_time(&meta),
    })
}
