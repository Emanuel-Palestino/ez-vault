import { invoke } from '@tauri-apps/api/core'
import { Environment, NewEnvironment } from '../types/entities'

export const createEnvironment = async (environment: NewEnvironment) => {
  await invoke('ez_vault_create_environment', { environment })
}

export const getEnvironments = async () => {
  return await invoke<Environment[]>('ez_vault_get_environments')
}
