import {
  GitCommit,
  GitPullRequest,
  Star,
  GitFork,
  MessageSquare,
  Eye,
  Plus,
  Trash2,
} from 'lucide-react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { ScrollArea } from '@/components/ui/scroll-area'
import { formatRelativeTime } from '@/lib/utils'
import Spinner from '@/components/common/Spinner'
import type { GitHubEvent } from '@shared/types'

interface RecentActivityProps {
  events: GitHubEvent[]
  isLoading: boolean
}

const eventIcons: Record<string, typeof GitCommit> = {
  PushEvent: GitCommit,
  PullRequestEvent: GitPullRequest,
  WatchEvent: Star,
  ForkEvent: GitFork,
  IssueCommentEvent: MessageSquare,
  IssuesEvent: MessageSquare,
  CreateEvent: Plus,
  DeleteEvent: Trash2,
  PublicEvent: Eye,
}

const eventDescriptions: Record<string, (payload: Record<string, unknown>) => string> = {
  PushEvent: (payload) => {
    const commits = payload.commits as { message: string }[] | undefined
    const count = commits?.length || 0
    return `pushed ${count} commit${count === 1 ? '' : 's'}`
  },
  PullRequestEvent: (payload) => {
    const action = payload.action as string
    return `${action} a pull request`
  },
  WatchEvent: () => 'starred',
  ForkEvent: () => 'forked',
  IssueCommentEvent: () => 'commented on an issue',
  IssuesEvent: (payload) => {
    const action = payload.action as string
    return `${action} an issue`
  },
  CreateEvent: (payload) => {
    const refType = payload.ref_type as string
    return `created a ${refType}`
  },
  DeleteEvent: (payload) => {
    const refType = payload.ref_type as string
    return `deleted a ${refType}`
  },
  PublicEvent: () => 'made repository public',
}

export default function RecentActivity({ events, isLoading }: RecentActivityProps) {
  if (isLoading) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Recent Activity</CardTitle>
        </CardHeader>
        <CardContent className="flex h-64 items-center justify-center">
          <Spinner />
        </CardContent>
      </Card>
    )
  }

  if (events.length === 0) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Recent Activity</CardTitle>
        </CardHeader>
        <CardContent className="flex h-64 items-center justify-center text-muted-foreground">
          No recent activity
        </CardContent>
      </Card>
    )
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Recent Activity</CardTitle>
      </CardHeader>
      <CardContent className="p-0">
        <ScrollArea className="h-80">
          <div className="space-y-1 p-4 pt-0">
            {events.map((event) => {
              const Icon = eventIcons[event.type] || GitCommit
              const getDescription = eventDescriptions[event.type]
              const description = getDescription
                ? getDescription(event.payload)
                : event.type.replace('Event', '')

              return (
                <div
                  key={event.id}
                  className="flex items-start gap-3 rounded-lg p-2 hover:bg-muted/50"
                >
                  <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-muted">
                    <Icon className="h-4 w-4" />
                  </div>
                  <div className="min-w-0 flex-1">
                    <p className="text-sm">
                      <span className="font-medium">{event.actor.login}</span>{' '}
                      <span className="text-muted-foreground">{description}</span>{' '}
                      <span className="font-medium">{event.repo.name}</span>
                    </p>
                    <p className="text-xs text-muted-foreground">
                      {formatRelativeTime(event.created_at)}
                    </p>
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
