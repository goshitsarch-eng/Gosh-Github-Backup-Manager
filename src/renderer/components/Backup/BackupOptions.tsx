import { FolderOpen } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch'
import { Slider } from '@/components/ui/slider'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Separator } from '@/components/ui/separator'
import { formatBytes } from '@/lib/utils'
import type { BackupOptions as BackupOptionsType, GitHubRepo } from '@shared/types'

interface BackupOptionsProps {
  options: BackupOptionsType
  setOptions: (options: BackupOptionsType) => void
  onSelectFolder: () => void
  selectedRepos: GitHubRepo[]
}

export default function BackupOptions({
  options,
  setOptions,
  onSelectFolder,
  selectedRepos,
}: BackupOptionsProps) {
  const updateOption = <K extends keyof BackupOptionsType>(
    key: K,
    value: BackupOptionsType[K]
  ) => {
    setOptions({ ...options, [key]: value })
  }

  const totalSize = selectedRepos.reduce((sum, r) => sum + r.size * 1024, 0)
  const forksCount = selectedRepos.filter((r) => r.fork).length
  const archivedCount = selectedRepos.filter((r) => r.archived).length

  return (
    <div className="grid gap-6 lg:grid-cols-2">
      {/* Destination */}
      <Card>
        <CardHeader>
          <CardTitle>Destination</CardTitle>
          <CardDescription>Choose where to save your backups</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center gap-2">
            <Input
              value={options.destination}
              readOnly
              placeholder="Select a folder..."
              className="flex-1"
            />
            <Button variant="outline" onClick={onSelectFolder}>
              <FolderOpen className="mr-2 h-4 w-4" />
              Browse
            </Button>
          </div>

          <div className="space-y-2">
            <Label>Organize by</Label>
            <Select
              value={options.splitBy}
              onValueChange={(value) =>
                updateOption('splitBy', value as BackupOptionsType['splitBy'])
              }
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="none">Single folder</SelectItem>
                <SelectItem value="owner">By owner/organization</SelectItem>
                <SelectItem value="visibility">By visibility</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      {/* Clone Settings */}
      <Card>
        <CardHeader>
          <CardTitle>Clone Settings</CardTitle>
          <CardDescription>Configure how repositories are cloned</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label>Clone type</Label>
            <Select
              value={options.cloneType}
              onValueChange={(value) =>
                updateOption('cloneType', value as BackupOptionsType['cloneType'])
              }
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="full">Full clone (with history)</SelectItem>
                <SelectItem value="mirror">Mirror (complete backup)</SelectItem>
                <SelectItem value="shallow">Shallow (latest only)</SelectItem>
              </SelectContent>
            </Select>
            <p className="text-xs text-muted-foreground">
              {options.cloneType === 'full' && 'Includes complete git history. Best for development.'}
              {options.cloneType === 'mirror' && 'Complete backup including all refs. Best for archival.'}
              {options.cloneType === 'shallow' && 'Only latest commit. Fastest and smallest.'}
            </p>
          </div>

          <div className="space-y-2">
            <Label>Concurrent downloads</Label>
            <div className="flex items-center gap-4">
              <Slider
                value={[options.maxConcurrent]}
                onValueChange={([value]) => updateOption('maxConcurrent', value)}
                min={1}
                max={5}
                step={1}
                className="flex-1"
              />
              <span className="w-8 text-center text-sm font-medium">
                {options.maxConcurrent}
              </span>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Filters */}
      <Card>
        <CardHeader>
          <CardTitle>Include/Exclude</CardTitle>
          <CardDescription>Filter which repositories to backup</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-between">
            <div className="space-y-0.5">
              <Label>Include forks</Label>
              <p className="text-xs text-muted-foreground">
                {forksCount} fork{forksCount !== 1 ? 's' : ''} selected
              </p>
            </div>
            <Switch
              checked={options.includeForks}
              onCheckedChange={(checked) => updateOption('includeForks', checked)}
            />
          </div>

          <Separator />

          <div className="flex items-center justify-between">
            <div className="space-y-0.5">
              <Label>Include archived</Label>
              <p className="text-xs text-muted-foreground">
                {archivedCount} archived repo{archivedCount !== 1 ? 's' : ''} selected
              </p>
            </div>
            <Switch
              checked={options.includeArchived}
              onCheckedChange={(checked) => updateOption('includeArchived', checked)}
            />
          </div>
        </CardContent>
      </Card>

      {/* Archive Options */}
      <Card>
        <CardHeader>
          <CardTitle>Archive Options</CardTitle>
          <CardDescription>Create a zip archive after backup</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-between">
            <div className="space-y-0.5">
              <Label>Create zip archive</Label>
              <p className="text-xs text-muted-foreground">
                Compress backup into a single zip file
              </p>
            </div>
            <Switch
              checked={options.createZip}
              onCheckedChange={(checked) => updateOption('createZip', checked)}
            />
          </div>

          {options.createZip && (
            <>
              <Separator />
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <Label>Compression level</Label>
                  <span className="text-sm font-medium">{options.zipCompression}</span>
                </div>
                <Slider
                  value={[options.zipCompression]}
                  onValueChange={([value]) => updateOption('zipCompression', value)}
                  min={1}
                  max={9}
                  step={1}
                />
                <p className="text-xs text-muted-foreground">
                  1 = fastest, 9 = smallest file size
                </p>
              </div>
            </>
          )}
        </CardContent>
      </Card>

      {/* Summary */}
      <Card className="lg:col-span-2">
        <CardHeader>
          <CardTitle>Summary</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid gap-4 sm:grid-cols-3">
            <div>
              <p className="text-sm text-muted-foreground">Repositories</p>
              <p className="text-2xl font-bold">{selectedRepos.length}</p>
            </div>
            <div>
              <p className="text-sm text-muted-foreground">Estimated size</p>
              <p className="text-2xl font-bold">{formatBytes(totalSize)}</p>
            </div>
            <div>
              <p className="text-sm text-muted-foreground">Destination</p>
              <p className="truncate text-sm font-medium">
                {options.destination || 'Not selected'}
              </p>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
