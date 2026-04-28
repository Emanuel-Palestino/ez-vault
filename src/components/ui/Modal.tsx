import { forwardRef, useRef } from 'react'

interface ModalProps {
  children: React.ReactNode
  size?: 'lg'
}

export const Modal = forwardRef<HTMLDialogElement, ModalProps>(
  ({ children, size }, ref) => {

    return (
      <dialog ref={ref} className="modal">
        <div
          className={`modal-box flex flex-col ${size === 'lg' && 'max-w-220'}`}
        >
          {children}
        </div>
        <form
          method="dialog"
          className="modal-backdrop"
        >
          <button type="submit">close</button>
        </form>
      </dialog>
    )
  },
)

export const useModal = () => {
  const modalRef = useRef<HTMLDialogElement>(null)

  const open = () => {
    if (modalRef.current) modalRef.current.showModal()
  }

  const close = () => {
    if (modalRef.current) {
      modalRef.current.classList.remove('modal-open')
      modalRef.current.close()
    }
  }

  return { modalRef, open, close }
}
