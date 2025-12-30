import { useApp } from '@/context/AppContext'
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Moon, Sun, Monitor, LogOut, RefreshCw } from 'lucide-react'

export default function Header() {
  const { user, theme, setTheme, logout, refreshData } = useApp()

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

  const handleLogout = async () => {
    await logout()
  }

  return (
    <header className="flex h-16 items-center justify-between border-b bg-card px-6 drag-region">
      {/* Left side - can add breadcrumbs or search later */}
      <div className="no-drag">
        <Button
          variant="ghost"
          size="icon"
          onClick={refreshData}
          title="Refresh data"
        >
          <RefreshCw className="h-4 w-4" />
        </Button>
      </div>

      {/* Right side */}
      <div className="flex items-center gap-4 no-drag">
        {/* Theme toggle */}
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="ghost" size="icon">
              {theme === 'light' && <Sun className="h-4 w-4" />}
              {theme === 'dark' && <Moon className="h-4 w-4" />}
              {theme === 'system' && <Monitor className="h-4 w-4" />}
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem onClick={() => setTheme('light')}>
              <Sun className="mr-2 h-4 w-4" />
              Light
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => setTheme('dark')}>
              <Moon className="mr-2 h-4 w-4" />
              Dark
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => setTheme('system')}>
              <Monitor className="mr-2 h-4 w-4" />
              System
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>

        {/* User menu */}
        {user && (
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="ghost" className="relative h-10 w-10 rounded-full">
                <Avatar>
                  <AvatarImage src={user.avatar_url} alt={user.login} />
                  <AvatarFallback>
                    {getInitials(user.name, user.login)}
                  </AvatarFallback>
                </Avatar>
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" className="w-56">
              <DropdownMenuLabel>
                <div className="flex flex-col space-y-1">
                  <p className="text-sm font-medium">{user.name || user.login}</p>
                  <p className="text-xs text-muted-foreground">@{user.login}</p>
                </div>
              </DropdownMenuLabel>
              <DropdownMenuSeparator />
              <DropdownMenuItem
                className="text-destructive focus:text-destructive"
                onClick={handleLogout}
              >
                <LogOut className="mr-2 h-4 w-4" />
                Log out
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        )}
      </div>
    </header>
  )
}
