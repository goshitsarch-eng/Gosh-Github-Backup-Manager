import { useState, useCallback } from 'react'
import type { GitHubEvent } from '../../shared/types'

interface UseGitHubReturn {
  events: GitHubEvent[]
  stats: { stars: number; forks: number } | null
  isLoadingEvents: boolean
  isLoadingStats: boolean
  fetchEvents: (limit?: number) => Promise<void>
  fetchStats: () => Promise<void>
  error: string | null
}

export function useGitHub(): UseGitHubReturn {
  const [events, setEvents] = useState<GitHubEvent[]>([])
  const [stats, setStats] = useState<{ stars: number; forks: number } | null>(null)
  const [isLoadingEvents, setIsLoadingEvents] = useState(false)
  const [isLoadingStats, setIsLoadingStats] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const fetchEvents = useCallback(async (limit = 20) => {
    setIsLoadingEvents(true)
    setError(null)

    try {
      const result = await window.electronAPI.github.getEvents(limit)
      if (result.success && result.data) {
        setEvents(result.data)
      } else {
        setError(result.error || 'Failed to fetch events')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch events')
    } finally {
      setIsLoadingEvents(false)
    }
  }, [])

  const fetchStats = useCallback(async () => {
    setIsLoadingStats(true)
    setError(null)

    try {
      const result = await window.electronAPI.github.getStats()
      if (result.success && result.data) {
        setStats(result.data)
      } else {
        setError(result.error || 'Failed to fetch stats')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch stats')
    } finally {
      setIsLoadingStats(false)
    }
  }, [])

  return {
    events,
    stats,
    isLoadingEvents,
    isLoadingStats,
    fetchEvents,
    fetchStats,
    error,
  }
}
