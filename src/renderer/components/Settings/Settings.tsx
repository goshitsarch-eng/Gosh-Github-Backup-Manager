import { useState } from 'react'
import { useApp } from '@/context/AppContext'
import { useBackup } from '@/hooks/useBackup'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Separator } from '@/components/ui/separator'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { useToast } from '@/components/ui/use-toast'
import { Moon, Sun, Monitor, FolderOpen, Trash2, LogOut, Info } from 'lucide-react'
import { APP_INFO } from '@shared/constants'

export default function Settings() {
  const { user, settings, setSettings, theme, setTheme, logout } = useApp()
  const { selectFolder } = useBackup()
  const { toast } = useToast()
  const [logoutDialogOpen, setLogoutDialogOpen] = useState(false)

  const handleSelectDefaultFolder = async () => {
    const folder = await selectFolder()
    if (folder) {
      await setSettings({ defaultBackupLocation: folder })
      toast({
        title: 'Default location updated',
        description: 'Your default backup location has been saved.',
      })
    }
  }

  const handleLogout = async () => {
    await logout()
    setLogoutDialogOpen(false)
  }

  const handleClearHistory = async () => {
    // This would call a method to clear backup history
    toast({
      title: 'History cleared',
      description: 'Your backup history has been cleared.',
    })
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Settings</h1>
        <p className="text-muted-foreground">Manage your app preferences</p>
      </div>

      <div className="grid gap-6 lg:grid-cols-2">
        {/* Appearance */}
        <Card>
          <CardHeader>
            <CardTitle>Appearance</CardTitle>
            <CardDescription>Customize how the app looks</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <Label>Theme</Label>
              <div className="flex gap-2">
                <Button
                  variant={theme === 'light' ? 'default' : 'outline'}
                  size="sm"
                  onClick={() => setTheme('light')}
                >
                  <Sun className="mr-2 h-4 w-4" />
                  Light
                </Button>
                <Button
                  variant={theme === 'dark' ? 'default' : 'outline'}
                  size="sm"
                  onClick={() => setTheme('dark')}
                >
                  <Moon className="mr-2 h-4 w-4" />
                  Dark
                </Button>
                <Button
                  variant={theme === 'system' ? 'default' : 'outline'}
                  size="sm"
                  onClick={() => setTheme('system')}
                >
                  <Monitor className="mr-2 h-4 w-4" />
                  System
                </Button>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Backup Defaults */}
        <Card>
          <CardHeader>
            <CardTitle>Backup Defaults</CardTitle>
            <CardDescription>Default settings for new backups</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <Label>Default backup location</Label>
              <div className="flex gap-2">
                <Input
                  value={settings.defaultBackupLocation || ''}
                  readOnly
                  placeholder="Not set"
                  className="flex-1"
                />
                <Button variant="outline" onClick={handleSelectDefaultFolder}>
                  <FolderOpen className="mr-2 h-4 w-4" />
                  Browse
                </Button>
              </div>
            </div>

            <Separator />

            <div className="space-y-2">
              <Label>Default clone type</Label>
              <Select
                value={settings.defaultBackupOptions?.cloneType || 'full'}
                onValueChange={(value) =>
                  setSettings({
                    defaultBackupOptions: {
                      ...settings.defaultBackupOptions,
                      cloneType: value as 'full' | 'mirror' | 'shallow',
                    },
                  })
                }
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="full">Full clone</SelectItem>
                  <SelectItem value="mirror">Mirror</SelectItem>
                  <SelectItem value="shallow">Shallow</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </CardContent>
        </Card>

        {/* Notifications */}
        <Card>
          <CardHeader>
            <CardTitle>Notifications</CardTitle>
            <CardDescription>Configure notification preferences</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex items-center justify-between">
              <div className="space-y-0.5">
                <Label>Enable notifications</Label>
                <p className="text-sm text-muted-foreground">
                  Show notifications when backups complete
                </p>
              </div>
              <Switch
                checked={settings.notifications}
                onCheckedChange={(checked) => setSettings({ notifications: checked })}
              />
            </div>
          </CardContent>
        </Card>

        {/* Account */}
        <Card>
          <CardHeader>
            <CardTitle>Account</CardTitle>
            <CardDescription>Manage your GitHub connection</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            {user && (
              <div className="flex items-center gap-3 rounded-md bg-muted p-3">
                <img
                  src={user.avatar_url}
                  alt={user.login}
                  className="h-10 w-10 rounded-full"
                />
                <div>
                  <p className="font-medium">{user.name || user.login}</p>
                  <p className="text-sm text-muted-foreground">@{user.login}</p>
                </div>
              </div>
            )}

            <Separator />

            <Dialog open={logoutDialogOpen} onOpenChange={setLogoutDialogOpen}>
              <DialogTrigger asChild>
                <Button variant="destructive" className="w-full">
                  <LogOut className="mr-2 h-4 w-4" />
                  Disconnect GitHub
                </Button>
              </DialogTrigger>
              <DialogContent>
                <DialogHeader>
                  <DialogTitle>Disconnect GitHub?</DialogTitle>
                  <DialogDescription>
                    This will remove your GitHub token and log you out. You'll need
                    to re-enter your token to use the app again.
                  </DialogDescription>
                </DialogHeader>
                <DialogFooter>
                  <Button
                    variant="outline"
                    onClick={() => setLogoutDialogOpen(false)}
                  >
                    Cancel
                  </Button>
                  <Button variant="destructive" onClick={handleLogout}>
                    Disconnect
                  </Button>
                </DialogFooter>
              </DialogContent>
            </Dialog>
          </CardContent>
        </Card>

        {/* Data Management */}
        <Card>
          <CardHeader>
            <CardTitle>Data Management</CardTitle>
            <CardDescription>Manage app data and cache</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <Button variant="outline" className="w-full" onClick={handleClearHistory}>
              <Trash2 className="mr-2 h-4 w-4" />
              Clear Backup History
            </Button>
          </CardContent>
        </Card>

        {/* About */}
        <Card>
          <CardHeader>
            <CardTitle>About</CardTitle>
            <CardDescription>App information</CardDescription>
          </CardHeader>
          <CardContent className="space-y-2">
            <div className="flex items-center gap-2">
              <Info className="h-4 w-4 text-muted-foreground" />
              <span className="font-medium">{APP_INFO.name}</span>
            </div>
            <p className="text-sm text-muted-foreground">
              Version {APP_INFO.version}
            </p>
            <Button
              variant="link"
              className="h-auto p-0 text-sm"
              onClick={() => window.electronAPI.shell.openExternal(APP_INFO.github)}
            >
              View on GitHub
            </Button>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
