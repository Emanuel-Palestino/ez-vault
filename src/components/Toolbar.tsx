import { FC } from 'react'
import { useGetApps, useGetEnvironments, getPorts } from '../services/storage'

// Extend the react CSSProperties interface to accept popover API and CSS anchor positioning
declare module 'react' {
  interface CSSProperties {
    anchorName?: string
    positionAnchor?: string
  }

  interface HTMLAttributes<T> {
    popovertarget?: string
    popover?: 'auto' | 'manual'
  }
}

interface ToolbarProps {
  openCreateApp: () => void
  openCreateEnv: () => void
  openCreateCredential: () => void
  openCreateSecret: () => void
  openCreatePort: () => void
}

export const Toolbar: FC<ToolbarProps> = ({
  openCreateApp,
  openCreateEnv,
  openCreateCredential,
  openCreateSecret,
  openCreatePort,
}) => {
  const { apps } = useGetApps()
  const { environments } = useGetEnvironments()

  const testOnClick = async () => {
    const ports = await getPorts()

    console.log({ environments, apps, ports })
  }

  return (
    <>
      <nav className="w-full flex justify-end sticky top-2 z-50">
        <section className="menu menu-horizontal bg-base-200 rounded-box">
          <button className="btn" onClick={testOnClick}>
            Settings
          </button>
          <button
            className="btn"
            popovertarget="new-menu"
            style={{ anchorName: '--anchor-new-menu' }}
          >
            New
          </button>
        </section>
      </nav>

      <ul
        className="dropdown dropdown-end menu w-40 mt-3 rounded-box bg-base-100 shadow-sm"
        popover="auto"
        id="new-menu"
        style={{ positionAnchor: '--anchor-new-menu' }}
      >
        <li>
          <a onClick={openCreateEnv}>Environment</a>
        </li>
        <li>
          <a onClick={openCreateApp}>Application</a>
        </li>
        <li>
          <a onClick={openCreateCredential}>Credential</a>
        </li>
        <li>
          <a onClick={openCreateSecret}>Secret</a>
        </li>
        <li>
          <a onClick={openCreatePort}>Port</a>
        </li>
      </ul>
    </>
  )
}
