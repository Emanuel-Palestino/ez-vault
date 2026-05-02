import { useState } from 'react'
import { useSession } from '../context/SessionContext'
import { createVault, joinVault } from '../services/session'
import type { VaultError } from '../types/entities'

type Tab = 'create' | 'join'

export function SetupScreen() {
  const { refresh } = useSession()
  const [tab, setTab] = useState<Tab>('create')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const [email, setEmail] = useState('')
  const [tursoUrl, setTursoUrl] = useState('')
  const [tursoToken, setTursoToken] = useState('')
  const [masterPassword, setMasterPassword] = useState('')
  const [inviteCode, setInviteCode] = useState('')

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)
    setLoading(true)
    try {
      if (tab === 'create') {
        await createVault({ email, tursoUrl, tursoToken, masterPassword })
      } else {
        await joinVault({ email, tursoUrl, tursoToken, masterPassword, inviteCode })
      }
      await refresh()
    } catch (err) {
      const ve = err as VaultError
      setError(ve?.message ?? String(err))
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="w-full h-dvh flex items-center justify-center bg-base-200">
      <div className="card w-full max-w-md bg-base-100 shadow-xl">
        <div className="card-body">
          <h1 className="card-title text-2xl mb-2">ez-vault</h1>

          <div role="tablist" className="tabs tabs-bordered mb-4">
            <button
              role="tab"
              className={`tab${tab === 'create' ? ' tab-active' : ''}`}
              onClick={() => setTab('create')}
            >
              Create Vault
            </button>
            <button
              role="tab"
              className={`tab${tab === 'join' ? ' tab-active' : ''}`}
              onClick={() => setTab('join')}
            >
              Join Vault
            </button>
          </div>

          <form onSubmit={handleSubmit} className="flex flex-col gap-3">
            <label className="form-control w-full">
              <span className="label-text mb-1">Email</span>
              <input
                type="email"
                className="input input-bordered w-full"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                required
                autoComplete="email"
              />
            </label>

            <label className="form-control w-full">
              <span className="label-text mb-1">Turso URL</span>
              <input
                type="text"
                className="input input-bordered w-full"
                value={tursoUrl}
                onChange={(e) => setTursoUrl(e.target.value)}
                required
                placeholder="libsql://..."
              />
            </label>

            <label className="form-control w-full">
              <span className="label-text mb-1">Turso Token</span>
              <input
                type="password"
                className="input input-bordered w-full"
                value={tursoToken}
                onChange={(e) => setTursoToken(e.target.value)}
                required
                autoComplete="off"
              />
            </label>

            <label className="form-control w-full">
              <span className="label-text mb-1">Master Password</span>
              <input
                type="password"
                className="input input-bordered w-full"
                value={masterPassword}
                onChange={(e) => setMasterPassword(e.target.value)}
                required
                autoComplete="new-password"
              />
            </label>

            {tab === 'join' && (
              <label className="form-control w-full">
                <span className="label-text mb-1">Invite Code</span>
                <input
                  type="text"
                  className="input input-bordered w-full tracking-widest"
                  value={inviteCode}
                  onChange={(e) => setInviteCode(e.target.value)}
                  required
                  placeholder="XXXX-XXXX"
                  autoComplete="off"
                />
              </label>
            )}

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
              {tab === 'create' ? 'Create Vault' : 'Join Vault'}
            </button>
          </form>
        </div>
      </div>
    </div>
  )
}
