import { FC, useRef } from 'react'
import { Modal } from './ui/Modal'
import { createEnvironment } from '../services/storage'

interface CreateEnvModalProps {
  closeModal: () => void
  modalRef: React.RefObject<HTMLDialogElement>
}

export const CreateEnvModal: FC<CreateEnvModalProps> = ({
  closeModal,
  modalRef,
}) => {
  const formRef = useRef<HTMLFormElement>(null)

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()

    const formData = new FormData(formRef.current!)

    await createEnvironment({
      name: formData.get('env_name') as string,
      note: formData.get('env_note') as string,
    })

    formRef.current?.reset()
    closeModal()
  }

  return (
    <Modal ref={modalRef}>
      <h2>Create environment</h2>

      <form
        ref={formRef}
        id="create-env-form"
        className="mt-5 overflow-y-auto"
        onSubmit={handleSubmit}
      >
        <fieldset className="fieldset">
          <legend className="fieldset-legend">Environment name *</legend>
          <input
            type="text"
            className="input validator"
            name="env_name"
            placeholder="Environment's name"
            required
            autoComplete="off"
          />
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Note</legend>
          <textarea
            className="textarea"
            name="env_note"
            placeholder="Optional environment's note"
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
          form="create-env-form"
          type="submit"
        >
          Create
        </button>
      </div>
    </Modal>
  )
}
