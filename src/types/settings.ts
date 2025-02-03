export const StorageType = {
  LOCAL: "LOCAL",
  REMOTE: "REMOTE",
  REPLICA: "REPLICA",
} as const
export type StorageType = typeof StorageType[keyof typeof StorageType]
