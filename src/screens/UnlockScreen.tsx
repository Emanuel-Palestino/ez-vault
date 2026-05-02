import { useState } from 'react'
import { useSession } from '../context/SessionContext'
import { unlock } from '../services/session'
import type { VaultError } from '../types/entities'

export function UnlockScreen() {
  const { refresh } = useSession()
  const [masterPassword, setMasterPassword] = useState('')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)
    setLoading(true)
    try {
      await unlock(masterPassword)
      await refresh()
    } catch (err) {
      const ve = err as VaultError
      setError(ve?.message ?? 'Wrong master password')
      setMasterPassword('')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="w-full h-dvh flex items-center justify-center bg-base-200">
      <div className="card w-full max-w-sm bg-base-100 shadow-xl">
        <div className="card-body">
          <h1 className="card-title text-2xl mb-2">ez-vault</h1>
          <p className="text-sm text-base-content/60 mb-4">Enter your master password to unlock</p>

          <form onSubmit={handleSubmit} className="flex flex-col gap-3">
            <label className="form-control w-full">
              <span className="label-text mb-1">Master Password</span>
              <input
                type="password"
                className="input input-bordered w-full"
                value={masterPassword}
                onChange={(e) => setMasterPassword(e.target.value)}
                required
                autoFocus
                autoComplete="current-password"
              />
            </label>

            {error && (
              <div className="alert alert-error text-sm py-2">
                <span>{error}</span>
              </div>
            )}

            <button
              type="submit"
              className="btn btn-primary mt-2"
              disabled={loading}
            >
              {loading && <span className="loading loading-spinner loading-sm" />}
              Unlock
            </button>
          </form>
        </div>
      </div>
    </div>
  )
}
