import { FC, useRef } from 'react'
import { Modal } from './ui/Modal'

interface CreateEnvModalProps {
  closeModal: () => void
  modalRef: React.RefObject<HTMLDialogElement>
}

export const CreateEnvModal: FC<CreateEnvModalProps> = ({
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
      <h2>Create environment</h2>

      <form
        ref={formRef}
        id="create-env-form"
        className="mb-4"
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
      </form>

      <div className="flex gap-2 justify-end">
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
