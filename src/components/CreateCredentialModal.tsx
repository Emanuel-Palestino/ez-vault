import { FC, useRef } from 'react'
import { Modal } from './ui/Modal'
import { createCredential, useGetApps } from '../services/storage'

interface CreateCredentialModalProps {
  closeModal: () => void
  modalRef: React.RefObject<HTMLDialogElement>
}

export const CreateCredentialModal: FC<CreateCredentialModalProps> = ({
  closeModal,
  modalRef,
}) => {
  const { apps } = useGetApps()
  const formRef = useRef<HTMLFormElement>(null)

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()

    const formData = new FormData(formRef.current!)

    await createCredential({
      app_id: formData.get('credential_app_id') as string,
      username: formData.get('credential_username') as string,
      password: formData.get('credential_password') as string,
      note: formData.get('credential_note') as string,
    })

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
          <select
            defaultValue="Select an application"
            className="select validator"
            name="credential_app_id"
            required
          >
            <option disabled={true}>Select an application</option>
            {apps.map((app) => (
              <option key={app.id} value={app.id}>
                {app.name}
              </option>
            ))}
          </select>
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
          <legend className="fieldset-legend">Note</legend>
          <textarea
            className="textarea"
            name="credential_note"
            placeholder="Optional credential's note"
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
