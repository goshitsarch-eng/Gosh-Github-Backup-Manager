import { GitBranch, Star, GitFork, Lock, Unlock, HardDrive } from 'lucide-react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { formatBytes } from '@/lib/utils'
import Spinner from '@/components/common/Spinner'
import type { GitHubUser, GitHubRepo } from '@shared/types'

interface StatsGridProps {
  user: GitHubUser
  repos: GitHubRepo[]
  stats: { stars: number; forks: number } | null
  isLoading: boolean
}

export default function StatsGrid({ user: _user, repos, stats, isLoading }: StatsGridProps) {
  const publicRepos = repos.filter((r) => !r.private).length
  const privateRepos = repos.filter((r) => r.private).length
  const totalSize = repos.reduce((sum, r) => sum + r.size * 1024, 0) // size is in KB

  const statCards = [
    {
      title: 'Total Repositories',
      value: repos.length,
      icon: GitBranch,
      description: `${publicRepos} public, ${privateRepos} private`,
    },
    {
      title: 'Total Stars',
      value: stats?.stars ?? '...',
      icon: Star,
      description: 'Across all repositories',
      loading: isLoading,
    },
    {
      title: 'Total Forks',
      value: stats?.forks ?? '...',
      icon: GitFork,
      description: 'Across all repositories',
      loading: isLoading,
    },
    {
      title: 'Public Repos',
      value: publicRepos,
      icon: Unlock,
      description: 'Visible to everyone',
    },
    {
      title: 'Private Repos',
      value: privateRepos,
      icon: Lock,
      description: 'Only visible to you',
    },
    {
      title: 'Total Size',
      value: formatBytes(totalSize),
      icon: HardDrive,
      description: 'Combined repository size',
    },
  ]

  return (
    <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
      {statCards.map((stat) => (
        <Card key={stat.title}>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">{stat.title}</CardTitle>
            <stat.icon className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {stat.loading ? <Spinner size="sm" /> : stat.value}
            </div>
            <p className="text-xs text-muted-foreground">{stat.description}</p>
          </CardContent>
        </Card>
      ))}
    </div>
  )
}
