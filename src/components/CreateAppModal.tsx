import { FC, useRef, useState } from 'react'
import { Modal } from './ui/Modal'

interface CreateAppModalProps {
  closeModal: () => void
  modalRef: React.RefObject<HTMLDialogElement>
}

export const CreateAppModal: FC<CreateAppModalProps> = ({
  closeModal,
  modalRef,
}) => {
  const formRef = useRef<HTMLFormElement>(null)

  const [extraOptions, setExtraOptions] = useState<boolean>(false)

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()

    formRef.current?.reset()
    closeModal()
  }

  return (
    <Modal ref={modalRef}>
      <h2>Create an application</h2>

      <form
        ref={formRef}
        id="create-app-form"
        className="mb-4"
        onSubmit={handleSubmit}
      >
        <fieldset className="fieldset">
          <legend className="fieldset-legend">App name *</legend>
          <input
            type="text"
            className="input validator"
            name="app_name"
            placeholder="Application's name"
            autoComplete="off"
            required
          />
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">URL</legend>
          <input
            type="url"
            className="input"
            name="app_url"
            placeholder="Application's url"
            autoComplete="off"
          />
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Description</legend>
          <textarea
            className="textarea"
            name="app_description"
            placeholder="Optional application's description"
            autoComplete="off"
          ></textarea>
        </fieldset>

        <div className="w-full flex justify-end">
          <button
            type="button"
            className="btn btn-link px-2 btn-sm"
            onClick={() => setExtraOptions(!extraOptions)}
          >
            {extraOptions ? 'Hide extra options' : 'Show extra options'}
          </button>
        </div>

        {extraOptions && (
          <section>
            <fieldset className="fieldset">
              <legend className="fieldset-legend">Bounded context</legend>
              <input
                type="text"
                className="input"
                name="app_bounded_context"
                placeholder="Application's bounded context"
                autoComplete="off"
              />
            </fieldset>

            <fieldset className="fieldset">
              <legend className="fieldset-legend">Environments</legend>
              <input
                type="text"
                className="input"
                name="app_environments"
                placeholder="Application's environments"
                autoComplete="off"
              />
            </fieldset>

            <fieldset className="fieldset">
              <legend className="fieldset-legend">Labels</legend>
              <input
                type="text"
                className="input"
                name="labels"
                placeholder="Application's labels"
                autoComplete="off"
              />
            </fieldset>
          </section>
        )}
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
