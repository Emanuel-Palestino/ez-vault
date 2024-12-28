import { FC } from 'react'

interface ToolbarProps {
  openCreateApp: () => void
}

export const Toolbar: FC<ToolbarProps> = ({ openCreateApp }) => {
  return (
    <nav className='w-full flex justify-end sticky top-2 z-50'>
      <section className="menu menu-horizontal bg-base-200 rounded-box">
        <button className="btn">Settings</button>
        <button className="btn" onClick={openCreateApp}>
          New
        </button>
      </section>
    </nav>
  )
}
