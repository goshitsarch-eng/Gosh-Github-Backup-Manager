// GitHub User
export interface GitHubUser {
  id: number
  login: string
  name: string | null
  avatar_url: string
  bio: string | null
  location: string | null
  company: string | null
  blog: string | null
  email: string | null
  public_repos: number
  total_private_repos?: number
  owned_private_repos?: number
  followers: number
  following: number
  disk_usage?: number
  created_at: string
  updated_at: string
}

// GitHub Repository
export interface GitHubRepo {
  id: number
  name: string
  full_name: string
  description: string | null
  private: boolean
  fork: boolean
  archived: boolean
  disabled: boolean
  html_url: string
  clone_url: string
  ssh_url: string
  homepage: string | null
  language: string | null
  stargazers_count: number
  watchers_count: number
  forks_count: number
  open_issues_count: number
  size: number
  default_branch: string
  visibility: 'public' | 'private' | 'internal'
  pushed_at: string | null
  created_at: string
  updated_at: string
  owner: {
    login: string
    id: number
    avatar_url: string
    type: 'User' | 'Organization'
  }
  permissions?: {
    admin: boolean
    push: boolean
    pull: boolean
  }
}

// GitHub Organization
export interface GitHubOrg {
  id: number
  login: string
  description: string | null
  avatar_url: string
  url: string
}

// GitHub Event
export interface GitHubEvent {
  id: string
  type: string
  actor: {
    login: string
    avatar_url: string
  }
  repo: {
    id: number
    name: string
  }
  payload: Record<string, unknown>
  created_at: string
}

// Backup Options
export interface BackupOptions {
  destination: string
  cloneType: 'full' | 'mirror' | 'shallow'
  includeForks: boolean
  includeArchived: boolean
  splitBy: 'none' | 'owner' | 'visibility'
  createZip: boolean
  zipCompression: number // 1-9
  excludePatterns: string[]
  maxConcurrent: number // 1-5
}

// Backup Status
export type RepoBackupStatus =
  | 'pending'
  | 'cloning'
  | 'pulling'
  | 'complete'
  | 'failed'
  | 'skipped'

export interface RepoBackupProgress {
  repoId: number
  repoName: string
  status: RepoBackupStatus
  progress?: number // 0-100 for cloning progress
  error?: string
}

export interface BackupProgress {
  totalRepos: number
  completedRepos: number
  failedRepos: number
  currentRepo: string | null
  repos: RepoBackupProgress[]
  startTime: number
  isRunning: boolean
  isCancelled: boolean
}

// Backup History
export interface BackupHistoryEntry {
  id: string
  date: string
  repoCount: number
  totalSize: number // in bytes
  duration: number // in milliseconds
  destination: string
  status: 'complete' | 'partial' | 'failed'
  failedRepos: string[]
  options: BackupOptions
}

// Archive Options
export interface ArchiveOptions {
  compressionLevel: number // 1-9
  excludePatterns: string[]
  splitByOwner: boolean
}

export interface ArchiveProgress {
  isRunning: boolean
  progress: number // 0-100
  currentFile: string | null
  totalFiles: number
  processedFiles: number
}

// App Settings
export interface AppSettings {
  theme: 'light' | 'dark' | 'system'
  defaultBackupLocation: string
  defaultBackupOptions: Partial<BackupOptions>
  notifications: boolean
  autoBackup: {
    enabled: boolean
    schedule: 'daily' | 'weekly'
    time: string // HH:mm format
    lastRun: string | null
  }
}

// IPC Channel Types
export interface IpcChannels {
  // GitHub
  'github:validate-token': { token: string }
  'github:get-user': void
  'github:get-repos': void
  'github:get-orgs': void
  'github:get-events': { limit?: number }

  // Backup
  'backup:start': { repos: number[]; options: BackupOptions }
  'backup:cancel': void
  'backup:get-history': void

  // Archive
  'archive:create': { source: string; options: ArchiveOptions }
  'archive:cancel': void

  // Settings
  'settings:get': void
  'settings:set': Partial<AppSettings>

  // Dialogs
  'dialog:select-folder': void
  'shell:open-path': { path: string }
}

// API Response wrapper
export interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: string
}
