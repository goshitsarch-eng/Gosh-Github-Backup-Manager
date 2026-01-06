import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Separator } from '@/components/ui/separator'
import {
  Github,
  Heart,
  Scale,
  Code2,
  ExternalLink,
  Archive
} from 'lucide-react'
import { APP_INFO } from '@shared/constants'
import { open } from '@tauri-apps/plugin-shell'

export default function About() {
  const handleOpenLink = async (url: string) => {
    await open(url)
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">About</h1>
        <p className="text-muted-foreground">Information about this application</p>
      </div>

      <div className="grid gap-6 lg:grid-cols-2">
        {/* App Info */}
        <Card className="lg:col-span-2">
          <CardHeader>
            <div className="flex items-center gap-4">
              <div className="flex h-16 w-16 items-center justify-center rounded-xl bg-primary/10">
                <Archive className="h-8 w-8 text-primary" />
              </div>
              <div>
                <CardTitle className="text-2xl">{APP_INFO.name}</CardTitle>
                <CardDescription className="text-base">
                  {APP_INFO.description}
                </CardDescription>
              </div>
            </div>
          </CardHeader>
          <CardContent>
            <div className="flex flex-wrap gap-4">
              <div className="flex items-center gap-2 rounded-full bg-muted px-4 py-2">
                <Code2 className="h-4 w-4" />
                <span className="text-sm font-medium">Version {APP_INFO.version}</span>
              </div>
              <div className="flex items-center gap-2 rounded-full bg-muted px-4 py-2">
                <Scale className="h-4 w-4" />
                <span className="text-sm font-medium">{APP_INFO.license}</span>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Author & Links */}
        <Card>
          <CardHeader>
            <CardTitle>Author</CardTitle>
            <CardDescription>Created and maintained by</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex items-center gap-3">
              <div className="flex h-10 w-10 items-center justify-center rounded-full bg-muted">
                <Heart className="h-5 w-5 text-primary" />
              </div>
              <div>
                <p className="font-medium">{APP_INFO.author}</p>
                <p className="text-sm text-muted-foreground">Open Source Developer</p>
              </div>
            </div>

            <Separator />

            <Button
              variant="outline"
              className="w-full justify-start"
              onClick={() => handleOpenLink(APP_INFO.github)}
            >
              <Github className="mr-2 h-4 w-4" />
              View on GitHub
              <ExternalLink className="ml-auto h-4 w-4" />
            </Button>
          </CardContent>
        </Card>

        {/* Technology */}
        <Card>
          <CardHeader>
            <CardTitle>Built With</CardTitle>
            <CardDescription>Technologies powering this app</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-2 gap-3">
              {[
                { name: 'Tauri', desc: 'App Framework' },
                { name: 'React', desc: 'UI Library' },
                { name: 'TypeScript', desc: 'Language' },
                { name: 'Rust', desc: 'Backend' },
                { name: 'Tailwind CSS', desc: 'Styling' },
                { name: 'shadcn/ui', desc: 'Components' },
              ].map((tech) => (
                <div
                  key={tech.name}
                  className="rounded-lg border bg-card p-3 text-center"
                >
                  <p className="font-medium">{tech.name}</p>
                  <p className="text-xs text-muted-foreground">{tech.desc}</p>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>

        {/* License */}
        <Card className="lg:col-span-2">
          <CardHeader>
            <CardTitle>License</CardTitle>
            <CardDescription>Open source software license</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <p className="text-sm text-muted-foreground">
              This application is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0).
              This means you are free to use, modify, and distribute this software, provided that any
              modifications or derivative works are also released under the same license.
            </p>
            <Button
              variant="outline"
              onClick={() => handleOpenLink(`${APP_INFO.github}/blob/main/LICENSE`)}
            >
              <Scale className="mr-2 h-4 w-4" />
              View Full License
              <ExternalLink className="ml-auto h-4 w-4" />
            </Button>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
