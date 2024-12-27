import { FC } from 'react'

interface ToolbarProps {
  openCreateApp: () => void
}

export const Toolbar: FC<ToolbarProps> = ({ openCreateApp }) => {
  return (
    <section className="bg-neutral-100 w-fit flex gap-2 p-2 rounded-xl absolute right-2 top-2">
      <button className="w-28 h-10 cursor-pointer rounded-lg hover:bg-neutral-200 active:bg-neutral-300 duration-100 transition-colors">
        Settings
      </button>
      <button
        className="w-28 cursor-pointer rounded-lg hover:bg-neutral-200 active:bg-neutral-300 duration-100 transition-colors"
        onClick={openCreateApp}
      >
        New
      </button>
    </section>
  )
}
