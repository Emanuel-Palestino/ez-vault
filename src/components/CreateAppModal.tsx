import { FC } from 'react'
import { Modal } from './ui/Modal'

interface CreateAppModalProps {
  isModalOpen: boolean
  closeModal: () => void
  modalRef: React.RefObject<HTMLDialogElement>
}

export const CreateAppModal: FC<CreateAppModalProps> = ({
  isModalOpen,
  closeModal,
  modalRef,
}) => {
  return (
    <Modal isOpen={isModalOpen} onClose={closeModal} ref={modalRef}>
      <h2 className="text-xl font-bold mb-6">Create application</h2>

      <form
        id="create-app-form"
        className="mb-6"
        onSubmit={(e) => {
          e.preventDefault()
          closeModal()
        }}
      >
        <input type="text" />
      </form>

      <div className="flex gap-2 justify-end">
        <button
          className="cursor-pointer bg-rose-600 text-white px-4 py-2 rounded-xl hover:bg-rose-700 transition-colors duration-200 active:bg-rose-800"
          onClick={closeModal}
        >
          Cancel
        </button>

        <button
          className="cursor-pointer bg-emerald-600 text-white px-4 py-2 rounded-xl hover:bg-emerald-700 transition-colors duration-200 active:bg-emerald-800"
          form="create-app-form"
        >
          Create
        </button>
      </div>
    </Modal>
  )
}
