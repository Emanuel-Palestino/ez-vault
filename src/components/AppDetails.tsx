import { FC, useState } from 'react'
import { Modal } from './ui/Modal'
import { App } from '../types/entities'
import {
  useGetCredentialsByAppId,
  useGetEnvironments,
  useGetSecretsByAppId,
} from '../services/storage'

const DEFAULT_APP_DETAILS: App = {
  id: '',
  name: '',
  url: '',
  note: '',
  environmentId: '',
  labels: [],
  createdAtTs: 0,
  updatedAtTs: 0,
  deleted: false,
}

function MaskedField({ value, label }: { value: string | null; label: string }) {
  const [revealed, setRevealed] = useState(false)

  if (!value) {
    return (
      <div>
        <dt className="text-sm text-gray-400">{label}</dt>
        <dd className="ml-2 mb-2 text-base-content/40">n/a</dd>
      </div>
    )
  }

  return (
    <div>
      <dt className="text-sm text-gray-400">{label}</dt>
      <dd className="ml-2 mb-2 flex items-center gap-2">
        <span className={revealed ? '' : 'tracking-widest select-none'}>
          {revealed ? value : '••••••••'}
        </span>
        <button
          type="button"
          className="btn btn-ghost btn-xs"
          onClick={() => setRevealed((v) => !v)}
        >
          {revealed ? 'hide' : 'show'}
        </button>
      </dd>
    </div>
  )
}

interface AppDetailsProps {
  appDetailsRef: React.RefObject<HTMLDialogElement | null>
  closeAppDetails: () => void
  app: App | null
}

export const AppDetails: FC<AppDetailsProps> = ({
  appDetailsRef,
  closeAppDetails,
  app,
}) => {

  const appData = app || DEFAULT_APP_DETAILS

  const { credentials } = useGetCredentialsByAppId(appData.id)
  const { secrets } = useGetSecretsByAppId(appData.id)
  const { environments } = useGetEnvironments()
  const environment = environments.find((env) => env.id === appData.environmentId)

  return (
    <Modal ref={appDetailsRef} size="lg">
      <h2>{appData.name}</h2>

      <section className="overflow-y-auto mt-5">
        <div className="flex gap-4 items-center">
          <h3>General Info</h3>
          <button className="btn btn-ghost btn-sm">edit</button>
        </div>
        <dl className="grid grid-cols-3 gap-x-4 gap-y-2 mt-2 mb-6">
          <div>
            <dt className="text-sm text-gray-400">Link</dt>
            <dd className="ml-2 mb-2">{appData.url}</dd>
          </div>

          <div>
            <dt className="text-sm text-gray-400">Environment</dt>
            <dd className="ml-2 mb-2">
              {environment ? (
                <span className="badge badge-primary">
                  {environment.name}
                </span>
              ) : null}
            </dd>
          </div>

          <div>
            <dt className="text-sm text-gray-400">Labels</dt>
            <dd className="ml-2 mb-2">
              {appData.labels.map((label) => (
                <span key={label} className="badge badge-accent">
                  {label}
                </span>
              ))}
            </dd>
          </div>

          <div>
            <dt className="text-sm text-gray-400">Note</dt>
            <dd className="ml-2 mb-2">{appData.note}</dd>
          </div>
        </dl>

        <div className="flex gap-4 items-center">
          <h3>Credentials</h3>
          <button className="btn btn-ghost btn-sm">edit</button>
        </div>
        <dl className="mt-2 mb-6">
          {credentials.map((credential) => (
            <div className="grid grid-cols-4 mt-2" key={credential.id}>
              <div>
                <dt className="text-sm text-gray-400">Context</dt>
                <dd className="ml-2 mb-2">{credential.context}</dd>
              </div>

              <div>
                <dt className="text-sm text-gray-400">Url</dt>
                <dd className="ml-2 mb-2">{credential.url}</dd>
              </div>

              <div>
                <dt className="text-sm text-gray-400">Username</dt>
                <dd className="ml-2 mb-2">{credential.username}</dd>
              </div>

              <MaskedField value={credential.password} label="Password" />

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

              <MaskedField value={secret.value} label="Value" />

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
