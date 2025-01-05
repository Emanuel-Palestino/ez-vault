import { FC } from 'react'
import { Modal } from './ui/Modal'
import { App } from '../types/entities'
import {
  useGetCredentialsByAppId,
  useGetPortsByAppId,
  useGetSecretsByAppId,
} from '../services/storage'

interface AppDetailsProps {
  appDetailsRef: React.RefObject<HTMLDialogElement>
  closeAppDetails: () => void
  app: App
}

export const AppDetails: FC<AppDetailsProps> = ({
  appDetailsRef,
  closeAppDetails,
  app,
}) => {
  const { ports } = useGetPortsByAppId(app.id)
  const { credentials } = useGetCredentialsByAppId(app.id)
  const { secrets } = useGetSecretsByAppId(app.id)

  return (
    <Modal ref={appDetailsRef} defaultOpen={true} size="lg">
      <h2>{app.name}</h2>

      <section className="overflow-y-auto mt-5">
        <div className="flex gap-4 items-center">
          <h3>General Info</h3>
          <button className="btn btn-ghost btn-sm">edit</button>
        </div>
        <dl className="grid grid-cols-3 gap-x-4 gap-y-2 mt-2 mb-6">
          <div>
            <dt className="text-sm text-gray-400">Note</dt>
            <dd className="ml-2 mb-2">{app.note}</dd>
          </div>

          <div>
            <dt className="text-sm text-gray-400">Link</dt>
            <dd className="ml-2 mb-2">{app.url}</dd>
          </div>

          <div>
            <dt className="text-sm text-gray-400">Environments</dt>
            <dd className="ml-2 mb-2">
              {app.environments.map((env) => (
                <span key={env.id} className="badge badge-primary">
                  {env.name}
                </span>
              ))}
            </dd>
          </div>

          <div>
            <dt className="text-sm text-gray-400">Labels</dt>
            <dd className="ml-2 mb-2">
              {app.labels.map((label) => (
                <span key={label} className="badge badge-accent">
                  {label}
                </span>
              ))}
            </dd>
          </div>

          <div>
            <dt className="text-sm text-gray-400">Bounded Context</dt>
            <dd className="ml-2 mb-2">
              {app.bounded_context && (
                <span className="badge badge-accent">
                  {app.bounded_context}
                </span>
              )}
            </dd>
          </div>
        </dl>

        <div className="flex gap-4 items-center">
          <h3>Ports</h3>
          <button className="btn btn-ghost btn-sm">edit</button>
        </div>
        <div className="mt-2 mb-6">
          {ports.map((port) => (
            <span key={port.id} className="badge badge-primary">
              {port.value}
            </span>
          ))}
        </div>

        <div className="flex gap-4 items-center">
          <h3>Credentials</h3>
          <button className="btn btn-ghost btn-sm">edit</button>
        </div>
        <dl className="mt-2 mb-6">
          {credentials.map((credential) => (
            <div className="grid grid-cols-4 mt-2" key={credential.id}>
              <div>
                <dt className="text-sm text-gray-400">Username</dt>
                <dd className="ml-2 mb-2">{credential.username}</dd>
              </div>

              <div>
                <dt className="text-sm text-gray-400">Password</dt>
                <dd className="ml-2 mb-2">{credential.password}</dd>
              </div>

              <div className="col-span-2">
                <dt className="text-sm text-gray-400">Note</dt>
                <dd className="ml-2 mb-2">{credential.note}</dd>
              </div>
            </div>
          ))}
        </dl>

        <div className="flex gap-4 items-center">
          <h3>Secrets</h3>
          <button className="btn btn-ghost btn-sm">edit</button>
        </div>
        <dl className="mt-2">
          {secrets.map((secret) => (
            <div key={secret.id}>
              <dt className="text-sm text-gray-400">Key</dt>
              <dd className="ml-2 mb-2">{secret.key}</dd>

              <dt className="text-sm text-gray-400">Value</dt>
              <dd className="ml-2 mb-2">{secret.value}</dd>

              <dt className="text-sm text-gray-400">Note</dt>
              <dd className="ml-2 mb-2">{secret.note}</dd>
            </div>
          ))}
        </dl>
      </section>

      <div className="modal-action">
        <button className="btn btn-warning" onClick={closeAppDetails}>
          Close
        </button>
      </div>
    </Modal>
  )
}
