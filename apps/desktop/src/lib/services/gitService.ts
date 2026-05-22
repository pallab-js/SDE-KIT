import { workspaceRoot } from '$lib/stores/workspace';
import { get } from 'svelte/store';

export interface GitStatusEntry {
  status: string;
  path: string;
  staged: boolean;
}

export interface GitBranch {
  current: string;
  branches: string[];
}

export interface GitLogEntry {
  hash: string;
  message: string;
  author: string;
  date: string;
}

async function runGit(args: string[]): Promise<string> {
  const cwd = get(workspaceRoot);
  if (!cwd) throw new Error('No workspace root set');

  const { Command } = await import('@tauri-apps/plugin-shell');
  const cmd = Command.create('git', args, { cwd });
  let stdout = '';
  let stderr = '';

  cmd.stdout.on('data', (line: string) => {
    stdout += line;
  });
  cmd.stderr.on('data', (line: string) => {
    stderr += line;
  });

  const result = await cmd.execute();
  if (result.code !== 0 && stderr.trim()) {
    // git may output to stderr for non-error info
    if (stderr.toLowerCase().includes('not a git repository')) {
      throw new Error('Not a git repository');
    }
    if (stderr.toLowerCase().includes('fatal')) {
      throw new Error(stderr.trim());
    }
  }
  return stdout || stderr;
}

export async function getStatus(): Promise<GitStatusEntry[]> {
  try {
    const stdout = await runGit(['status', '--porcelain']);
    if (!stdout.trim()) return [];

    return stdout
      .split('\n')
      .filter(Boolean)
      .map((line) => ({
        status: line.substring(0, 2).trim(),
        path: line.substring(3).trim(),
        staged: line[0] !== ' ' && line[0] !== '?',
      }));
  } catch {
    return [];
  }
}

export async function getBranches(): Promise<GitBranch> {
  try {
    const stdout = await runGit(['branch']);
    const branches = stdout
      .split('\n')
      .filter(Boolean)
      .map((b) => b.trim());
    const current =
      branches.find((b) => b.startsWith('*'))?.replace('* ', '') ?? 'main';
    return {
      current,
      branches: branches.map((b) => b.replace('* ', '')),
    };
  } catch {
    return { current: 'unknown', branches: [] };
  }
}

export async function stage(path: string): Promise<void> {
  await runGit(['add', path]);
}

export async function unstage(path: string): Promise<void> {
  await runGit(['reset', 'HEAD', path]);
}

export async function stageAll(): Promise<void> {
  await runGit(['add', '-A']);
}

export async function commit(message: string): Promise<void> {
  await runGit(['commit', '-m', message]);
}

export async function getLog(count = 10): Promise<GitLogEntry[]> {
  try {
    const stdout = await runGit([
      'log',
      `--max-count=${count}`,
      '--format=%H|||%s|||%an|||%ar',
    ]);
    if (!stdout.trim()) return [];

    return stdout
      .split('\n')
      .filter(Boolean)
      .map((line) => {
        const [hash, message, author, date] = line.split('|||');
        return {
          hash: hash?.slice(0, 7) ?? '',
          message: message ?? '',
          author: author ?? '',
          date: date ?? '',
        };
      });
  } catch {
    return [];
  }
}

export async function getDiff(path: string, staged = false): Promise<string> {
  try {
    const args = staged
      ? ['diff', '--cached', '--', path]
      : ['diff', '--', path];
    return await runGit(args);
  } catch {
    return '';
  }
}

export async function checkout(branch: string): Promise<void> {
  await runGit(['checkout', branch]);
}

export async function createBranch(name: string): Promise<void> {
  await runGit(['checkout', '-b', name]);
}

export async function isRepo(): Promise<boolean> {
  try {
    await runGit(['rev-parse', '--git-dir']);
    return true;
  } catch {
    return false;
  }
}
