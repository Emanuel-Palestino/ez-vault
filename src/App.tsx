import { Toolbar } from './components/Toolbar'
import { CreateAppModal } from './components/CreateAppModal'
import { useModal } from './components/ui/Modal'
import { CreateEnvModal } from './components/CreateEnvModal'
import { CreateCredentialModal } from './components/CreateCredentialModal'
import { CreateSecretModal } from './components/CreateSecretModal'
import { CreateCertificateModal } from './components/CreateCertificateModal'
import { MainContent } from './components/MainContent'

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
    modalRef: createCertificateModalRef,
    open: openCreateCertificateModal,
    close: closeCreateCertificateModal,
  } = useModal()

  return (
    <main className="w-full h-dvh p-2 bg-base-200">
      <Toolbar
        openCreateApp={openCreateAppModal}
        openCreateEnv={openCreateEnvModal}
        openCreateCredential={openCreateCredentialModal}
        openCreateSecret={openCreateSecretModal}
        openCreateCertificate={openCreateCertificateModal}
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

      <CreateCertificateModal
        closeModal={closeCreateCertificateModal}
        modalRef={createCertificateModalRef}
      />
    </main>
  )
}
