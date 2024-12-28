import { Toolbar } from './components/Toolbar'
import { CreateAppModal } from './components/CreateAppModal'
import { useModal } from './components/ui/Modal'

export function App() {
  const { modalRef, open, close } = useModal()

  return (
    <main className="w-full p-2">
      <Toolbar openCreateApp={open} />

      <CreateAppModal closeModal={close} modalRef={modalRef} />
    </main>
  )
}
