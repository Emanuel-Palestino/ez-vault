import { FC, useEffect, useRef, useState } from 'react'
import { Modal } from './ui/Modal'
import { createSecret, getApps } from '../services/storage'
import { App } from '../types/entities'

interface CreateSecretModalProps {
  closeModal: () => void
  modalRef: React.RefObject<HTMLDialogElement>
}

export const CreateSecretModal: FC<CreateSecretModalProps> = ({
  closeModal,
  modalRef,
}) => {
  const [apps, setApps] = useState<App[]>([])
  const formRef = useRef<HTMLFormElement>(null)

  useEffect(() => {
    getApps().then(setApps)
  }, [])

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()

    const formData = new FormData(formRef.current!)

    await createSecret({
      app_id: formData.get('secret_app_id') as string,
      key: formData.get('secret_key') as string,
      value: formData.get('secret_value') as string,
      note: formData.get('secret_note') as string,
    })

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
          <select
            defaultValue="Select an application"
            className="select validator"
            name="secret_app_id"
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
          <legend className="fieldset-legend">Note</legend>
          <textarea
            className="textarea"
            name="secret_note"
            placeholder="Optional secret's note"
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
