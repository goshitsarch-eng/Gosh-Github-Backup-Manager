import { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import { useApp } from '@/context/AppContext'
import { useBackup, getDefaultBackupOptions } from '@/hooks/useBackup'
import BackupOptions from './BackupOptions'
import BackupProgress from './BackupProgress'
import BackupHistory from './BackupHistory'
import { Button } from '@/components/ui/button'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { ArrowLeft, Play, FolderOpen } from 'lucide-react'
import { toast } from '@/components/ui/use-toast'
import type { BackupOptions as BackupOptionsType } from '@shared/types'

export default function BackupManager() {
  const { repos, selectedRepos, settings } = useApp()
  const { progress, history, isRunning, startBackup, cancelBackup, fetchHistory, selectFolder, openFolder } = useBackup()
  const navigate = useNavigate()

  const [options, setOptions] = useState<BackupOptionsType>(
    getDefaultBackupOptions(settings.defaultBackupOptions)
  )
  const [activeTab, setActiveTab] = useState('options')

  useEffect(() => {
    fetchHistory()
  }, [fetchHistory])

  // Switch to progress tab when backup starts
  useEffect(() => {
    if (isRunning) {
      setActiveTab('progress')
    }
  }, [isRunning])

  // Refresh history when backup completes
  useEffect(() => {
    const unsubscribe = window.electronAPI.backup.onComplete(() => {
      fetchHistory()
    })
    return unsubscribe
  }, [fetchHistory])

  const selectedReposList = repos.filter((r) => selectedRepos.has(r.id))

  const handleSelectFolder = async () => {
    const folder = await selectFolder()
    if (folder) {
      setOptions({ ...options, destination: folder })
    }
  }

  const handleStartBackup = async () => {
    if (!options.destination) {
      toast({
        title: 'No destination selected',
        description: 'Please select a backup destination folder',
        variant: 'destructive',
      })
      return
    }

    if (selectedReposList.length === 0) {
      toast({
        title: 'No repositories selected',
        description: 'Please select at least one repository to backup',
        variant: 'destructive',
      })
      return
    }

    const result = await startBackup(
      selectedReposList.map((r) => r.id),
      options
    )

    if (!result.success) {
      toast({
        title: 'Failed to start backup',
        description: result.error,
        variant: 'destructive',
      })
    }
  }

  const handleCancelBackup = async () => {
    await cancelBackup()
    toast({
      title: 'Backup cancelled',
      description: 'The backup process has been cancelled',
    })
  }

  const handleOpenBackupFolder = () => {
    if (options.destination) {
      openFolder(options.destination)
    }
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <Button variant="ghost" size="icon" onClick={() => navigate(-1)}>
            <ArrowLeft className="h-4 w-4" />
          </Button>
          <div>
            <h1 className="text-3xl font-bold tracking-tight">Backup Manager</h1>
            <p className="text-muted-foreground">
              {selectedReposList.length} repositories selected for backup
            </p>
          </div>
        </div>
        <div className="flex items-center gap-2">
          {options.destination && (
            <Button variant="outline" onClick={handleOpenBackupFolder}>
              <FolderOpen className="mr-2 h-4 w-4" />
              Open Folder
            </Button>
          )}
          {isRunning ? (
            <Button variant="destructive" onClick={handleCancelBackup}>
              Cancel Backup
            </Button>
          ) : (
            <Button onClick={handleStartBackup} disabled={selectedReposList.length === 0}>
              <Play className="mr-2 h-4 w-4" />
              Start Backup
            </Button>
          )}
        </div>
      </div>

      {/* Content */}
      <Tabs value={activeTab} onValueChange={setActiveTab}>
        <TabsList>
          <TabsTrigger value="options">Options</TabsTrigger>
          <TabsTrigger value="progress">
            Progress
            {isRunning && (
              <span className="ml-2 h-2 w-2 animate-pulse rounded-full bg-primary" />
            )}
          </TabsTrigger>
          <TabsTrigger value="history">History</TabsTrigger>
        </TabsList>

        <TabsContent value="options" className="mt-6">
          <BackupOptions
            options={options}
            setOptions={setOptions}
            onSelectFolder={handleSelectFolder}
            selectedRepos={selectedReposList}
          />
        </TabsContent>

        <TabsContent value="progress" className="mt-6">
          <BackupProgress progress={progress} isRunning={isRunning} />
        </TabsContent>

        <TabsContent value="history" className="mt-6">
          <BackupHistory history={history} onOpenFolder={openFolder} />
        </TabsContent>
      </Tabs>
    </div>
  )
}
