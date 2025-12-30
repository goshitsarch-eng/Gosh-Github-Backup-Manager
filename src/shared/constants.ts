// IPC Channel names
export const IPC_CHANNELS = {
  // GitHub
  GITHUB_VALIDATE_TOKEN: 'github:validate-token',
  GITHUB_GET_USER: 'github:get-user',
  GITHUB_GET_REPOS: 'github:get-repos',
  GITHUB_GET_ORGS: 'github:get-orgs',
  GITHUB_GET_EVENTS: 'github:get-events',
  GITHUB_GET_STATS: 'github:get-stats',

  // Backup
  BACKUP_START: 'backup:start',
  BACKUP_CANCEL: 'backup:cancel',
  BACKUP_GET_HISTORY: 'backup:get-history',
  BACKUP_PROGRESS: 'backup:progress',
  BACKUP_REPO_STATUS: 'backup:repo-status',
  BACKUP_COMPLETE: 'backup:complete',
  BACKUP_ERROR: 'backup:error',

  // Archive
  ARCHIVE_CREATE: 'archive:create',
  ARCHIVE_CANCEL: 'archive:cancel',
  ARCHIVE_PROGRESS: 'archive:progress',
  ARCHIVE_COMPLETE: 'archive:complete',

  // Settings
  SETTINGS_GET: 'settings:get',
  SETTINGS_SET: 'settings:set',
  SETTINGS_GET_TOKEN: 'settings:get-token',
  SETTINGS_SET_TOKEN: 'settings:set-token',
  SETTINGS_CLEAR_TOKEN: 'settings:clear-token',

  // Dialogs & Shell
  DIALOG_SELECT_FOLDER: 'dialog:select-folder',
  SHELL_OPEN_PATH: 'shell:open-path',
} as const

// Default settings
export const DEFAULT_SETTINGS = {
  theme: 'system' as const,
  defaultBackupLocation: '',
  defaultBackupOptions: {
    cloneType: 'full' as const,
    includeForks: true,
    includeArchived: false,
    splitBy: 'owner' as const,
    createZip: false,
    zipCompression: 6,
    excludePatterns: ['node_modules', '.DS_Store', 'Thumbs.db'],
    maxConcurrent: 3,
  },
  notifications: true,
  autoBackup: {
    enabled: false,
    schedule: 'weekly' as const,
    time: '03:00',
    lastRun: null,
  },
}

// App info
export const APP_INFO = {
  name: 'Gosh Github Backup Manager',
  version: '1.0.0',
  author: 'Gosh-Its-Arch',
  github: 'https://github.com/goshitsarch-eng/Gosh-Github-Backup-Manager',
}

// GitHub token scopes required
export const REQUIRED_SCOPES = ['repo', 'read:user', 'read:org']

// Backup limits
export const BACKUP_LIMITS = {
  MAX_CONCURRENT: 5,
  MIN_CONCURRENT: 1,
  DEFAULT_CONCURRENT: 3,
  MAX_COMPRESSION: 9,
  MIN_COMPRESSION: 1,
  DEFAULT_COMPRESSION: 6,
}
