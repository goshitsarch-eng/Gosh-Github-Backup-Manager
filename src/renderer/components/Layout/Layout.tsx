import { ReactNode } from 'react'
import { useApp } from '@/context/AppContext'
import Spinner from '@/components/common/Spinner'
import Sidebar from './Sidebar'
import Header from './Header'

interface LayoutProps {
  children: ReactNode
}

export default function Layout({ children }: LayoutProps) {
  const { authStatus } = useApp()

  return (
    <div className="flex h-screen bg-background">
      {/* Sidebar */}
      <Sidebar />

      {/* Main content area */}
      <div className="flex flex-1 flex-col overflow-hidden">
        {/* Header */}
        <Header />
        {authStatus && (
          <div className="flex items-center gap-2 border-b bg-muted/30 px-6 py-2 text-sm text-muted-foreground">
            <Spinner size="sm" />
            <span>{authStatus}</span>
          </div>
        )}

        {/* Page content */}
        <main className="flex-1 overflow-auto p-6">
          {children}
        </main>
      </div>
    </div>
  )
}
