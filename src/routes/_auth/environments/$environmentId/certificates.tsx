import { createFileRoute } from '@tanstack/react-router'
import { useGetCertificatesByEnvironmentId } from '../../../../services/storage'

export const Route = createFileRoute(
  '/_auth/environments/$environmentId/certificates',
)({
  component: RouteComponent,
})

function RouteComponent() {
  const { environmentId } = Route.useParams()
  const { certificates } = useGetCertificatesByEnvironmentId(environmentId)

  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
      {certificates?.map((cert) => (
        <div key={cert.id} className="card bg-base-100 shadow">
          <div className="card-body">
            <h3 className="card-title text-base">{cert.name}</h3>
            <p className="text-base-content/60 text-sm">
              {cert.fileName}.{cert.fileExtension}
            </p>
          </div>
        </div>
      ))}
    </div>
  )
}
