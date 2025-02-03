import { StateStorage } from "zustand/middleware"
import { load } from "@tauri-apps/plugin-store"


const STORE_FILE = "store.json"

export const tauriStorageEngine: StateStorage = {
  getItem: async (name: string): Promise<string | null> => {
    const store = await load(STORE_FILE, { autoSave: false })

    const value = await store.get<{ value: any }>(name)
    return value ? value.value.toString() : null
  },
  setItem: async (name: string, value: string): Promise<void> => {
    const store = await load(STORE_FILE, { autoSave: false })

    await store.set(name, { value })
    await store.save()
  },
  removeItem: async (name: string): Promise<void> => {
    const store = await load(STORE_FILE, { autoSave: false })

    await store.delete(name)
    await store.save()
  }
}
