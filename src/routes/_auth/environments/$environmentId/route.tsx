import { createFileRoute, Link, Outlet } from '@tanstack/react-router'
import { useGetEnvironments } from '../../../../services/storage'
import AppsIcon from '../../../../icons/Apps'
import CertificatesIcon from '../../../../icons/Certificates'
import { useModal } from '../../../../components/ui/Modal'
import { CreateAppModal } from '../../../../components/CreateAppModal'
import { CreateCertificateModal } from '../../../../components/CreateCertificateModal'

export const Route = createFileRoute('/_auth/environments/$environmentId')({
  component: EnvironmentLayout,
})

function EnvironmentLayout() {
  const { environmentId } = Route.useParams()
  const { environments } = useGetEnvironments()
  const environment = environments?.find((e) => e.id === environmentId)

  const {
    modalRef: createAppRef,
    open: openCreateApp,
    close: closeCreateApp,
  } = useModal()
  const {
    modalRef: createCertRef,
    open: openCreateCert,
    close: closeCreateCert,
  } = useModal()

  return (
    <>
      <header className="w-full bg-base-100 px-4 pt-2 shadow-sm">
        <Link
          to="/environments"
          className="link link-hover text-gray-500 text-sm"
        >
          ← Environments
        </Link>

        <div className="mt-3">
          <h2 className="text-2xl flex items-center gap-3 font-semibold pl-2.5">
            {environment?.name}
            <span className="text-gray-400 text-sm">{environment?.note}</span>
          </h2>
        </div>

        <div role="tablist" className="tabs tabs-border mt-3">
          <Link
            to="/environments/$environmentId/apps"
            params={{ environmentId }}
            className="tab"
            activeProps={{ className: 'tab-active', role: 'tab' }}
            inactiveProps={{ role: 'tab' }}
          >
            <AppsIcon className="w-5 h-5 mr-2 fill-current" />
            Apps
          </Link>
          <Link
            to="/environments/$environmentId/certificates"
            params={{ environmentId }}
            className="tab"
            activeProps={{ className: 'tab-active', role: 'tab' }}
            inactiveProps={{ role: 'tab' }}
          >
            <CertificatesIcon className="w-5 h-5 mr-2 fill-current" />
            Certificates
          </Link>
        </div>
      </header>
      <div className="p-4 w-full">
        <Outlet />
      </div>

      <CreateAppModal
        modalRef={createAppRef}
        closeModal={closeCreateApp}
        environmentId={environmentId}
      />
      <CreateCertificateModal
        modalRef={createCertRef}
        closeModal={closeCreateCert}
        environmentId={environmentId}
      />
    </>
  )
}
