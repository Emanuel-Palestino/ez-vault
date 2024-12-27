import React, { forwardRef } from 'react'

interface ModalProps {
  isOpen: boolean
  onClose: () => void
  children: React.ReactNode
}

export const Modal = forwardRef<HTMLDialogElement, ModalProps>(
  ({ isOpen, onClose, children }, ref) => {
    return (
      <>
        {isOpen && (
          <div className="fixed inset-0 z-40 bg-black/50 flex items-center justify-center transition-opacity">
            <dialog
              ref={ref}
              open={isOpen}
              className="static bg-white rounded-lg p-6 shadow-lg w-full max-w-md transform transition-all duration-300 scale-100"
            >
              <button
                className="absolute top-3 right-3 text-gray-500 hover:text-black cursor-pointer"
                onClick={onClose}
              >
                ✖
              </button>
              {children}
            </dialog>
          </div>
        )}
      </>
    )
  },
)
