/**
 * Local-First Database Service
 * - Wraps Tauri Rust SQLite export commands
 * - Provides export/backup for solo developers
 * - Zero cloud dependencies; works fully offline
 */

import { invoke } from '@tauri-apps/api/core';

export interface DBResult<T> {
  success: boolean;
  data?: T;
  error?: string;
}

export type ExportFormat = 'json' | 'sqlite';

export class LocalDatabase {
  private static instance: LocalDatabase;
  private initialized = false;

  private constructor() {}

  static getInstance(): LocalDatabase {
    if (!LocalDatabase.instance) {
      LocalDatabase.instance = new LocalDatabase();
    }
    return LocalDatabase.instance;
  }

  async initialize(): Promise<DBResult<void>> {
    if (this.initialized) return { success: true };
    this.initialized = true;
    return { success: true };
  }

  async exportProject(
    projectId: string,
    format: ExportFormat
  ): Promise<DBResult<Blob>> {
    try {
      if (format === 'json') {
        const jsonStr = await invoke<string>('export_project_json', {
          projectId,
        });
        return {
          success: true,
          data: new Blob([jsonStr], { type: 'application/json' }),
        };
      }
      const bytes = await invoke<number[]>('export_project_sqlite', {
        projectId,
      });
      return {
        success: true,
        data: new Blob([new Uint8Array(bytes)], {
          type: 'application/x-sqlite3',
        }),
      };
    } catch (err) {
      return { success: false, error: String(err) };
    }
  }
}
