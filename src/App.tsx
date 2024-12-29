import { Toolbar } from './components/Toolbar'
import { CreateAppModal } from './components/CreateAppModal'
import { useModal } from './components/ui/Modal'
import { CreateEnvModal } from './components/CreateEnvModal'

export function App() {
  const {
    modalRef: createAppModalRef,
    open: openCreateAppModal,
    close: closeCreateAppModal,
  } = useModal()

  const {
    modalRef: createEnvModalRef,
    open: openCreateEnvModal,
    close: closeCreateEnvModal,
  } = useModal()

  return (
    <main className="w-full p-2">
      <Toolbar
        openCreateApp={openCreateAppModal}
        openCreateEnv={openCreateEnvModal}
      />

      <CreateAppModal
        closeModal={closeCreateAppModal}
        modalRef={createAppModalRef}
      />
      <CreateEnvModal
        closeModal={closeCreateEnvModal}
        modalRef={createEnvModalRef}
      />
    </main>
  )
}
