import { useState, useMemo } from 'react'
import { useNavigate } from 'react-router-dom'
import { useApp } from '@/context/AppContext'
import RepoCard from './RepoCard'
import RepoFilters from './RepoFilters'
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Download } from 'lucide-react'

export type SortOption = 'name' | 'stars' | 'updated' | 'size'
export type SortDirection = 'asc' | 'desc'

export interface Filters {
  search: string
  owner: string
  visibility: 'all' | 'public' | 'private'
  language: string
}

export default function RepoList() {
  const { repos, selectedRepos, toggleRepoSelection, deselectAllRepos } = useApp()
  const navigate = useNavigate()

  const [filters, setFilters] = useState<Filters>({
    search: '',
    owner: 'all',
    visibility: 'all',
    language: 'all',
  })
  const [sortBy, setSortBy] = useState<SortOption>('updated')
  const [sortDirection, setSortDirection] = useState<SortDirection>('desc')

  // Get unique values for filters
  const owners = useMemo(() => {
    const ownerSet = new Set(repos.map((r) => r.owner.login))
    return Array.from(ownerSet).sort()
  }, [repos])

  const languages = useMemo(() => {
    const langSet = new Set(repos.map((r) => r.language).filter(Boolean) as string[])
    return Array.from(langSet).sort()
  }, [repos])

  // Filter and sort repos
  const filteredRepos = useMemo(() => {
    let result = repos.filter((repo) => {
      // Search filter
      if (filters.search) {
        const searchLower = filters.search.toLowerCase()
        if (
          !repo.name.toLowerCase().includes(searchLower) &&
          !repo.description?.toLowerCase().includes(searchLower)
        ) {
          return false
        }
      }

      // Owner filter
      if (filters.owner !== 'all' && repo.owner.login !== filters.owner) {
        return false
      }

      // Visibility filter
      if (filters.visibility !== 'all') {
        if (filters.visibility === 'public' && repo.private) return false
        if (filters.visibility === 'private' && !repo.private) return false
      }

      // Language filter
      if (filters.language !== 'all' && repo.language !== filters.language) {
        return false
      }

      return true
    })

    // Sort
    result.sort((a, b) => {
      let comparison = 0

      switch (sortBy) {
        case 'name':
          comparison = a.name.localeCompare(b.name)
          break
        case 'stars':
          comparison = a.stargazers_count - b.stargazers_count
          break
        case 'updated':
          comparison = new Date(a.updated_at).getTime() - new Date(b.updated_at).getTime()
          break
        case 'size':
          comparison = a.size - b.size
          break
      }

      return sortDirection === 'asc' ? comparison : -comparison
    })

    return result
  }, [repos, filters, sortBy, sortDirection])

  const allSelected = filteredRepos.length > 0 && filteredRepos.every((r) => selectedRepos.has(r.id))
  const someSelected = filteredRepos.some((r) => selectedRepos.has(r.id))

  const handleSelectAll = () => {
    if (allSelected) {
      deselectAllRepos()
    } else {
      // Select only filtered repos
      const filteredIds = new Set(filteredRepos.map((r) => r.id))
      repos.forEach((r) => {
        if (filteredIds.has(r.id) && !selectedRepos.has(r.id)) {
          toggleRepoSelection(r.id)
        }
      })
    }
  }

  const handleStartBackup = () => {
    navigate('/backup')
  }

  return (
    <div className="flex h-full flex-col">
      {/* Header */}
      <div className="mb-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-bold tracking-tight">Repositories</h1>
            <p className="text-muted-foreground">
              {repos.length} repositories total, {filteredRepos.length} shown
            </p>
          </div>
          <Button
            onClick={handleStartBackup}
            disabled={selectedRepos.size === 0}
          >
            <Download className="mr-2 h-4 w-4" />
            Backup Selected ({selectedRepos.size})
          </Button>
        </div>
      </div>

      {/* Filters */}
      <RepoFilters
        filters={filters}
        setFilters={setFilters}
        sortBy={sortBy}
        setSortBy={setSortBy}
        sortDirection={sortDirection}
        setSortDirection={setSortDirection}
        owners={owners}
        languages={languages}
      />

      {/* Select all */}
      <div className="mb-4 flex items-center gap-2">
        <Checkbox
          id="select-all"
          checked={allSelected}
          onCheckedChange={handleSelectAll}
          className={someSelected && !allSelected ? 'data-[state=checked]:bg-primary/50' : ''}
        />
        <label
          htmlFor="select-all"
          className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
        >
          {allSelected ? 'Deselect all' : 'Select all'} ({filteredRepos.length})
        </label>
      </div>

      {/* Repository list */}
      <ScrollArea className="flex-1">
        <div className="grid gap-4 pb-4 md:grid-cols-2 xl:grid-cols-3">
          {filteredRepos.map((repo) => (
            <RepoCard
              key={repo.id}
              repo={repo}
              isSelected={selectedRepos.has(repo.id)}
              onToggleSelect={() => toggleRepoSelection(repo.id)}
            />
          ))}
        </div>

        {filteredRepos.length === 0 && (
          <div className="flex h-64 items-center justify-center text-muted-foreground">
            No repositories match your filters
          </div>
        )}
      </ScrollArea>
    </div>
  )
}
