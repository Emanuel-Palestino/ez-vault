import { FC } from 'react'

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
}

export const Toolbar: FC<ToolbarProps> = ({ openCreateApp }) => {
  return (
    <>
      <nav className="w-full flex justify-end sticky top-2 z-50">
        <section className="menu menu-horizontal bg-base-200 rounded-box">
          <button className="btn">Settings</button>
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
          <a>Environment</a>
        </li>
        <li>
          <a onClick={openCreateApp}>Application</a>
        </li>
        <li>
          <a>Port</a>
        </li>
        <li>
          <a>Credential</a>
        </li>
        <li>
          <a>Secret</a>
        </li>
      </ul>
    </>
  )
}
