import { forwardRef, useRef, useState } from 'react'

interface ModalProps {
  children: React.ReactNode
  defaultOpen?: boolean
  size?: 'lg'
}

export const Modal = forwardRef<HTMLDialogElement, ModalProps>(
  ({ children, defaultOpen = false, size }, ref) => {
    const [isOpen, setIsOpen] = useState(defaultOpen)

    return (
      <dialog ref={ref} className={`modal ${isOpen ? 'modal-open' : ''}`}>
        <div
          className={`modal-box flex flex-col ${size === 'lg' && 'max-w-[55rem]'}`}
        >
          {children}
        </div>
        <form
          method="dialog"
          onSubmit={() => setIsOpen(false)}
          className="modal-backdrop"
        >
          <button>close</button>
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
