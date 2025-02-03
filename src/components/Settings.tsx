import { FC } from "react"
import { Modal } from "./ui/Modal"

interface SettingsProps {
  closeModal: () => void
  modalRef: React.RefObject<HTMLDialogElement>
}

export const Settings: FC<SettingsProps> = ({
  closeModal,
  modalRef,
}) => {

  return (
    <Modal ref={modalRef}>
      <h2>Settings</h2>

      <div className="modal-action">
        <button className="btn btn-warning" onClick={closeModal}>
          Cerrar
        </button>
      </div>
    </Modal>
  )

}
