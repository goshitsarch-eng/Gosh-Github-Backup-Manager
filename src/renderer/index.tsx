import React from 'react'
import ReactDOM from 'react-dom/client'
import { HashRouter } from 'react-router-dom'
import App from './App'
import { AppProvider } from './context/AppContext'
import { Toaster } from './components/ui/toaster'
import './styles/globals.css'

// Initialize Tauri API bridge (sets up window.electronAPI for backwards compatibility)
import './lib/tauri-api'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <HashRouter>
      <AppProvider>
        <App />
        <Toaster />
      </AppProvider>
    </HashRouter>
  </React.StrictMode>
)
