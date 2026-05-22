export interface Project {
  id: string;
  name: string;
  path: string;
  description?: string;
  createdAt: string;
  updatedAt: string;
  tags: string[];
}

export type TaskStatus = 'todo' | 'doing' | 'done';
export type TaskPriority = 'low' | 'medium' | 'high';

export interface Task {
  id: string;
  title: string;
  description?: string;
  status: TaskStatus;
  priority: TaskPriority;
  createdAt: string;
  updatedAt: string;
  projectId?: string;
  milestoneId?: string;
}

export interface Milestone {
  id: string;
  title: string;
  description?: string;
  dueDate?: string;
  status: 'open' | 'closed';
  projectId?: string;
  createdAt: string;
  updatedAt: string;
}

export type PanelId =
  | 'explorer'
  | 'projects'
  | 'tasks'
  | 'milestones'
  | 'graphs'
  | 'search'
  | 'activity'
  | 'layouts'
  | 'notes'
  | 'inspector'
  | 'dashboard'
  | 'git';

export interface Tab {
  id: string;
  title: string;
  icon?: string;
  closable: boolean;
  dirty?: boolean;
  filePath?: string;
}

export interface Command {
  id: string;
  label: string;
  shortcut?: string;
  category: string;
  icon?: string;
  action: () => void;
}

export interface LayoutConfig {
  activePanel: PanelId | null;
  sidebarWidth: number;
  leftSidebarVisible: boolean;
  bottomPanelOpen: boolean;
  openTabs: Tab[];
  activeTabId: string | null;
}

export interface FileEntry {
  name: string;
  path: string;
  is_dir: boolean;
  size: number;
  modified: string;
}

export interface Note {
  id: string;
  title: string;
  content: string;
  projectId?: string;
  createdAt: string;
  updatedAt: string;
}
