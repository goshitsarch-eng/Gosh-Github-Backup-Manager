import { MapPin, Building2, Link as LinkIcon, Users } from 'lucide-react'
import { Card, CardContent } from '@/components/ui/card'
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import type { GitHubUser, GitHubOrg } from '@shared/types'

interface ProfileCardProps {
  user: GitHubUser
  orgs: GitHubOrg[]
}

export default function ProfileCard({ user, orgs }: ProfileCardProps) {
  const getInitials = (name: string | null, login: string) => {
    if (name) {
      return name
        .split(' ')
        .map((n) => n[0])
        .join('')
        .toUpperCase()
        .slice(0, 2)
    }
    return login.slice(0, 2).toUpperCase()
  }

  return (
    <Card>
      <CardContent className="pt-6">
        <div className="flex flex-col items-center text-center">
          <Avatar className="h-24 w-24">
            <AvatarImage src={user.avatar_url} alt={user.login} />
            <AvatarFallback className="text-2xl">
              {getInitials(user.name, user.login)}
            </AvatarFallback>
          </Avatar>

          <h2 className="mt-4 text-xl font-semibold">
            {user.name || user.login}
          </h2>
          <p className="text-muted-foreground">@{user.login}</p>

          {user.bio && (
            <p className="mt-2 text-sm text-muted-foreground">{user.bio}</p>
          )}

          <div className="mt-4 flex flex-wrap items-center justify-center gap-3 text-sm text-muted-foreground">
            {user.location && (
              <span className="flex items-center gap-1">
                <MapPin className="h-4 w-4" />
                {user.location}
              </span>
            )}
            {user.company && (
              <span className="flex items-center gap-1">
                <Building2 className="h-4 w-4" />
                {user.company}
              </span>
            )}
            {user.blog && (
              <a
                href={user.blog.startsWith('http') ? user.blog : `https://${user.blog}`}
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center gap-1 hover:text-primary"
                onClick={(e) => {
                  e.preventDefault()
                  window.electronAPI.shell.openExternal(
                    user.blog!.startsWith('http') ? user.blog! : `https://${user.blog}`
                  )
                }}
              >
                <LinkIcon className="h-4 w-4" />
                Website
              </a>
            )}
          </div>

          <div className="mt-4 flex items-center gap-4 text-sm">
            <span className="flex items-center gap-1">
              <Users className="h-4 w-4" />
              <strong>{user.followers}</strong> followers
            </span>
            <span>
              <strong>{user.following}</strong> following
            </span>
          </div>

          {/* Organizations */}
          {orgs.length > 0 && (
            <div className="mt-4 w-full">
              <p className="mb-2 text-sm font-medium">Organizations</p>
              <div className="flex flex-wrap justify-center gap-2">
                {orgs.slice(0, 5).map((org) => (
                  <Badge key={org.id} variant="secondary" className="gap-1">
                    <img
                      src={org.avatar_url}
                      alt={org.login}
                      className="h-4 w-4 rounded-full"
                    />
                    {org.login}
                  </Badge>
                ))}
                {orgs.length > 5 && (
                  <Badge variant="outline">+{orgs.length - 5} more</Badge>
                )}
              </div>
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  )
}
