import { FC, useRef } from 'react'
import { Modal } from './ui/Modal'
import { createCertificate, useGetEnvironments } from '../services/storage'

interface CreateCertificateModalProps {
  closeModal: () => void
  modalRef: React.RefObject<HTMLDialogElement | null>
  environmentId?: string
}

export const CreateCertificateModal: FC<CreateCertificateModalProps> = ({
  closeModal,
  modalRef,
  environmentId,
}) => {
  const { environments } = useGetEnvironments()
  const formRef = useRef<HTMLFormElement>(null)

  const handleSubmit = async (e: React.SubmitEvent<HTMLFormElement>) => {
    e.preventDefault()

    const formData = new FormData(formRef.current!)

    await createCertificate({
      name: formData.get('certificate_name') as string,
      fileName: formData.get('certificate_file_name') as string,
      fileExtension: formData.get('certificate_file_extension') as string,
      value: formData.get('certificate_value') as string,
      environmentId: environmentId ?? (formData.get('certificate_environment_id') as string),
      labels: formData.get('certificate_labels')
        ? [formData.get('certificate_labels') as string]
        : [],
      note: formData.get('certificate_note') as string,
    })

    formRef.current?.reset()
    closeModal()
  }

  return (
    <Modal ref={modalRef} size="lg">
      <h2>Create certificate</h2>

      <form
        ref={formRef}
        id="create-certificate-form"
        className="mt-5"
        onSubmit={handleSubmit}
      >
        <fieldset className="fieldset">
          <legend className="fieldset-legend">Certificate name *</legend>
          <input
            type="text"
            className="input validator"
            name="certificate_name"
            placeholder="Certificate's name"
            required
            autoComplete="off"
          />
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">File name *</legend>
          <input
            type="text"
            className="input validator"
            name="certificate_file_name"
            placeholder="Certificate's file name"
            required
            autoComplete="off"
          />
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">File extension *</legend>
          <input
            type="text"
            className="input validator"
            name="certificate_file_extension"
            placeholder="E.g. crt, pem, p12"
            required
            autoComplete="off"
          />
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Value *</legend>
          <textarea
            className="textarea h-40 validator"
            name="certificate_value"
            placeholder="Certificate content"
            required
            autoComplete="off"
          ></textarea>
        </fieldset>

        {!environmentId && (
          <fieldset className="fieldset">
            <legend className="fieldset-legend">Environment *</legend>
            <select
              defaultValue="Select an environment"
              className="select validator"
              name="certificate_environment_id"
              required
            >
              <option disabled={true}>Select an environment</option>
              {environments.map((environment) => (
                <option key={environment.id} value={environment.id}>
                  {environment.name}
                </option>
              ))}
            </select>
          </fieldset>
        )}

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Labels</legend>
          <input
            type="text"
            className="input"
            name="certificate_labels"
            placeholder="Certificate's labels"
            autoComplete="off"
          />
        </fieldset>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Note</legend>
          <textarea
            className="textarea"
            name="certificate_note"
            placeholder="Optional certificate's note"
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
          form="create-certificate-form"
          type="submit"
        >
          Create
        </button>
      </div>
    </Modal>
  )
}
