import { useState, useCallback } from 'react'
import { useApp } from '@/context/AppContext'
import type { BackupOptions, BackupHistoryEntry } from '../../shared/types'
import { DEFAULT_SETTINGS } from '../../shared/constants'

interface UseBackupReturn {
  progress: ReturnType<typeof useApp>['backupProgress']
  history: BackupHistoryEntry[]
  isRunning: boolean
  isLoadingHistory: boolean
  error: string | null
  startBackup: (repoIds: number[], options: BackupOptions) => Promise<{ success: boolean; error?: string }>
  cancelBackup: () => Promise<void>
  fetchHistory: () => Promise<void>
  selectFolder: () => Promise<string | null>
  openFolder: (path: string) => Promise<void>
}

export function useBackup(): UseBackupReturn {
  // Use global state from AppContext for progress (persists across navigation)
  const { backupProgress, isBackupRunning } = useApp()

  // Local state for history and errors (can be refetched)
  const [history, setHistory] = useState<BackupHistoryEntry[]>([])
  const [isLoadingHistory, setIsLoadingHistory] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const startBackup = useCallback(async (
    repoIds: number[],
    options: BackupOptions
  ): Promise<{ success: boolean; error?: string }> => {
    setError(null)

    try {
      const result = await window.electronAPI.backup.start(repoIds, options)
      if (!result.success) {
        setError(result.error || 'Failed to start backup')
        return { success: false, error: result.error }
      }
      return { success: true }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to start backup'
      setError(errorMessage)
      return { success: false, error: errorMessage }
    }
  }, [])

  const cancelBackup = useCallback(async () => {
    try {
      await window.electronAPI.backup.cancel()
    } catch (err) {
      console.error('Failed to cancel backup:', err)
    }
  }, [])

  const fetchHistory = useCallback(async () => {
    setIsLoadingHistory(true)

    try {
      const result = await window.electronAPI.backup.getHistory()
      if (result.success && result.data) {
        setHistory(result.data)
      }
    } catch (err) {
      console.error('Failed to fetch history:', err)
    } finally {
      setIsLoadingHistory(false)
    }
  }, [])

  const selectFolder = useCallback(async (): Promise<string | null> => {
    try {
      return await window.electronAPI.dialog.selectFolder()
    } catch (err) {
      console.error('Failed to select folder:', err)
      return null
    }
  }, [])

  const openFolder = useCallback(async (path: string) => {
    try {
      await window.electronAPI.shell.openPath(path)
    } catch (err) {
      console.error('Failed to open folder:', err)
    }
  }, [])

  return {
    progress: backupProgress,
    history,
    isRunning: isBackupRunning,
    isLoadingHistory,
    error,
    startBackup,
    cancelBackup,
    fetchHistory,
    selectFolder,
    openFolder,
  }
}

// Default backup options
export function getDefaultBackupOptions(settingsDefaults?: Partial<BackupOptions>): BackupOptions {
  return {
    destination: '',
    ...DEFAULT_SETTINGS.defaultBackupOptions,
    ...settingsDefaults,
  } as BackupOptions
}
