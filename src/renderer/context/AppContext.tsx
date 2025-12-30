import React, { createContext, useContext, useState, useEffect, useCallback } from 'react'
import type { GitHubUser, GitHubRepo, GitHubOrg, AppSettings, BackupProgress } from '../../shared/types'
import { DEFAULT_SETTINGS } from '../../shared/constants'
import { toast } from '@/components/ui/use-toast'

interface AppState {
  // Auth
  isAuthenticated: boolean
  isLoading: boolean
  authStatus: string | null

  // User data
  user: GitHubUser | null
  repos: GitHubRepo[]
  orgs: GitHubOrg[]

  // Settings
  settings: AppSettings
  theme: 'light' | 'dark' | 'system'

  // Backup
  selectedRepos: Set<number>
  backupProgress: BackupProgress | null
  isBackupRunning: boolean

  // Actions
  login: (token: string) => Promise<{ success: boolean; error?: string }>
  logout: () => Promise<void>
  refreshData: () => Promise<void>
  setSelectedRepos: (repos: Set<number>) => void
  toggleRepoSelection: (repoId: number) => void
  selectAllRepos: () => void
  deselectAllRepos: () => void
  setSettings: (settings: Partial<AppSettings>) => Promise<void>
  setTheme: (theme: 'light' | 'dark' | 'system') => void
}

const AppContext = createContext<AppState | null>(null)

const IPC_TIMEOUT_MS = 5000
const LOGIN_TIMEOUT_MS = 30000

function withTimeout<T>(promise: Promise<T>, ms: number, label: string): Promise<T> {
  let timeoutId: number | undefined
  const timeout = new Promise<T>((_, reject) => {
    timeoutId = window.setTimeout(() => {
      reject(new Error(`${label} timed out after ${ms}ms`))
    }, ms)
  })

  return Promise.race([promise, timeout]).finally(() => {
    if (timeoutId !== undefined) {
      window.clearTimeout(timeoutId)
    }
  })
}

