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
        <dl>
          <dt>Note</dt>
          <dd>{app.note}</dd>

          <dt>Link</dt>
          <dd>{app.url}</dd>

          <dt>Environments</dt>
          <dd>
            {app.environments.map((env) => (
              <span key={env.id} className="badge badge-primary">
                {env.name}
              </span>
            ))}
          </dd>

          <dt>Labels</dt>
          <dd>
            {app.labels.map((label) => (
              <span key={label} className="badge badge-accent">
                {label}
              </span>
            ))}
          </dd>

          <dt>Bounded Context</dt>
          <dd>
            {app.bounded_context && (
              <span className="badge badge-accent">{app.bounded_context}</span>
            )}
          </dd>
        </dl>

        <h3>Ports</h3>
        <p>
          {ports.map((port) => (
            <span key={port.id} className="badge badge-primary">
              {port.value}
            </span>
          ))}
        </p>

        <h3>Credentials</h3>
        <dl>
          {credentials.map((credential) => (
            <div key={credential.id}>
              <dt>Username</dt>
              <dd>{credential.username}</dd>

              <dt>Password</dt>
              <dd>{credential.password}</dd>
            </div>
          ))}
        </dl>

        <h3>Secrets</h3>
        <dl>
          {secrets.map((secret) => (
            <div key={secret.id}>
              <dt>Key</dt>
              <dd>{secret.key}</dd>

              <dt>Value</dt>
              <dd>{secret.value}</dd>
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
