import { Toolbar } from './components/Toolbar'
import { CreateAppModal } from './components/CreateAppModal'
import { useModal } from './components/ui/Modal'
import { CreateEnvModal } from './components/CreateEnvModal'
import { CreateCredentialModal } from './components/CreateCredentialModal'
import { CreateSecretModal } from './components/CreateSecretModal'
import { CreatePortModal } from './components/CreatePortModal'
import { MainContent } from './components/MainContent'
import { Settings } from './components/Settings'

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

  const {
    modalRef: createCredentialModalRef,
    open: openCreateCredentialModal,
    close: closeCreateCredentialModal,
  } = useModal()

  const {
    modalRef: createSecretModalRef,
    open: openCreateSecretModal,
    close: closeCreateSecretModal,
  } = useModal()

  const {
    modalRef: createPortModalRef,
    open: openCreatePortModal,
    close: closeCreatePortModal,
  } = useModal()

  const {
    modalRef: settingsModalRef,
    open: openSettingsModal,
    close: closeSettingsModal,
  } = useModal()

  return (
    <main className="w-full p-2">
      <Toolbar
        openSettings={openSettingsModal}
        openCreateApp={openCreateAppModal}
        openCreateEnv={openCreateEnvModal}
        openCreateCredential={openCreateCredentialModal}
        openCreateSecret={openCreateSecretModal}
        openCreatePort={openCreatePortModal}
      />

      <Settings
        closeModal={closeSettingsModal}
        modalRef={settingsModalRef}
      />

      <MainContent />

      <CreateAppModal
        closeModal={closeCreateAppModal}
        modalRef={createAppModalRef}
      />

      <CreateEnvModal
        closeModal={closeCreateEnvModal}
        modalRef={createEnvModalRef}
      />

      <CreateCredentialModal
        closeModal={closeCreateCredentialModal}
        modalRef={createCredentialModalRef}
      />

      <CreateSecretModal
        closeModal={closeCreateSecretModal}
        modalRef={createSecretModalRef}
      />

      <CreatePortModal
        closeModal={closeCreatePortModal}
        modalRef={createPortModalRef}
      />
    </main>
  )
}
