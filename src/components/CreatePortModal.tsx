import { FC, useRef } from 'react'
import { Modal } from './ui/Modal'
import { createPort, useGetApps } from '../services/storage'

interface CreatePortModalProps {
  closeModal: () => void
  modalRef: React.RefObject<HTMLDialogElement>
}

export const CreatePortModal: FC<CreatePortModalProps> = ({
  closeModal,
  modalRef,
}) => {
  const { apps } = useGetApps()
  const formRef = useRef<HTMLFormElement>(null)

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()

    const formData = new FormData(formRef.current!)

    await createPort({
      app_id: formData.get('port_app_id') as string,
      value: parseInt(formData.get('port_value') as string),
      note: formData.get('port_note') as string,
    })

    formRef.current?.reset()
    closeModal()
  }

  return (
    <Modal ref={modalRef}>
      <h2>Create port</h2>

      <form
        ref={formRef}
        id="create-port-form"
        className="mb-4 overflow-y-auto"
        onSubmit={handleSubmit}
      >
        <fieldset className="fieldset">
          <legend className="fieldset-legend">Application *</legend>
          <select
            defaultValue="Select an application"
            className="select validator"
            name="port_app_id"
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
          <legend className="fieldset-legend">Port *</legend>
          <input
            type="number"
            className="input validator"
            name="port_value"
            placeholder="Port number"
            required
            autoComplete="off"
            min={0}
          />
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Note</legend>
          <textarea
            className="textarea"
            name="port_note"
            placeholder="Optional port's note"
            autoComplete="off"
          ></textarea>
        </fieldset>
      </form>

      <div className="modal-action">
        <button className="btn btn-error" onClick={closeModal}>
          Cancel
        </button>

        <button
          className="btn btn-success"
          form="create-port-form"
          type="submit"
        >
          Create
        </button>
      </div>
    </Modal>
  )
}
