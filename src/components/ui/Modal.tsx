import { forwardRef, useRef } from 'react'

interface ModalProps {
  children: React.ReactNode
}

export const Modal = forwardRef<HTMLDialogElement, ModalProps>(
  ({ children }, ref) => {
    return (
      <dialog ref={ref} className="modal">
        <div className="modal-box">
          <article className="prose prose-headings:mt-0">{children}</article>
        </div>
        <form method="dialog" className="modal-backdrop">
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
    if (modalRef.current) modalRef.current.close()
  }

  return { modalRef, open, close }
}
