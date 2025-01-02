import { FC, useRef } from 'react'
import { Modal } from './ui/Modal'

interface CreateSecretModalProps {
  closeModal: () => void
  modalRef: React.RefObject<HTMLDialogElement>
}

export const CreateSecretModal: FC<CreateSecretModalProps> = ({
  closeModal,
  modalRef,
}) => {
  const formRef = useRef<HTMLFormElement>(null)

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()

    formRef.current?.reset()
    closeModal()
  }

  return (
    <Modal ref={modalRef}>
      <h2>Create secret</h2>

      <form
        ref={formRef}
        id="create-secret-form"
        className="mb-4"
        onSubmit={handleSubmit}
      >
        <fieldset className="fieldset">
          <legend className="fieldset-legend">Application *</legend>
          <input
            type="text"
            className="input validator"
            name="secret_app_owner"
            placeholder="Application owner of the secret"
            required
            autoComplete="off"
          />
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Key *</legend>
          <input
            type="text"
            className="input validator"
            name="secret_key"
            placeholder="Key or name of the secret"
            required
            autoComplete="off"
          />
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Value *</legend>
          <textarea
            className="textarea h-40 validator"
            name="secret_value"
            placeholder="Value or content of the secret"
            required
            autoComplete="off"
          ></textarea>
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Description</legend>
          <textarea
            className="textarea"
            name="secret_description"
            placeholder="Optional secret's description"
            autoComplete="off"
          ></textarea>
        </fieldset>
      </form>

      <div className="flex gap-2 justify-end">
        <button className="btn btn-error" onClick={closeModal}>
          Cancel
        </button>

        <button
          className="btn btn-success"
          form="create-secret-form"
          type="submit"
        >
          Create
        </button>
      </div>
    </Modal>
  )
}
