import { FC } from 'react'
import { Modal } from './ui/Modal'

interface CreateAppModalProps {
  closeModal: () => void
  modalRef: React.RefObject<HTMLDialogElement>
}

export const CreateAppModal: FC<CreateAppModalProps> = ({
  closeModal,
  modalRef,
}) => {
  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    closeModal()
  }

  return (
    <Modal ref={modalRef}>
      <h2>Create application</h2>

      <form id="create-app-form" className="mb-4" onSubmit={handleSubmit}>
        <fieldset className="fieldset">
          <legend className="fieldset-legend">App name</legend>
          <input
            type="text"
            className="input"
            name="name"
            placeholder="Application's name"
          />
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">URL</legend>
          <input
            type="url"
            className="input"
            name="url"
            placeholder="Application's url"
          />
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Bounded context</legend>
          <input
            type="text"
            className="input"
            name="bounded_context"
            placeholder="Application's bounded context"
          />
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Description</legend>
          <textarea
            className="textarea h-24"
            name="description"
            placeholder="Optional application's description"
          ></textarea>
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Environments</legend>
          <input
            type="text"
            className="input"
            name="environments"
            placeholder="Application's environments"
          />
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Labels</legend>
          <input
            type="text"
            className="input"
            name="labels"
            placeholder="Application's labels"
          />
        </fieldset>
      </form>

      <div className="flex gap-2 justify-end">
        <button className="btn btn-error" onClick={closeModal}>
          Cancel
        </button>

        <button
          className="btn btn-success"
          form="create-app-form"
          type="submit"
        >
          Create
        </button>
      </div>
    </Modal>
  )
}
