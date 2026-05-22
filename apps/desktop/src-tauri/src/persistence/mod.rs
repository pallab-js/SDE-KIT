use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::path::Path;
use thiserror::Error;

pub mod migrations;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("database error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("pool error: {0}")]
    Pool(#[from] r2d2::Error),

    #[error("migration error: {0}")]
    Migration(#[from] rusqlite_migration::Error),
}

#[derive(Debug)]
struct SqliteCustomizer;

impl r2d2::CustomizeConnection<rusqlite::Connection, rusqlite::Error> for SqliteCustomizer {
    fn on_acquire(&self, conn: &mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;",
        )?;
        Ok(())
    }
}

pub struct Database {
    pub pool: Pool<SqliteConnectionManager>,
}

impl Database {
    pub fn new(app_dir: &Path) -> Result<Self, DbError> {
        std::fs::create_dir_all(app_dir).ok();
        let db_path = app_dir.join("sde-kit.db");

        let manager = SqliteConnectionManager::file(db_path);
        let pool = r2d2::Pool::builder()
            .connection_customizer(Box::new(SqliteCustomizer))
            .build(manager)?;

        let db = Database { pool };
        db.run_migrations()?;
        Ok(db)
    }

    fn run_migrations(&self) -> Result<(), DbError> {
        let mut conn = self.pool.get()?;
        let migrations = migrations::get_migrations();
        migrations.to_latest(&mut conn)?;
        Ok(())
    }
}

use sde_kit_graph::types::{GraphEdge, GraphNode, GraphSnapshot};

pub fn save_graph(conn: &mut rusqlite::Connection, snap: &GraphSnapshot) -> Result<(), DbError> {
    let tx = conn.unchecked_transaction()?;
    tx.execute("DELETE FROM graph_edges", [])?;
    tx.execute("DELETE FROM graph_nodes", [])?;
    for n in &snap.nodes {
        tx.execute(
            "INSERT INTO graph_nodes (id, node_type, label, metadata) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                n.id,
                n.node_type,
                n.label,
                n.metadata.as_ref().map(|m| m.to_string())
            ],
        )?;
    }
    for e in &snap.edges {
        tx.execute(
            "INSERT INTO graph_edges (id, source_id, target_id, edge_type, label) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![e.id, e.source_id, e.target_id, e.edge_type, e.label],
        )?;
    }
    tx.commit()?;
    Ok(())
}

#[allow(dead_code)]
pub fn schema_version(conn: &rusqlite::Connection) -> u32 {
    conn.query_row("SELECT version FROM schema_version", [], |r| r.get(0))
        .unwrap_or(0)
}

pub fn load_graph(conn: &rusqlite::Connection) -> Result<GraphSnapshot, DbError> {
    let mut stmt = conn.prepare("SELECT id, node_type, label, metadata FROM graph_nodes")?;
    let nodes: Vec<GraphNode> = stmt
        .query_map([], |row| {
            let meta_str: Option<String> = row.get(3)?;
            let metadata = meta_str.and_then(|s| serde_json::from_str(&s).ok());
            Ok(GraphNode {
                id: row.get(0)?,
                node_type: row.get(1)?,
                label: row.get(2)?,
                metadata,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    let mut stmt =
        conn.prepare("SELECT id, source_id, target_id, edge_type, label FROM graph_edges")?;
    let edges: Vec<GraphEdge> = stmt
        .query_map([], |row| {
            Ok(GraphEdge {
                id: row.get(0)?,
                source_id: row.get(1)?,
                target_id: row.get(2)?,
                edge_type: row.get(3)?,
                label: row.get(4)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(GraphSnapshot { nodes, edges })
}
