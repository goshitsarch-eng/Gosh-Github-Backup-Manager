import { Star, GitFork, Lock, Unlock, Archive, GitBranch, ExternalLink } from 'lucide-react'
import { Card, CardContent, CardHeader } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Checkbox } from '@/components/ui/checkbox'
import { Button } from '@/components/ui/button'
import { formatBytes, formatRelativeTime } from '@/lib/utils'
import { cn } from '@/lib/utils'
import type { GitHubRepo } from '@shared/types'

interface RepoCardProps {
  repo: GitHubRepo
  isSelected: boolean
  onToggleSelect: () => void
}

// Language colors (subset of GitHub's language colors)
const languageColors: Record<string, string> = {
  JavaScript: '#f1e05a',
  TypeScript: '#3178c6',
  Python: '#3572A5',
  Java: '#b07219',
  Go: '#00ADD8',
  Rust: '#dea584',
  Ruby: '#701516',
  PHP: '#4F5D95',
  'C#': '#178600',
  'C++': '#f34b7d',
  C: '#555555',
  Swift: '#F05138',
  Kotlin: '#A97BFF',
  Dart: '#00B4AB',
  HTML: '#e34c26',
  CSS: '#563d7c',
  Shell: '#89e051',
  Vue: '#41b883',
}

export default function RepoCard({ repo, isSelected, onToggleSelect }: RepoCardProps) {
  const handleOpenRepo = () => {
    window.electronAPI.shell.openExternal(repo.html_url)
  }

  return (
    <Card
      className={cn(
        'cursor-pointer transition-colors hover:bg-muted/50',
        isSelected && 'ring-2 ring-primary'
      )}
      onClick={onToggleSelect}
    >
      <CardHeader className="pb-2">
        <div className="flex items-start justify-between gap-2">
          <div className="flex items-start gap-3">
            <Checkbox
              checked={isSelected}
              onClick={(e) => e.stopPropagation()}
              onCheckedChange={onToggleSelect}
              className="mt-1"
            />
            <div className="min-w-0">
              <div className="flex items-center gap-2">
                <h3 className="truncate font-semibold">{repo.name}</h3>
                {repo.private ? (
                  <Lock className="h-3 w-3 text-muted-foreground" />
                ) : (
                  <Unlock className="h-3 w-3 text-muted-foreground" />
                )}
              </div>
              <p className="text-sm text-muted-foreground">{repo.owner.login}</p>
            </div>
          </div>
          <Button
            variant="ghost"
            size="icon"
            className="h-8 w-8 shrink-0"
            onClick={(e) => {
              e.stopPropagation()
              handleOpenRepo()
            }}
          >
            <ExternalLink className="h-4 w-4" />
          </Button>
        </div>
      </CardHeader>
      <CardContent className="space-y-3">
        {repo.description && (
          <p className="line-clamp-2 text-sm text-muted-foreground">
            {repo.description}
          </p>
        )}

        <div className="flex flex-wrap gap-2">
          {repo.language && (
            <Badge variant="outline" className="gap-1">
              <span
                className="h-2 w-2 rounded-full"
                style={{
                  backgroundColor: languageColors[repo.language] || '#8b8b8b',
                }}
              />
              {repo.language}
            </Badge>
          )}
          {repo.fork && (
            <Badge variant="secondary">
              <GitBranch className="mr-1 h-3 w-3" />
              Fork
            </Badge>
          )}
          {repo.archived && (
            <Badge variant="secondary">
              <Archive className="mr-1 h-3 w-3" />
              Archived
            </Badge>
          )}
        </div>

        <div className="flex items-center justify-between text-sm text-muted-foreground">
          <div className="flex items-center gap-4">
            <span className="flex items-center gap-1">
              <Star className="h-4 w-4" />
              {repo.stargazers_count}
            </span>
            <span className="flex items-center gap-1">
              <GitFork className="h-4 w-4" />
              {repo.forks_count}
            </span>
          </div>
          <span>{formatBytes(repo.size * 1024)}</span>
        </div>

        <div className="text-xs text-muted-foreground">
          Updated {formatRelativeTime(repo.updated_at)}
        </div>
      </CardContent>
    </Card>
  )
}
