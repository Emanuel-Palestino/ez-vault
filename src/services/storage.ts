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
import useSWR, { mutate } from 'swr'
import { FETCHER, FetcherArgs, TAURI_CMD } from '../utils/constants'

export const createEnvironment = async (environment: NewEnvironment) => {
  await invoke(TAURI_CMD.CREATE_ENVIRONMENT, { environment })
  await mutate({ cmd: TAURI_CMD.GET_ENVIRONMENTS } satisfies FetcherArgs)
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
  await invoke(TAURI_CMD.CREATE_APP, { app })
  await mutate({ cmd: TAURI_CMD.GET_APPS } satisfies FetcherArgs)
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
  await invoke(TAURI_CMD.CREATE_PORT, { port })
  await mutate({ cmd: TAURI_CMD.GET_PORTS } satisfies FetcherArgs)
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
  await invoke(TAURI_CMD.CREATE_CREDENTIAL, { credential })
  await mutate({
    cmd: TAURI_CMD.GET_CREDENTIALS_BY_APP_ID,
    args: { appId: credential.app_id },
  } satisfies FetcherArgs)
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
  await invoke(TAURI_CMD.CREATE_SECRET, { secret })
  await mutate({
    cmd: TAURI_CMD.GET_SECRETS_BY_APP_ID,
    args: { appId: secret.app_id },
  } satisfies FetcherArgs)
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
