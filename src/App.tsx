import { useRef, useState } from 'react'
import { Toolbar } from './components/Toolbar'
import { CreateAppModal } from './components/CreateAppModal'

export function App() {
  const [isModalOpen, setModalOpen] = useState<boolean>(false)
  const modalRef = useRef<HTMLDialogElement>(null)

  const openModal = () => setModalOpen(true)
  const closeModal = () => {
    setModalOpen(false)
    if (modalRef.current) modalRef.current.close()
  }

  return (
    <main className="w-screen h-screen bg-neutral-200 p-2">
      <Toolbar openCreateApp={openModal} />
      <CreateAppModal
        isModalOpen={isModalOpen}
        closeModal={closeModal}
        modalRef={modalRef}
      />
    </main>
  )
}
