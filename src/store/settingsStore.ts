import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";
import { tauriStorageEngine } from "./engine";
import { StorageType } from "../types/settings";

type SettingsStore = {
  storageType: StorageType
  databaseUrl: string;
}

type SettingsStoreActions = {
  setStorageType: (type: StorageType) => void;
  setDatabaseUrl: (url: string) => void;
}

export type SettingsStoreState = SettingsStore & SettingsStoreActions;

export const useSettingsStore = create<SettingsStoreState>()(
  persist(
    (set) => ({
      databaseUrl: "",
      storageType: StorageType.LOCAL,

      setStorageType: (type: StorageType) => set({ storageType: type }),
      setDatabaseUrl: (url: string) => set({ databaseUrl: url }),
    }),
    {
      name: "settings-store",
      storage: createJSONStorage(() => tauriStorageEngine),
    }
  )
)
