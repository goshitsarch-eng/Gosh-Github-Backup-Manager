import { useState } from 'react'
import { Archive, Key, ExternalLink, CheckCircle2, AlertCircle } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { useApp } from '@/context/AppContext'
import Spinner from '@/components/common/Spinner'

export default function AuthScreen() {
  const { login, authStatus } = useApp()
  const [token, setToken] = useState('')
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [scopes, setScopes] = useState<string[] | null>(null)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)
    setScopes(null)

    if (!token.trim()) {
      setError('Please enter a Personal Access Token')
      return
    }

    setIsLoading(true)

    try {
      const result = await login(token.trim())
      if (!result.success) {
        setError(result.error || 'Failed to authenticate')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to authenticate')
    } finally {
      setIsLoading(false)
    }
  }

  const openTokenCreationPage = () => {
    window.electronAPI.shell.openExternal(
      'https://github.com/settings/tokens/new?scopes=repo,read:user,read:org&description=Gosh%20Github%20Backup%20Manager'
    )
  }

  const requiredScopes = ['repo', 'read:user', 'read:org']

  return (
    <div className="flex min-h-screen items-center justify-center bg-background p-4">
      <Card className="w-full max-w-md">
        <CardHeader className="text-center">
          <div className="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-primary/10">
            <Archive className="h-8 w-8 text-primary" />
          </div>
          <CardTitle className="text-2xl">Gosh Github Backup Manager</CardTitle>
          <CardDescription>
            Connect your GitHub account to backup your repositories
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit} className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="token">Personal Access Token</Label>
              <div className="relative">
                <Key className="absolute left-3 top-3 h-4 w-4 text-muted-foreground" />
                <Input
                  id="token"
                  type="password"
                  placeholder="ghp_xxxxxxxxxxxx"
                  value={token}
                  onChange={(e) => setToken(e.target.value)}
                  className="pl-10"
                  disabled={isLoading}
                />
              </div>
            </div>

            {error && (
              <div className="flex items-center gap-2 rounded-md bg-destructive/10 p-3 text-sm text-destructive">
                <AlertCircle className="h-4 w-4" />
                {error}
              </div>
            )}

            {scopes && (
              <div className="rounded-md bg-green-500/10 p-3 text-sm">
                <div className="mb-2 flex items-center gap-2 text-green-600 dark:text-green-400">
                  <CheckCircle2 className="h-4 w-4" />
                  Token validated successfully
                </div>
                <div className="text-xs text-muted-foreground">
                  Scopes: {scopes.join(', ')}
                </div>
              </div>
            )}

            <Button type="submit" className="w-full" disabled={isLoading}>
              {isLoading ? (
                <>
                  <Spinner size="sm" className="mr-2" />
                  Connecting...
                </>
              ) : (
                'Connect to GitHub'
              )}
            </Button>

            {isLoading && (
              <div className="flex items-center justify-center gap-3 rounded-md bg-muted/40 px-3 py-2 text-sm text-muted-foreground">
                <span className="relative flex h-2 w-2">
                  <span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-primary/60" />
                  <span className="relative inline-flex h-2 w-2 rounded-full bg-primary" />
                </span>
                <span>{authStatus || 'Signing you in...'}</span>
              </div>
            )}
          </form>

          <div className="mt-6 space-y-4">
            <div className="relative">
              <div className="absolute inset-0 flex items-center">
                <span className="w-full border-t" />
              </div>
              <div className="relative flex justify-center text-xs uppercase">
                <span className="bg-card px-2 text-muted-foreground">
                  Need a token?
                </span>
              </div>
            </div>

            <Button
              variant="outline"
              className="w-full"
              onClick={openTokenCreationPage}
            >
              <ExternalLink className="mr-2 h-4 w-4" />
              Create a new token on GitHub
            </Button>

            <div className="rounded-md bg-muted p-4 text-sm">
              <p className="mb-2 font-medium">Required scopes:</p>
              <ul className="space-y-1 text-muted-foreground">
                {requiredScopes.map((scope) => (
                  <li key={scope} className="flex items-center gap-2">
                    <CheckCircle2 className="h-3 w-3 text-primary" />
                    <code className="text-xs">{scope}</code>
                    <span className="text-xs">
                      {scope === 'repo' && '- Access private repos'}
                      {scope === 'read:user' && '- Read user profile'}
                      {scope === 'read:org' && '- Read org membership'}
                    </span>
                  </li>
                ))}
              </ul>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
