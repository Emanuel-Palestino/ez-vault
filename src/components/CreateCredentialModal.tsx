import { FC, useRef } from 'react'
import { Modal } from './ui/Modal'

interface CreateCredentialModalProps {
  closeModal: () => void
  modalRef: React.RefObject<HTMLDialogElement>
}

export const CreateCredentialModal: FC<CreateCredentialModalProps> = ({
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
      <h2>Create credential</h2>

      <form
        ref={formRef}
        id="create-credential-form"
        className="mb-4"
        onSubmit={handleSubmit}
      >
        <fieldset className="fieldset">
          <legend className="fieldset-legend">Application *</legend>
          <input
            type="text"
            className="input validator"
            name="credential_app_owner"
            placeholder="Application owner of the credential"
            required
            autoComplete="off"
          />
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Username *</legend>
          <input
            type="text"
            className="input validator"
            name="credential_username"
            placeholder="Username or email"
            required
            autoComplete="off"
          />
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Password *</legend>
          <input
            type="text"
            className="input validator"
            name="credential_password"
            placeholder="Password"
            required
            autoComplete="off"
          />
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Description</legend>
          <textarea
            className="textarea"
            name="credential_description"
            placeholder="Optional credential's description"
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
          form="create-credential-form"
          type="submit"
        >
          Create
        </button>
      </div>
    </Modal>
  )
}
