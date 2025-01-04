import { invoke } from '@tauri-apps/api/core'
import {
  App,
  Environment,
  NewApp,
  NewCredential,
  NewEnvironment,
  NewPort,
  NewSecret,
  Port,
  Secret,
} from '../types/entities'
import useSWR from 'swr'
import { FETCHER, TAURI_CMD } from '../utils/constants'

export const createEnvironment = async (environment: NewEnvironment) => {
  await invoke('command_create_environment', { environment })
}

export const useGetEnvironments = () => {
  const { data, error, isLoading } = useSWR(
    { cmd: TAURI_CMD.GET_ENVIRONMENTS },
    FETCHER<Environment[]>,
    { fallbackData: [] },
  )
  return {
    environments: data,
    isError: error,
    isLoading: isLoading,
  }
}

export const createApp = async (app: NewApp) => {
  await invoke('command_create_app', { app })
}

export const useGetApps = () => {
  const { data, error, isLoading } = useSWR(
    { cmd: TAURI_CMD.GET_APPS },
    FETCHER<App[]>,
    { fallbackData: [] },
  )
  return {
    apps: data,
    isError: error,
    isLoading: isLoading,
  }
}

export const createPort = async (port: NewPort) => {
  await invoke('command_create_port', { port })
}

export const getPorts = async () => {
  return await invoke<Port[]>('command_get_ports')
}

export const getPortsByAppId = async (appId: string) => {
  return await invoke<Port[]>('command_get_ports_by_app_id', { appId })
}

export const createCredential = async (credential: NewCredential) => {
  await invoke('command_create_credential', { credential })
}

export const getCredentialsByAppId = async (appId: string) => {
  return await invoke<Credential[]>('command_get_credentials_by_app_id', {
    appId,
  })
}

export const createSecret = async (secret: NewSecret) => {
  await invoke('command_create_secret', { secret })
}

export const getSecretsByAppId = async (appId: string) => {
  return await invoke<Secret[]>('command_get_secrets_by_app_id', { appId })
}
