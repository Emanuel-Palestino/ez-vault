import { createFileRoute, Outlet, redirect } from '@tanstack/react-router'
import { isConfigured } from '../services/storage'
import { Sidebar } from '../components/Sidebar'

export const Route = createFileRoute('/_auth')({
  beforeLoad: async () => {
    const configured = await isConfigured()
    if (!configured) throw redirect({ to: '/setup' })
  },
  component: () => (
    <main className="w-full h-dvh bg-base-200 flex">
      <Sidebar />
      <div className="grow">
        <Outlet />
      </div>
    </main>
  ),
})
