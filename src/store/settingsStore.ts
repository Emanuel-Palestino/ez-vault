import { create } from "zustand";
import { StorageType } from "../types/settings";
import { LazyStore } from "@tauri-apps/plugin-store";

type SettingsStore = {
  storageType: StorageType
  databaseUrl: string;
}

type SettingsStoreActions = {
  setStorageType: (type: StorageType) => Promise<void>;
  setDatabaseUrl: (url: string) => Promise<void>;
}

export type SettingsStoreState = SettingsStore & SettingsStoreActions;

const STORE_FILE = "store.json"
const store = new LazyStore(STORE_FILE, { autoSave: false })

export const useSettingsStore = create<SettingsStoreState>()(
  (set) => ({
    databaseUrl: "",
    storageType: StorageType.LOCAL,

    setStorageType: async (type: StorageType) => {
      set({ storageType: type })
      await store.set("storageType", type)
    },
    setDatabaseUrl: async (url: string) => {
      set({ databaseUrl: url })
      await store.set("databaseUrl", url)
    },
  })
)

// set initial state from store

store.get<string>("storageType").then((storageType) => {
  if (storageType) {
    useSettingsStore.setState({ storageType: storageType as StorageType })
  }
})

store.get<string>("databaseUrl").then((databaseUrl) => {
  if (databaseUrl) {
    useSettingsStore.setState({ databaseUrl })
  }
})
