import { useState } from 'react'
import { App } from '../types/entities'
import { useModal } from './ui/Modal'
import { AppDetails } from './AppDetails'
import { useGetApps } from '../services/storage'

export const MainContent = () => {
  const { modalRef, open, close } = useModal()
  const [selectedApp, setSelectedApp] = useState<App | null>(null)

  const { apps } = useGetApps()

  const handleOpenDetails = (app: App) => {
    setSelectedApp(app)
    open()
  }

  return (
    <>
      <section className="container mx-auto mt-6">
        <div className="flex flex-wrap gap-4 justify-center">
          {apps.map((app) => (
            <div
              key={app.id}
              className="card basis-xs bg-base-200 shadow-sm cursor-pointer"
              onClick={() => handleOpenDetails(app)}
            >
              <div className="card-body p-6 prose prose-headings:mt-0 prose-headings:mb-0">
                <div className="flex gap-2">
                  <span className="badge badge-xs badge-primary">default</span>
                  <span className="badge badge-xs badge-accent">bc</span>
                </div>
                <h2>{app.name}</h2>
                <p>{app.note}</p>
              </div>
            </div>
          ))}
        </div>
      </section>

      {selectedApp && (
        <AppDetails
          appDetailsRef={modalRef}
          closeAppDetails={close}
          app={selectedApp}
        />
      )}
    </>
  )
}
