import { FC, FormEvent, useEffect, useState } from "react"
import { Modal } from "./ui/Modal"
import { useSettingsStore } from "../store/settingsStore"
import { StorageType } from "../types/settings"
import { updateStorageType } from "../services/storage"

interface SettingsProps {
  closeModal: () => void
  modalRef: React.RefObject<HTMLDialogElement>
}

export const Settings: FC<SettingsProps> = ({
  closeModal,
  modalRef,
}) => {

  const settings = useSettingsStore()

  const [storageType, setStorageType] = useState<StorageType>(settings.storageType)

  const handleStorageSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const data = new FormData(e.currentTarget)

    const type = data.get('type') as StorageType
    const url = data.get('url') as string
    const token = data.get('token') as string

    await settings.setStorageType(type)
    if (type === StorageType.REMOTE || type === StorageType.REPLICA) {
      await settings.setDatabaseUrl(url)
      await updateStorageType(type, url, token)
    }
  }

  useEffect(() => {
    setStorageType(settings.storageType)
  }, [settings.storageType])

  return (
    <Modal ref={modalRef}>
      <h2>Settings</h2>

      <div className="mt-4">
        <form onSubmit={handleStorageSubmit}>
          <h3>Storage</h3>
          <fieldset className="fieldset">
            <legend className="fieldset-legend">Storage type</legend>
            <select
              className="select"
              name="type"
              value={storageType}
              onChange={(e) => setStorageType(e.target.value as StorageType)}
            >
              {Object.keys(StorageType).map((type) => (
                <option key={type} value={type}>
                  {type}
                </option>
              ))}
            </select>
          </fieldset>

          {(storageType === StorageType.REMOTE || storageType === StorageType.REPLICA)
            && (
              <>
                <fieldset className="fieldset">
                  <legend className="fieldset-legend">Database Url *</legend>
                  <input
                    type="text"
                    className="input validator"
                    name="url"
                    placeholder="Turso database url"
                    defaultValue={settings.databaseUrl}
                    required
                    autoComplete="off"
                  />
                </fieldset>

                <fieldset className="fieldset">
                  <legend className="fieldset-legend">Auth token *</legend>
                  <input
                    type="text"
                    className="input validator"
                    name="token"
                    placeholder="Turso auth token"
                    required
                    autoComplete="off"
                  />
                </fieldset>
              </>
            )
          }

          <div className="flex justify-end mt-2">
            <button className="btn btn-sm btn-success" type="submit">
              Save changes
            </button>
          </div>
        </form>
      </div>

      <div className="modal-action">
        <button className="btn btn-warning" onClick={closeModal}>
          Cerrar
        </button>
      </div>
    </Modal>
  )

}
