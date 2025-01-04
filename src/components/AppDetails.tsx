import { FC } from "react"
import { Modal } from "./ui/Modal"
import { App } from "../types/entities"

interface AppDetailsProps {
  appDetailsRef: React.RefObject<HTMLDialogElement>
  closeAppDetails: () => void
  app: App
}

export const AppDetails: FC<AppDetailsProps> = ({
  appDetailsRef,
  closeAppDetails,
  app
}) => {
  return (
    <Modal
      ref={appDetailsRef}
      defaultOpen={true}
    >
      <h2>{app.name}</h2>
      <dl>
        <dt>Note</dt>
        <dd>{app.note}</dd>

        <dt>Link</dt>
        <dd>{app.url}</dd>

        <dt>Environments</dt>
        <dd>
          {app.environments.map((env) => (
            <span key={env.id} className="badge badge-primary">{env.name}</span>
          ))}
        </dd>

        <dt>Labels</dt>
        <dd>
          {app.labels.map((label) => (
            <span key={label} className="badge badge-accent">{label}</span>
          ))}
        </dd>

        <dt>Bounded Context</dt>
        <dd>
          {app.bounded_context && (
            <span className="badge badge-accent">{app.bounded_context}</span>
          )}
        </dd>
      </dl>

      <div className="flex gap-2 justify-end">
        <button className="btn btn-warning" onClick={closeAppDetails}>
          Close
        </button>
      </div>
    </Modal>
  )
}
