import { Routes, Route, Navigate } from 'react-router-dom'
import { useApp } from './context/AppContext'
import Layout from './components/Layout/Layout'
import Dashboard from './components/Dashboard/Dashboard'
import RepoList from './components/Repositories/RepoList'
import BackupManager from './components/Backup/BackupManager'
import Settings from './components/Settings/Settings'
import About from './components/About/About'
import AuthScreen from './components/Auth/AuthScreen'
import Spinner from './components/common/Spinner'

function App() {
  const { isAuthenticated, isLoading } = useApp()

  // Show loading spinner while checking auth
  if (isLoading) {
    return (
      <div className="flex h-screen items-center justify-center bg-background">
        <div className="text-center">
          <Spinner size="lg" />
          <p className="mt-4 text-muted-foreground">Loading...</p>
        </div>
      </div>
    )
  }

  // Show auth screen if not authenticated
  if (!isAuthenticated) {
    return <AuthScreen />
  }

  // Main app with navigation
  return (
    <Layout>
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/repositories" element={<RepoList />} />
        <Route path="/backup" element={<BackupManager />} />
        <Route path="/settings" element={<Settings />} />
        <Route path="/about" element={<About />} />
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </Layout>
  )
}

export default App