export function AppProvider({ children }: { children: React.ReactNode }) {
  const [isAuthenticated, setIsAuthenticated] = useState(false)
  const [isLoading, setIsLoading] = useState(true)
  const [authStatus, setAuthStatus] = useState<string | null>(null)
  const [user, setUser] = useState<GitHubUser | null>(null)
  const [repos, setRepos] = useState<GitHubRepo[]>([])
  const [orgs, setOrgs] = useState<GitHubOrg[]>([])
  const [settings, setSettingsState] = useState<AppSettings>(DEFAULT_SETTINGS as AppSettings)
  const [theme, setThemeState] = useState<'light' | 'dark' | 'system'>('system')
  const [selectedRepos, setSelectedRepos] = useState<Set<number>>(new Set())
  const [backupProgress, setBackupProgress] = useState<BackupProgress | null>(null)
  const [isBackupRunning, setIsBackupRunning] = useState(false)

  // Apply theme
  useEffect(() => {
    const applyTheme = (t: 'light' | 'dark' | 'system') => {
      const root = document.documentElement
      if (t === 'system') {
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
        root.classList.toggle('dark', prefersDark)
      } else {
        root.classList.toggle('dark', t === 'dark')
      }
    }

    applyTheme(theme)

    // Listen for system theme changes
    if (theme === 'system') {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
      const handler = () => applyTheme('system')
      mediaQuery.addEventListener('change', handler)
      return () => mediaQuery.removeEventListener('change', handler)
    }
  }, [theme])

  // Initialize app
  useEffect(() => {
    let isMounted = true
    const loadingTimer = window.setTimeout(() => {
      if (isMounted) {
        setIsLoading(false)
      }
    }, 3000)

    const init = async () => {
      try {
        if (!window.electronAPI?.settings?.get) {
          console.warn('Tauri API bridge not available; skipping init.')
          return
        }

        // Load settings
        const [settingsResult, tokenResult] = await Promise.allSettled([
          withTimeout(window.electronAPI.settings.get(), IPC_TIMEOUT_MS, 'settings.get'),
          withTimeout(window.electronAPI.settings.getToken(), IPC_TIMEOUT_MS, 'settings.getToken'),
        ])

        if (settingsResult.status === 'fulfilled') {
          const result = settingsResult.value
          if (result.success && result.data) {
            setSettingsState(result.data)
            setThemeState(result.data.theme)
          }
        } else {
          console.warn('Failed to load settings:', settingsResult.reason)
        }

        // Check for stored token
        if (tokenResult.status === 'fulfilled') {
          const result = tokenResult.value
          if (result.success && result.data) {
            // Validate and fetch user data
            const userResult = await withTimeout(
              window.electronAPI.github.getUser(),
              IPC_TIMEOUT_MS,
              'github.getUser'
            )
            if (userResult.success && userResult.data) {
              setUser(userResult.data)
              setIsAuthenticated(true)

              // Fetch repos and orgs in background
              fetchData()
            }
          }
        } else {
          console.warn('Failed to load stored token:', tokenResult.reason)
        }
      } catch (error) {
        console.error('Failed to initialize app:', error)
      } finally {
        if (isMounted) {
          setIsLoading(false)
        }
        window.clearTimeout(loadingTimer)
      }
    }

    init()
    return () => {
      isMounted = false
      window.clearTimeout(loadingTimer)
    }
  }, [])

  // Subscribe to backup progress
  useEffect(() => {
    const unsubscribeProgress = window.electronAPI.backup.onProgress((progress) => {
      setBackupProgress(progress)
      setIsBackupRunning(progress.isRunning)
    })

    const unsubscribeComplete = window.electronAPI.backup.onComplete((result) => {
      setIsBackupRunning(false)
      // Update progress to show completed state
      setBackupProgress(prev => prev ? {
        ...prev,
        isRunning: false,
        currentRepo: null,
      } : null)
      // Show toast notification (works even when not on backup page)
      toast({
        title: result.success ? 'Backup Complete' : 'Backup Finished',
        description: result.message,
        variant: result.success ? 'default' : 'destructive',
      })
    })

    return () => {
      unsubscribeProgress()
      unsubscribeComplete()
    }
  }, [])

  const fetchData = useCallback(async () => {
    try {
      const [reposResult, orgsResult] = await Promise.all([
        window.electronAPI.github.getRepos(),
        window.electronAPI.github.getOrgs(),
      ])

      if (reposResult.success && reposResult.data) {
        setRepos(reposResult.data)
      }
      if (orgsResult.success && orgsResult.data) {
        setOrgs(orgsResult.data)
      }
    } catch (error) {
      console.error('Failed to fetch data:', error)
    }
  }, [])

  const login = useCallback(async (token: string): Promise<{ success: boolean; error?: string }> => {
    try {
      setAuthStatus('Securing your token locally...')
      const setTokenResult = await window.electronAPI.settings.setToken(token)
      if (!setTokenResult.success) {
        setAuthStatus(null)
        return { success: false, error: setTokenResult.error || 'Failed to save token' }
      }

      // Fetch user data
      setAuthStatus('Fetching your GitHub profile...')
      const userResult = await withTimeout(
        window.electronAPI.github.getUser(),
        LOGIN_TIMEOUT_MS,
        'github.getUser'
      )
      if (!userResult.success || !userResult.data) {
        await window.electronAPI.settings.clearToken()
        setAuthStatus(null)
        return { success: false, error: userResult.error || 'Failed to get user' }
      }

      setUser(userResult.data)
      setIsAuthenticated(true)

      // Fetch additional data
      setAuthStatus('Syncing repositories and organizations...')
      void (async () => {
        try {
          await fetchData()
        } finally {
          setAuthStatus(null)
        }
      })()

      return { success: true }
    } catch (error) {
      try {
        await window.electronAPI.settings.clearToken()
      } catch (cleanupError) {
        console.warn('Failed to clear token after login error:', cleanupError)
      }
      setAuthStatus(null)
      return { success: false, error: error instanceof Error ? error.message : 'Login failed' }
    }
  }, [fetchData])

  const logout = useCallback(async () => {
    await window.electronAPI.settings.clearToken()
    setIsAuthenticated(false)
    setUser(null)
    setRepos([])
    setOrgs([])
    setSelectedRepos(new Set())
    setAuthStatus(null)
  }, [])

  const refreshData = useCallback(async () => {
    await fetchData()
  }, [fetchData])

  const toggleRepoSelection = useCallback((repoId: number) => {
    setSelectedRepos(prev => {
      const next = new Set(prev)
      if (next.has(repoId)) {
        next.delete(repoId)
      } else {
        next.add(repoId)
      }
      return next
    })
  }, [])

  const selectAllRepos = useCallback(() => {
    setSelectedRepos(new Set(repos.map(r => r.id)))
  }, [repos])

  const deselectAllRepos = useCallback(() => {
    setSelectedRepos(new Set())
  }, [])

  const setSettings = useCallback(async (newSettings: Partial<AppSettings>) => {
    const updated = { ...settings, ...newSettings }
    setSettingsState(updated)

    if (newSettings.theme) {
      setThemeState(newSettings.theme)
    }

    await window.electronAPI.settings.set(newSettings)
  }, [settings])

  const setTheme = useCallback((newTheme: 'light' | 'dark' | 'system') => {
    setThemeState(newTheme)
    setSettings({ theme: newTheme })
  }, [setSettings])

  const value: AppState = {
    isAuthenticated,
    isLoading,
    authStatus,
    user,
    repos,
    orgs,
    settings,
    theme,
    selectedRepos,
    backupProgress,
    isBackupRunning,
    login,
    logout,
    refreshData,
    setSelectedRepos,
    toggleRepoSelection,
    selectAllRepos,
    deselectAllRepos,
    setSettings,
    setTheme,
  }

  return <AppContext.Provider value={value}>{children}</AppContext.Provider>
}

export function useApp() {
  const context = useContext(AppContext)
  if (!context) {
    throw new Error('useApp must be used within an AppProvider')
  }
  return context
}
