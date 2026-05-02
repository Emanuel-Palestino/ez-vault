import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
  type ReactNode,
} from 'react'
import { invoke } from '@tauri-apps/api/core'
import { TAURI_CMD } from '../utils/constants'
import type { SetupStatus } from '../types/entities'

export type SessionState = 'loading' | 'needs_setup' | 'locked' | 'unlocked'

interface SessionContextValue {
  sessionState: SessionState
  currentUser: string | null
  refresh: () => Promise<void>
  lock: () => Promise<void>
}

const SessionContext = createContext<SessionContextValue | null>(null)

export function SessionProvider({ children }: { children: ReactNode }) {
  const [sessionState, setSessionState] = useState<SessionState>('loading')
  const [currentUser, setCurrentUser] = useState<string | null>(null)

  const refresh = useCallback(async () => {
    try {
      const status = await invoke<SetupStatus>(TAURI_CMD.CHECK_SETUP_STATUS)
      if (status.needsSetup) {
        setSessionState('needs_setup')
        setCurrentUser(null)
        return
      }
      if (status.isLocked) {
        setSessionState('locked')
        setCurrentUser(null)
        return
      }
      const user = await invoke<string | null>(TAURI_CMD.GET_CURRENT_USER)
      setCurrentUser(user)
      setSessionState('unlocked')
    } catch {
      setSessionState('locked')
      setCurrentUser(null)
    }
  }, [])

  const lock = useCallback(async () => {
    await invoke(TAURI_CMD.LOCK)
    setSessionState('locked')
    setCurrentUser(null)
  }, [])

  useEffect(() => {
    refresh()
  }, [refresh])

  return (
    <SessionContext.Provider value={{ sessionState, currentUser, refresh, lock }}>
      {children}
    </SessionContext.Provider>
  )
}

export function useSession(): SessionContextValue {
  const ctx = useContext(SessionContext)
  if (!ctx) throw new Error('useSession must be used within SessionProvider')
  return ctx
}
