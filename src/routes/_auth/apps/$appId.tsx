import { createFileRoute, Link } from '@tanstack/react-router'
import { useModal } from '../../../components/ui/Modal'
import { CreateCredentialModal } from '../../../components/CreateCredentialModal'
import { CreateSecretModal } from '../../../components/CreateSecretModal'
import {
  useGetApps,
  useGetCredentialsByAppId,
  useGetSecretsByAppId,
} from '../../../services/storage'

export const Route = createFileRoute('/_auth/apps/$appId')({
  component: AppPage,
})

function AppPage() {
  const { appId } = Route.useParams()
  const { apps } = useGetApps()
  const { credentials } = useGetCredentialsByAppId(appId)
  const { secrets } = useGetSecretsByAppId(appId)

  const app = apps?.find((a) => a.id === appId)

  const {
    modalRef: createCredRef,
    open: openCreateCred,
    close: closeCreateCred,
  } = useModal()
  const {
    modalRef: createSecretRef,
    open: openCreateSecret,
    close: closeCreateSecret,
  } = useModal()

  return (
    <div className="p-4 flex flex-col gap-6">
      <div className="flex items-center gap-3">
        {app?.environmentId && (
          <Link
            to="/environments/$environmentId"
            params={{ environmentId: app.environmentId }}
            className="btn btn-ghost btn-sm"
          >
            ← Environment
          </Link>
        )}
        <h1 className="text-2xl font-bold">{app?.name ?? appId}</h1>
        {app?.url && (
          <span className="text-base-content/60 text-sm">{app.url}</span>
        )}
      </div>

      {/* Credentials */}
      <section className="flex flex-col gap-3">
        <div className="flex items-center justify-between">
          <h2 className="text-xl font-bold">Credentials</h2>
          <button className="btn btn-primary btn-sm" onClick={openCreateCred}>
            New Credential
          </button>
        </div>

        <div className="flex flex-col gap-2">
          {credentials?.map((cred) => (
            <div key={cred.id} className="card bg-base-100 shadow">
              <div className="card-body py-3">
                <div className="flex items-center justify-between">
                  <span className="font-medium">{cred.context}</span>
                  <span className="text-base-content/60 text-sm">
                    {cred.username}
                  </span>
                </div>
                {cred.note && (
                  <p className="text-base-content/60 text-sm">{cred.note}</p>
                )}
              </div>
            </div>
          ))}
        </div>
      </section>

      {/* Secrets */}
      <section className="flex flex-col gap-3">
        <div className="flex items-center justify-between">
          <h2 className="text-xl font-bold">Secrets</h2>
          <button className="btn btn-primary btn-sm" onClick={openCreateSecret}>
            New Secret
          </button>
        </div>

        <div className="flex flex-col gap-2">
          {secrets?.map((secret) => (
            <div key={secret.id} className="card bg-base-100 shadow">
              <div className="card-body py-3">
                <div className="flex items-center justify-between">
                  <span className="font-mono text-sm font-medium">
                    {secret.key}
                  </span>
                  <span className="text-base-content/60 text-sm">••••••••</span>
                </div>
                {secret.note && (
                  <p className="text-base-content/60 text-sm">{secret.note}</p>
                )}
              </div>
            </div>
          ))}
        </div>
      </section>

      <CreateCredentialModal
        modalRef={createCredRef}
        closeModal={closeCreateCred}
      />
      <CreateSecretModal
        modalRef={createSecretRef}
        closeModal={closeCreateSecret}
      />
    </div>
  )
}
