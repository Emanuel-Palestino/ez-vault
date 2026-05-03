import { createFileRoute, redirect, useRouter } from '@tanstack/react-router'
import { useState } from 'react'
import { isConfigured, saveConfiguration } from '../services/storage'

export const Route = createFileRoute('/setup')({
  beforeLoad: async () => {
    const configured = await isConfigured()
    if (configured) throw redirect({ to: '/' })
  },
  component: SetupPage,
})

function SetupPage() {
  const router = useRouter()
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleSubmit = async (e: React.SubmitEvent<HTMLFormElement>) => {
    e.preventDefault()
    const data = new FormData(e.currentTarget)
    const databaseUrl = data.get('databaseUrl') as string
    const databaseToken = data.get('databaseToken') as string

    setLoading(true)
    setError(null)

    try {
      await saveConfiguration({ databaseUrl, databaseToken })
      await router.navigate({ to: '/' })
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err))
    } finally {
      setLoading(false)
    }
  }

  return (
    <main className="w-full h-dvh flex items-center justify-center bg-base-200">
      <div className="card bg-base-100 w-full max-w-md shadow-xl">
        <div className="card-body gap-4">
          <h1 className="card-title text-2xl">Setup ez-vault</h1>
          <p className="text-base-content/60 text-sm">
            Connect your Turso database to get started.
          </p>

          <form onSubmit={handleSubmit} className="flex flex-col gap-4">
            <fieldset className="fieldset">
              <label className="fieldset-label">Database URL</label>
              <input
                name="databaseUrl"
                type="text"
                className="input input-bordered w-full"
                placeholder="libsql://your-db.turso.io"
                required
              />
            </fieldset>

            <fieldset className="fieldset">
              <label className="fieldset-label">Database Token</label>
              <input
                name="databaseToken"
                type="password"
                className="input input-bordered w-full"
                placeholder="eyJ..."
                required
              />
            </fieldset>

            {error && (
              <div className="alert alert-error text-sm">
                <span>{error}</span>
              </div>
            )}

            <button
              type="submit"
              className="btn btn-primary"
              disabled={loading}
            >
              {loading && (
                <span className="loading loading-spinner loading-sm" />
              )}
              Connect
            </button>
          </form>
        </div>
      </div>
    </main>
  )
}
