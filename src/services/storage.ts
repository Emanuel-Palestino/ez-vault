import { invoke } from '@tauri-apps/api/core'
import {
  App,
  Environment,
  NewApp,
  NewCredential,
  Credential,
  NewEnvironment,
  NewPort,
  NewSecret,
  Port,
  Secret,
} from '../types/entities'
import useSWR from 'swr'
import { FETCHER, FetcherArgs, TAURI_CMD } from '../utils/constants'

export const createEnvironment = async (environment: NewEnvironment) => {
  await invoke('command_create_environment', { environment })
}

export const useGetEnvironments = () => {
  const { data, error, isLoading } = useSWR(
    { cmd: TAURI_CMD.GET_ENVIRONMENTS } satisfies FetcherArgs,
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
    { cmd: TAURI_CMD.GET_APPS } satisfies FetcherArgs,
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

export const useGetPortsByAppId = (appId: string) => {
  const { data, error, isLoading } = useSWR(
    {
      cmd: TAURI_CMD.GET_PORTS_BY_APP_ID,
      args: { appId },
    } satisfies FetcherArgs,
    FETCHER<Port[]>,
    { fallbackData: [] },
  )

  return {
    ports: data,
    isError: error,
    isLoading: isLoading,
  }
}

export const createCredential = async (credential: NewCredential) => {
  await invoke('command_create_credential', { credential })
}

export const useGetCredentialsByAppId = (appId: string) => {
  const { data, error, isLoading } = useSWR(
    {
      cmd: TAURI_CMD.GET_CREDENTIALS_BY_APP_ID,
      args: { appId },
    } satisfies FetcherArgs,
    FETCHER<Credential[]>,
    { fallbackData: [] },
  )

  return {
    credentials: data,
    isError: error,
    isLoading: isLoading,
  }
}

export const createSecret = async (secret: NewSecret) => {
  await invoke('command_create_secret', { secret })
}

export const useGetSecretsByAppId = (appId: string) => {
  const { data, error, isLoading } = useSWR(
    {
      cmd: TAURI_CMD.GET_SECRETS_BY_APP_ID,
      args: { appId },
    } satisfies FetcherArgs,
    FETCHER<Secret[]>,
    { fallbackData: [] },
  )

  return {
    secrets: data,
    isError: error,
    isLoading: isLoading,
  }
}
