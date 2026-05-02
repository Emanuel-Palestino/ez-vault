import { FC } from 'react'
import { useSession } from '../context/SessionContext'

// Extend the react CSSProperties interface to accept popover API and CSS anchor positioning
declare module 'react' {
  interface CSSProperties {
    anchorName?: string
    positionAnchor?: string
  }

  interface HTMLAttributes<T> {
    popovertarget?: string
    popover?: "" | "auto" | "manual" | "hint" | undefined
  }
}

interface ToolbarProps {
  openCreateApp: () => void
  openCreateEnv: () => void
  openCreateCredential: () => void
  openCreateSecret: () => void
  openCreateCertificate: () => void
}

export const Toolbar: FC<ToolbarProps> = ({
  openCreateApp,
  openCreateEnv,
  openCreateCredential,
  openCreateSecret,
  openCreateCertificate,
}) => {
  const { currentUser, lock } = useSession()

  return (
    <>
      <nav className="w-full flex justify-between items-center sticky top-2 z-50">
        <section className="menu menu-horizontal bg-base-300 rounded-box gap-1 px-2 py-1">
          {currentUser && (
            <span className="text-sm text-base-content/60 px-2 self-center">
              {currentUser}
            </span>
          )}
        </section>

        <section className="menu menu-horizontal bg-base-300 rounded-box gap-2">
          <button className="btn btn-ghost btn-sm" onClick={lock}>
            Lock
          </button>
          <button className="btn btn-ghost btn-sm">
            Settings
          </button>
          <button
            className="btn btn-ghost btn-sm"
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
          <a onClick={openCreateCertificate}>Certificate</a>
        </li>
      </ul>
    </>
  )
}
