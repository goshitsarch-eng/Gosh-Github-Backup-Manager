import { useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import { useApp } from '@/context/AppContext'
import { useGitHub } from '@/hooks/useGitHub'
import ProfileCard from './ProfileCard'
import StatsGrid from './StatsGrid'
import RecentActivity from './RecentActivity'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Download, GitBranch, Settings } from 'lucide-react'

export default function Dashboard() {
  const { user, repos, orgs } = useApp()
  const { events, stats, fetchEvents, fetchStats, isLoadingEvents, isLoadingStats } = useGitHub()
  const navigate = useNavigate()

  useEffect(() => {
    fetchEvents(20)
    fetchStats()
  }, [fetchEvents, fetchStats])

  if (!user) {
    return null
  }

  return (
    <div className="space-y-6">
      {/* Page title */}
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Dashboard</h1>
        <p className="text-muted-foreground">
          Welcome back, {user.name || user.login}
        </p>
      </div>

      {/* Main content grid */}
      <div className="grid gap-6 lg:grid-cols-3">
        {/* Left column - Profile and Quick Actions */}
        <div className="space-y-6">
          <ProfileCard user={user} orgs={orgs} />

          {/* Quick Actions */}
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">Quick Actions</CardTitle>
            </CardHeader>
            <CardContent className="space-y-2">
              <Button
                className="w-full justify-start"
                onClick={() => navigate('/backup')}
              >
                <Download className="mr-2 h-4 w-4" />
                Start Backup
              </Button>
              <Button
                variant="outline"
                className="w-full justify-start"
                onClick={() => navigate('/repositories')}
              >
                <GitBranch className="mr-2 h-4 w-4" />
                View Repositories
              </Button>
              <Button
                variant="outline"
                className="w-full justify-start"
                onClick={() => navigate('/settings')}
              >
                <Settings className="mr-2 h-4 w-4" />
                Settings
              </Button>
            </CardContent>
          </Card>
        </div>

        {/* Right column - Stats and Activity */}
        <div className="space-y-6 lg:col-span-2">
          <StatsGrid
            user={user}
            repos={repos}
            stats={stats}
            isLoading={isLoadingStats}
          />

          <RecentActivity
            events={events}
            isLoading={isLoadingEvents}
          />
        </div>
      </div>
    </div>
  )
}
