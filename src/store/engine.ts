import { StateStorage } from "zustand/middleware"
import { LazyStore } from "@tauri-apps/plugin-store"


const STORE_FILE = "store.json"
const store = new LazyStore(STORE_FILE, { autoSave: false })

export const tauriStorageEngine: StateStorage = {
  getItem: async (name: string): Promise<string | null> => {
    const value = await store.get<{ value: any }>(name)
    return value ? value.value.toString() : null
  },
  setItem: async (name: string, value: string): Promise<void> => {
    await store.set(name, { value })
    await store.save()
  },
  removeItem: async (name: string): Promise<void> => {
    await store.delete(name)
    await store.save()
  }
}
