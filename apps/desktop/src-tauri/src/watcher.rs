use notify::{Config, Event, EventKind, RecommendedWatcher, Watcher};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use tauri::{AppHandle, Emitter};

pub struct WatcherHandle {
    pub cancel: Arc<AtomicBool>,
}

impl Drop for WatcherHandle {
    fn drop(&mut self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}

/// Spawn a background thread that watches `path` recursively and emits
/// "fs-event" Tauri events. Returns immediately; watching continues
/// until the WatcherHandle is dropped.
pub fn start_watching(app: AppHandle, path: String) -> Result<WatcherHandle, String> {
    let cancel = Arc::new(AtomicBool::new(false));
    let cancel_clone = cancel.clone();
    let (tx, rx) = mpsc::channel::<Result<Event, notify::Error>>();

    // Watcher is moved into the thread so it lives as long as the thread.
    thread::spawn(move || {
        let mut watcher = match RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default(),
        ) {
            Ok(w) => w,
            Err(e) => {
                log::error!("Failed to create file watcher: {e}");
                return;
            }
        };

        if let Err(e) = watcher.watch(Path::new(&path), notify::RecursiveMode::Recursive) {
            log::error!("Failed to start watching path {}: {}", path, e);
            return;
        }

        while !cancel_clone.load(Ordering::Relaxed) {
            match rx.recv_timeout(std::time::Duration::from_millis(500)) {
                Ok(Ok(event)) => {
                    let kind = match event.kind {
                        EventKind::Create(_) => "created",
                        EventKind::Modify(_) => "modified",
                        EventKind::Remove(_) => "removed",
                        _ => continue,
                    };
                    let paths: Vec<String> = event
                        .paths
                        .iter()
                        .map(|p| p.to_string_lossy().replace('\\', "/"))
                        .collect();
                    let _ = app.emit(
                        "fs-event",
                        serde_json::json!({ "kind": kind, "paths": paths }),
                    );
                }
                Ok(Err(e)) => {
                    log::error!("Watcher event error: {e}");
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    // Periodic timeout check
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    break;
                }
            }
        }
        // watcher dropped here — watching ends cleanly on thread exit
    });

    Ok(WatcherHandle { cancel })
}
