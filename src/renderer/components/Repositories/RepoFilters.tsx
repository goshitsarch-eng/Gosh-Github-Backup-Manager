import { Search, ArrowUp, ArrowDown } from 'lucide-react'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import type { Filters, SortOption, SortDirection } from './RepoList'

interface RepoFiltersProps {
  filters: Filters
  setFilters: (filters: Filters) => void
  sortBy: SortOption
  setSortBy: (sort: SortOption) => void
  sortDirection: SortDirection
  setSortDirection: (direction: SortDirection) => void
  owners: string[]
  languages: string[]
}

export default function RepoFilters({
  filters,
  setFilters,
  sortBy,
  setSortBy,
  sortDirection,
  setSortDirection,
  owners,
  languages,
}: RepoFiltersProps) {
  const updateFilter = <K extends keyof Filters>(key: K, value: Filters[K]) => {
    setFilters({ ...filters, [key]: value })
  }

  const toggleSortDirection = () => {
    setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc')
  }

  return (
    <div className="mb-4 flex flex-wrap items-center gap-3">
      {/* Search */}
      <div className="relative flex-1 min-w-[200px]">
        <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          placeholder="Search repositories..."
          value={filters.search}
          onChange={(e) => updateFilter('search', e.target.value)}
          className="pl-9"
        />
      </div>

      {/* Owner filter */}
      <Select
        value={filters.owner}
        onValueChange={(value) => updateFilter('owner', value)}
      >
        <SelectTrigger className="w-[150px]">
          <SelectValue placeholder="Owner" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="all">All Owners</SelectItem>
          {owners.map((owner) => (
            <SelectItem key={owner} value={owner}>
              {owner}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>

      {/* Visibility filter */}
      <Select
        value={filters.visibility}
        onValueChange={(value) => updateFilter('visibility', value as Filters['visibility'])}
      >
        <SelectTrigger className="w-[130px]">
          <SelectValue placeholder="Visibility" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="all">All</SelectItem>
          <SelectItem value="public">Public</SelectItem>
          <SelectItem value="private">Private</SelectItem>
        </SelectContent>
      </Select>

      {/* Language filter */}
      <Select
        value={filters.language}
        onValueChange={(value) => updateFilter('language', value)}
      >
        <SelectTrigger className="w-[150px]">
          <SelectValue placeholder="Language" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="all">All Languages</SelectItem>
          {languages.map((lang) => (
            <SelectItem key={lang} value={lang}>
              {lang}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>

      {/* Sort */}
      <div className="flex items-center gap-1">
        <Select value={sortBy} onValueChange={(value) => setSortBy(value as SortOption)}>
          <SelectTrigger className="w-[130px]">
            <SelectValue placeholder="Sort by" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="updated">Updated</SelectItem>
            <SelectItem value="name">Name</SelectItem>
            <SelectItem value="stars">Stars</SelectItem>
            <SelectItem value="size">Size</SelectItem>
          </SelectContent>
        </Select>
        <Button variant="outline" size="icon" onClick={toggleSortDirection}>
          {sortDirection === 'asc' ? (
            <ArrowUp className="h-4 w-4" />
          ) : (
            <ArrowDown className="h-4 w-4" />
          )}
        </Button>
      </div>

      {/* Clear filters */}
      {(filters.search || filters.owner !== 'all' || filters.visibility !== 'all' || filters.language !== 'all') && (
        <Button
          variant="ghost"
          size="sm"
          onClick={() =>
            setFilters({
              search: '',
              owner: 'all',
              visibility: 'all',
              language: 'all',
            })
          }
        >
          Clear filters
        </Button>
      )}
    </div>
  )
}
