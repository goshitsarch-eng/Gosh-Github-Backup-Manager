import { NavLink } from 'react-router-dom'
import {
  LayoutDashboard,
  GitBranch,
  Download,
  Settings,
  Archive,
  Info,
} from 'lucide-react'
import { cn } from '@/lib/utils'
import { APP_INFO } from '@shared/constants'

const navItems = [
  { to: '/', label: 'Dashboard', icon: LayoutDashboard },
  { to: '/repositories', label: 'Repositories', icon: GitBranch },
  { to: '/backup', label: 'Backup', icon: Download },
  { to: '/settings', label: 'Settings', icon: Settings },
  { to: '/about', label: 'About', icon: Info },
]

export default function Sidebar() {
  return (
    <aside className="flex w-64 flex-col border-r bg-card">
      {/* App Logo/Title - extra left padding for macOS traffic lights */}
      <div className="flex h-16 items-center gap-2 border-b pl-20 pr-6 drag-region">
        <Archive className="h-6 w-6 text-primary no-drag" />
        <span className="font-semibold no-drag">Gosh Github Backup Manager</span>
      </div>

      {/* Navigation */}
      <nav className="flex-1 space-y-1 p-4">
        {navItems.map(({ to, label, icon: Icon }) => (
          <NavLink
            key={to}
            to={to}
            className={({ isActive }) =>
              cn(
                'flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors',
                isActive
                  ? 'bg-primary text-primary-foreground'
                  : 'text-muted-foreground hover:bg-accent hover:text-accent-foreground'
              )
            }
          >
            <Icon className="h-5 w-5" />
            {label}
          </NavLink>
        ))}
      </nav>

      {/* Footer */}
      <div className="border-t p-4">
        <p className="text-xs text-muted-foreground">
          {APP_INFO.name} v{APP_INFO.version}
        </p>
      </div>
    </aside>
  )
}
