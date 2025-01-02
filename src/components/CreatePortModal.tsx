import { FC, useRef } from 'react'
import { Modal } from './ui/Modal'

interface CreatePortModalProps {
  closeModal: () => void
  modalRef: React.RefObject<HTMLDialogElement>
}

export const CreatePortModal: FC<CreatePortModalProps> = ({
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
      <h2>Create port</h2>

      <form
        ref={formRef}
        id="create-port-form"
        className="mb-4"
        onSubmit={handleSubmit}
      >
        <fieldset className="fieldset">
          <legend className="fieldset-legend">Application *</legend>
          <input
            type="text"
            className="input validator"
            name="port_app_owner"
            placeholder="Application owner of the port"
            required
            autoComplete="off"
          />
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
          <legend className="fieldset-legend">Description</legend>
          <textarea
            className="textarea"
            name="port_description"
            placeholder="Optional port's description"
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
          form="create-port-form"
          type="submit"
        >
          Create
        </button>
      </div>
    </Modal>
  )
}
