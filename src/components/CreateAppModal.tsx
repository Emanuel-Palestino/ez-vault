import { FC, useRef, useState } from 'react'
import { Modal } from './ui/Modal'
import { createApp, useGetEnvironments } from '../services/storage'

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
  const { environments } = useGetEnvironments()

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()

    const formData = new FormData(formRef.current!)

    await createApp({
      name: formData.get('app_name') as string,
      url: formData.get('app_url') as string,
      note: formData.get('app_note') as string,
      bounded_context: formData.get('app_bounded_context') as string,
      environment_ids: [
        (formData.get('app_environments') as string) || environments[0].id,
      ],
      labels: formData.get('labels') ? [formData.get('labels') as string] : [],
    })

    formRef.current?.reset()
    closeModal()
  }

  return (
    <Modal ref={modalRef}>
      <h2>Create an application</h2>

      <form
        ref={formRef}
        id="create-app-form"
        className="mt-5 overflow-y-auto"
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
          <legend className="fieldset-legend">Note</legend>
          <textarea
            className="textarea"
            name="app_note"
            placeholder="Optional application's note"
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
              <select
                defaultValue="Select an environment"
                className="select"
                name="app_environments"
              >
                <option disabled={true}>Select an environment</option>
                {environments.map((env) => (
                  <option key={env.id} value={env.id}>
                    {env.name}
                  </option>
                ))}
              </select>
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

      <div className="modal-action">
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
