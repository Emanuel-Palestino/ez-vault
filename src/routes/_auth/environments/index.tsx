import { createFileRoute, Link } from '@tanstack/react-router'
import { useModal } from '../../../components/ui/Modal'
import { CreateEnvModal } from '../../../components/CreateEnvModal'
import { useGetEnvironments } from '../../../services/storage'
import ArrowRightIcon from '../../../icons/ArrowRight'
import PlusIcon from '../../../icons/Plus'

export const Route = createFileRoute('/_auth/environments/')({
  component: EnvironmentsPage,
})

function EnvironmentsPage() {
  const { environments } = useGetEnvironments()
  const { modalRef, open, close } = useModal()

  return (
    <div className="p-4">
      <header className="flex items-center justify-between mb-6">
        <div>
          <h2 className="text-2xl font-bold">Environments</h2>
          <p className="text-sm text-gray-500">
            {environments?.length || 0} environments created
          </p>
        </div>
        <button className="btn btn-primary btn-sm" onClick={open}>
          <PlusIcon className="w-3 h-3 fill-white" />
          New Environment
        </button>
      </header>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
        {environments?.map((env) => (
          <Link
            key={env.id}
            to="/environments/$environmentId"
            params={{ environmentId: env.id }}
            className="card card-sm bg-base-100 shadow hover:shadow-md transition-shadow cursor-pointer"
          >
            <div className="card-body flex-row items-center">
              <div className="h-full grow">
                <h3 className="card-title">{env.name}</h3>
                {env.note && (
                  <p className="text-base-content/60 text-sm">{env.note}</p>
                )}
              </div>
              <ArrowRightIcon className="w-5 h-5" />
            </div>
          </Link>
        ))}
      </div>

      <CreateEnvModal modalRef={modalRef} closeModal={close} />
    </div>
  )
}
