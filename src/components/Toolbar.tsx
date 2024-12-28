import { FC } from 'react'

interface ToolbarProps {
  openCreateApp: () => void
}

export const Toolbar: FC<ToolbarProps> = ({ openCreateApp }) => {
  return (
    <section className="menu menu-horizontal bg-base-200 rounded-box absolute right-2 top-2">
      <button className="btn">Settings</button>
      <button className="btn" onClick={openCreateApp}>
        New
      </button>
    </section>
  )
}
