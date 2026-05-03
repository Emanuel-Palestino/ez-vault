import { createFileRoute, Link } from '@tanstack/react-router'
import { useGetApps } from '../../../../services/storage'

export const Route = createFileRoute('/_auth/environments/$environmentId/apps')(
  {
    component: RouteComponent,
  },
)

function RouteComponent() {
  const { environmentId } = Route.useParams()
  const { apps } = useGetApps()

  const envApps = apps?.filter((a) => a.environmentId === environmentId) ?? []

  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
      {envApps.map((app) => (
        <Link
          key={app.id}
          to="/apps/$appId"
          params={{ appId: app.id }}
          className="card card-sm bg-base-100 shadow hover:shadow-md transition-shadow cursor-pointer"
        >
          <div className="card-body">
            <h3 className="card-title text-base">{app.name}</h3>
            {app.url && (
              <p className="text-base-content/60 text-sm truncate">{app.url}</p>
            )}
            {app.labels.length > 0 && (
              <div className="flex gap-1 flex-wrap">
                {app.labels.map((l) => (
                  <span key={l} className="badge badge-sm">
                    {l}
                  </span>
                ))}
              </div>
            )}
          </div>
        </Link>
      ))}
    </div>
  )
}
