import { invoke } from '@tauri-apps/api/core'
import { Environment } from '../types/entities'


export const createEnvironment = async (environment: Omit<Environment, 'id' | 'created_at_ts' | 'updated_at_ts'>) => {
  await invoke('ez_vault_create_environment', { environment })
}

export const getEnvironments = async () => {
  return await invoke<Environment[]>('ez_vault_get_environments')
}