import { Link } from '@tanstack/react-router'
import { FC } from 'react'

export const Sidebar: FC = () => {
  return (
    <div className="min-w-50 h-dvh sticky top-0 z-50 bg-base-100 p-3 flex flex-col gap-1 shadow">
      <header>
        <h1 className="text-lg font-bold text-center">ez Vault</h1>
      </header>

      <div className="divider m-0" />

      <nav className="grow">
        <ul className="menu w-full px-1">
          <li>
            <Link to="/environments" activeProps={{ className: 'menu-active' }}>
              Environments
            </Link>
          </li>
          {/* <li>
              <Link to="settings">Settings</Link>
            </li> */}
        </ul>
      </nav>

      <div className="divider m-0" />

      <div className="navbar-end pr-2">
        <span>email.com</span>
      </div>
    </div>
  )
}
