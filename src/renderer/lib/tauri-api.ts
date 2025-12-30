import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';
import { open as shellOpen } from '@tauri-apps/plugin-shell';
import type {
  GitHubUser,
  GitHubRepo,
  GitHubOrg,
  GitHubEvent,
  BackupOptions,
  BackupProgress,
  BackupHistoryEntry,
  ArchiveOptions,
  ArchiveProgress,
  AppSettings,
  ApiResponse,
} from '@shared/types';

// Type-safe API exposed to renderer (matching the Electron API interface)
export interface TauriAPI {
  // GitHub
  github: {
    validateToken: (token: string) => Promise<ApiResponse<{ valid: boolean; scopes?: string[] }>>;
    getUser: () => Promise<ApiResponse<GitHubUser>>;
    getRepos: () => Promise<ApiResponse<GitHubRepo[]>>;
    getOrgs: () => Promise<ApiResponse<GitHubOrg[]>>;
    getEvents: (limit?: number) => Promise<ApiResponse<GitHubEvent[]>>;
    getStats: () => Promise<ApiResponse<{ stars: number; forks: number }>>;
  };

  // Backup
  backup: {
    start: (repoIds: number[], options: BackupOptions) => Promise<ApiResponse<void>>;
    cancel: () => Promise<ApiResponse<void>>;
    getHistory: () => Promise<ApiResponse<BackupHistoryEntry[]>>;
    onProgress: (callback: (progress: BackupProgress) => void) => () => void;
    onComplete: (callback: (result: { success: boolean; message: string }) => void) => () => void;
    onError: (callback: (error: string) => void) => () => void;
  };

  // Archive
  archive: {
    create: (source: string, destination: string, options: ArchiveOptions) => Promise<ApiResponse<string>>;
    cancel: () => Promise<ApiResponse<void>>;
    onProgress: (callback: (progress: ArchiveProgress) => void) => () => void;
    onComplete: (callback: (result: { path: string; size: number }) => void) => () => void;
  };

  // Settings
  settings: {
    get: () => Promise<ApiResponse<AppSettings>>;
    set: (settings: Partial<AppSettings>) => Promise<ApiResponse<void>>;
    getToken: () => Promise<ApiResponse<string | null>>;
    setToken: (token: string) => Promise<ApiResponse<void>>;
    clearToken: () => Promise<ApiResponse<void>>;
  };

  // Dialog & Shell
  dialog: {
    selectFolder: () => Promise<string | null>;
  };
  shell: {
    openPath: (path: string) => Promise<void>;
    openExternal: (url: string) => Promise<void>;
  };
}

// Store for active listeners to clean up
const activeListeners: Map<string, UnlistenFn> = new Map();

// Helper to create event listeners with cleanup
function createListener<T>(channel: string, callback: (data: T) => void): () => void {
  // Clean up any existing listener for this channel
  const existing = activeListeners.get(channel);
  if (existing) {
    existing();
    activeListeners.delete(channel);
  }

  // Create new listener
  let unlisten: UnlistenFn | null = null;

  listen<T>(channel, (event) => {
    callback(event.payload);
  }).then((fn) => {
    unlisten = fn;
    activeListeners.set(channel, fn);
  });

  // Return cleanup function
  return () => {
    if (unlisten) {
      unlisten();
      activeListeners.delete(channel);
    }
  };
}

// The Tauri API implementation
export const tauriAPI: TauriAPI = {
  // GitHub API
  github: {
    validateToken: async (token: string) => {
      return await invoke<ApiResponse<{ valid: boolean; scopes?: string[] }>>('github_validate_token', { token });
    },
    getUser: async () => {
      return await invoke<ApiResponse<GitHubUser>>('github_get_user');
    },
    getRepos: async () => {
      return await invoke<ApiResponse<GitHubRepo[]>>('github_get_repos');
    },
    getOrgs: async () => {
      return await invoke<ApiResponse<GitHubOrg[]>>('github_get_orgs');
    },
    getEvents: async (limit?: number) => {
      return await invoke<ApiResponse<GitHubEvent[]>>('github_get_events', { limit });
    },
    getStats: async () => {
      return await invoke<ApiResponse<{ stars: number; forks: number }>>('github_get_stats');
    },
  },

  // Backup
  backup: {
    start: async (repoIds: number[], options: BackupOptions) => {
      return await invoke<ApiResponse<void>>('backup_start', { repoIds, options });
    },
    cancel: async () => {
      return await invoke<ApiResponse<void>>('backup_cancel');
    },
    getHistory: async () => {
      return await invoke<ApiResponse<BackupHistoryEntry[]>>('backup_get_history');
    },
    onProgress: (callback) => createListener('backup:progress', callback),
    onComplete: (callback) => createListener('backup:complete', callback),
    onError: (callback) => createListener('backup:error', callback),
  },

  // Archive
  archive: {
    create: async (source: string, destination: string, options: ArchiveOptions) => {
      return await invoke<ApiResponse<string>>('archive_create', { source, destination, options });
    },
    cancel: async () => {
      return await invoke<ApiResponse<void>>('archive_cancel');
    },
    onProgress: (callback) => createListener('archive:progress', callback),
    onComplete: (callback) => createListener('archive:complete', callback),
  },

  // Settings
  settings: {
    get: async () => {
      return await invoke<ApiResponse<AppSettings>>('settings_get');
    },
    set: async (settings: Partial<AppSettings>) => {
      return await invoke<ApiResponse<void>>('settings_set', { settings });
    },
    getToken: async () => {
      return await invoke<ApiResponse<string | null>>('settings_get_token');
    },
    setToken: async (token: string) => {
      return await invoke<ApiResponse<void>>('settings_set_token', { token });
    },
    clearToken: async () => {
      return await invoke<ApiResponse<void>>('settings_clear_token');
    },
  },

  // Dialog
  dialog: {
    selectFolder: async () => {
      const result = await open({
        directory: true,
        title: 'Select Backup Destination',
      });
      return result as string | null;
    },
  },

  // Shell
  shell: {
    openPath: async (path: string) => {
      await shellOpen(path);
    },
    openExternal: async (url: string) => {
      await shellOpen(url);
    },
  },
};

// Export a function to get the API (for compatibility with existing code structure)
export function getAPI(): TauriAPI {
  return tauriAPI;
}

// Make API available globally for easier migration
declare global {
  interface Window {
    tauriAPI: TauriAPI;
    electronAPI: TauriAPI; // For backwards compatibility during migration
  }
}

// Set up global API
if (typeof window !== 'undefined') {
  window.tauriAPI = tauriAPI;
  window.electronAPI = tauriAPI; // For backwards compatibility
}

export default tauriAPI;
