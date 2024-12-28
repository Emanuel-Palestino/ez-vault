import { Toolbar } from './components/Toolbar'
import { CreateAppModal } from './components/CreateAppModal'
import { useModal } from './components/ui/Modal'

export function App() {
  const { modalRef, open, close } = useModal()

  return (
    <main className="p-2 prose prose-headings:mt-0">
      <Toolbar openCreateApp={open} />

      <CreateAppModal closeModal={close} modalRef={modalRef} />
    </main>
  )
}
