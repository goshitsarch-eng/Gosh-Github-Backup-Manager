import { useState, useCallback } from 'react'
import type { AppSettings } from '../../shared/types'

interface UseSettingsReturn {
  isLoading: boolean
  error: string | null
  updateSettings: (settings: Partial<AppSettings>) => Promise<boolean>
  clearToken: () => Promise<boolean>
}

export function useSettings(): UseSettingsReturn {
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const updateSettings = useCallback(async (settings: Partial<AppSettings>): Promise<boolean> => {
    setIsLoading(true)
    setError(null)

    try {
      const result = await window.electronAPI.settings.set(settings)
      if (!result.success) {
        setError(result.error || 'Failed to save settings')
        return false
      }
      return true
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to save settings')
      return false
    } finally {
      setIsLoading(false)
    }
  }, [])

  const clearToken = useCallback(async (): Promise<boolean> => {
    setIsLoading(true)
    setError(null)

    try {
      const result = await window.electronAPI.settings.clearToken()
      if (!result.success) {
        setError(result.error || 'Failed to clear token')
        return false
      }
      return true
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to clear token')
      return false
    } finally {
      setIsLoading(false)
    }
  }, [])

  return {
    isLoading,
    error,
    updateSettings,
    clearToken,
  }
}
