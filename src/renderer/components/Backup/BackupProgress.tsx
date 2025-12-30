import { CheckCircle2, XCircle, Clock, Loader2 } from 'lucide-react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Progress } from '@/components/ui/progress'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Badge } from '@/components/ui/badge'
import { formatDuration } from '@/lib/utils'
import type { BackupProgress as BackupProgressType, RepoBackupStatus } from '@shared/types'

interface BackupProgressProps {
  progress: BackupProgressType | null
  isRunning: boolean
}

const statusConfig: Record<
  RepoBackupStatus,
  { icon: typeof CheckCircle2; color: string; label: string }
> = {
  pending: { icon: Clock, color: 'text-muted-foreground', label: 'Pending' },
  cloning: { icon: Loader2, color: 'text-blue-500', label: 'Cloning' },
  pulling: { icon: Loader2, color: 'text-blue-500', label: 'Pulling' },
  complete: { icon: CheckCircle2, color: 'text-green-500', label: 'Complete' },
  failed: { icon: XCircle, color: 'text-red-500', label: 'Failed' },
  skipped: { icon: Clock, color: 'text-yellow-500', label: 'Skipped' },
}

export default function BackupProgress({ progress, isRunning }: BackupProgressProps) {
  if (!progress) {
    return (
      <Card>
        <CardContent className="flex h-64 items-center justify-center text-muted-foreground">
          No backup in progress. Configure options and start a backup.
        </CardContent>
      </Card>
    )
  }

  const overallProgress =
    progress.totalRepos > 0
      ? Math.round(((progress.completedRepos + progress.failedRepos) / progress.totalRepos) * 100)
      : 0

  const elapsed = Date.now() - progress.startTime
  const estimatedTotal =
    overallProgress > 0 ? (elapsed / overallProgress) * 100 : 0
  const estimatedRemaining = estimatedTotal - elapsed

  return (
    <div className="space-y-6">
      {/* Overall Progress */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center justify-between">
            <span>Overall Progress</span>
            {isRunning && (
              <Badge variant="outline" className="animate-pulse">
                <Loader2 className="mr-1 h-3 w-3 animate-spin" />
                Running
              </Badge>
            )}
            {!isRunning && progress.completedRepos > 0 && (
              <Badge variant="success">Complete</Badge>
            )}
          </CardTitle>
          <CardDescription>
            {progress.completedRepos} of {progress.totalRepos} repositories backed up
            {progress.failedRepos > 0 && ` (${progress.failedRepos} failed)`}
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <Progress value={overallProgress} className="h-3" />

          <div className="grid gap-4 sm:grid-cols-3">
            <div>
              <p className="text-sm text-muted-foreground">Completed</p>
              <p className="text-2xl font-bold text-green-500">
                {progress.completedRepos}
              </p>
            </div>
            <div>
              <p className="text-sm text-muted-foreground">Failed</p>
              <p className="text-2xl font-bold text-red-500">
                {progress.failedRepos}
              </p>
            </div>
            <div>
              <p className="text-sm text-muted-foreground">Remaining</p>
              <p className="text-2xl font-bold">
                {progress.totalRepos - progress.completedRepos - progress.failedRepos}
              </p>
            </div>
          </div>

          <div className="flex items-center justify-between text-sm text-muted-foreground">
            <span>Elapsed: {formatDuration(elapsed)}</span>
            {isRunning && estimatedRemaining > 0 && (
              <span>Est. remaining: {formatDuration(estimatedRemaining)}</span>
            )}
          </div>

          {progress.currentRepo && (
            <div className="rounded-md bg-muted p-3">
              <p className="text-sm font-medium">Currently processing:</p>
              <p className="text-sm text-muted-foreground">{progress.currentRepo}</p>
            </div>
          )}
        </CardContent>
      </Card>

      {/* Repository Status List */}
      <Card>
        <CardHeader>
          <CardTitle>Repository Status</CardTitle>
        </CardHeader>
        <CardContent className="p-0">
          <ScrollArea className="h-80">
            <div className="divide-y">
              {progress.repos.map((repo) => {
                const config = statusConfig[repo.status]
                const Icon = config.icon

                return (
                  <div
                    key={repo.repoId}
                    className="flex items-center justify-between p-4"
                  >
                    <div className="flex items-center gap-3 min-w-0">
                      <Icon
                        className={`h-5 w-5 shrink-0 ${config.color} ${
                          repo.status === 'cloning' || repo.status === 'pulling'
                            ? 'animate-spin'
                            : ''
                        }`}
                      />
                      <div className="min-w-0">
                        <p className="truncate font-medium">{repo.repoName}</p>
                        {repo.error && (
                          <p className="text-sm text-red-500">{repo.error}</p>
                        )}
                      </div>
                    </div>
                    <div className="flex items-center gap-3 shrink-0">
                      {repo.progress !== undefined && repo.progress < 100 && (
                        <span className="text-sm text-muted-foreground">
                          {repo.progress}%
                        </span>
                      )}
                      <Badge
                        variant={
                          repo.status === 'complete'
                            ? 'success'
                            : repo.status === 'failed'
                            ? 'destructive'
                            : 'secondary'
                        }
                      >
                        {config.label}
                      </Badge>
                    </div>
                  </div>
                )
              })}
            </div>
          </ScrollArea>
        </CardContent>
      </Card>
    </div>
  )
}
