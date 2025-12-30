import { FolderOpen, CheckCircle2, AlertCircle, XCircle } from 'lucide-react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import { formatBytes, formatDate, formatDuration } from '@/lib/utils'
import type { BackupHistoryEntry } from '@shared/types'

interface BackupHistoryProps {
  history: BackupHistoryEntry[]
  onOpenFolder: (path: string) => void
}

const statusConfig = {
  complete: {
    icon: CheckCircle2,
    color: 'text-green-500',
    badge: 'success' as const,
    label: 'Complete',
  },
  partial: {
    icon: AlertCircle,
    color: 'text-yellow-500',
    badge: 'warning' as const,
    label: 'Partial',
  },
  failed: {
    icon: XCircle,
    color: 'text-red-500',
    badge: 'destructive' as const,
    label: 'Failed',
  },
}

export default function BackupHistory({ history, onOpenFolder }: BackupHistoryProps) {
  if (history.length === 0) {
    return (
      <Card>
        <CardContent className="flex h-64 items-center justify-center text-muted-foreground">
          No backup history yet. Complete a backup to see it here.
        </CardContent>
      </Card>
    )
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Backup History</CardTitle>
        <CardDescription>Your recent backup operations</CardDescription>
      </CardHeader>
      <CardContent className="p-0">
        <ScrollArea className="h-[500px]">
          <div className="divide-y">
            {history.map((entry) => {
              const config = statusConfig[entry.status]
              const Icon = config.icon

              return (
                <div key={entry.id} className="p-4">
                  <div className="flex items-start justify-between gap-4">
                    <div className="flex items-start gap-3 min-w-0">
                      <Icon className={`h-5 w-5 shrink-0 mt-0.5 ${config.color}`} />
                      <div className="min-w-0 space-y-1">
                        <div className="flex items-center gap-2">
                          <p className="font-medium">{formatDate(entry.date)}</p>
                          <Badge variant={config.badge}>{config.label}</Badge>
                        </div>
                        <p className="text-sm text-muted-foreground truncate">
                          {entry.destination}
                        </p>
                        <div className="flex flex-wrap gap-4 text-sm text-muted-foreground">
                          <span>{entry.repoCount} repositories</span>
                          <span>{formatBytes(entry.totalSize)}</span>
                          <span>{formatDuration(entry.duration)}</span>
                        </div>
                        {entry.failedRepos.length > 0 && (
                          <p className="text-sm text-red-500">
                            Failed: {entry.failedRepos.join(', ')}
                          </p>
                        )}
                      </div>
                    </div>
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => onOpenFolder(entry.destination)}
                    >
                      <FolderOpen className="mr-2 h-4 w-4" />
                      Open
                    </Button>
                  </div>
                </div>
              )
            })}
          </div>
        </ScrollArea>
      </CardContent>
    </Card>
  )
}
