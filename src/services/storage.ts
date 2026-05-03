import { invoke } from '@tauri-apps/api/core'
import {
  App,
  Environment,
  AppCreate,
  CredentialCreate,
  Credential,
  EnvironmentCreate,
  SaveConfigInput,
  SecretCreate,
  Secret,
  CertificateCreate,
  Certificate,
} from '../types/entities'
import useSWR, { mutate } from 'swr'
import { FETCHER, FetcherArgs, TAURI_CMD } from '../utils/constants'

// region Configuration
export const isConfigured = (): Promise<boolean> =>
  invoke<boolean>(TAURI_CMD.IS_CONFIGURED)

export const saveConfiguration = (config: SaveConfigInput): Promise<void> =>
  invoke(TAURI_CMD.SAVE_CONFIGURATION, { config })
// endregion

// region Environments
export const createEnvironment = async (environment: EnvironmentCreate) => {
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
// endregion

// region Apps
export const createApp = async (app: AppCreate) => {
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
// endregion

// region Credentials
export const createCredential = async (credential: CredentialCreate) => {
  await invoke(TAURI_CMD.CREATE_CREDENTIAL, { credential })
  await mutate({
    cmd: TAURI_CMD.GET_CREDENTIALS_BY_APP_ID,
    args: { appId: credential.appId },
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
// endregion

// region Secrets
export const createSecret = async (secret: SecretCreate) => {
  await invoke(TAURI_CMD.CREATE_SECRET, { secret })
  await mutate({
    cmd: TAURI_CMD.GET_SECRETS_BY_APP_ID,
    args: { appId: secret.appId },
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
// endregion

// region Certificates
export const createCertificate = async (certificate: CertificateCreate) => {
  await invoke(TAURI_CMD.CREATE_CERTIFICATE, { certificate })
  await mutate({
    cmd: TAURI_CMD.GET_CERTIFICATES_BY_ENVIRONMENT_ID,
    args: { environmentId: certificate.environmentId },
  } satisfies FetcherArgs)
}

export const useGetCertificatesByEnvironmentId = (environmentId: string) => {
  const { data, error, isLoading } = useSWR(
    {
      cmd: TAURI_CMD.GET_CERTIFICATES_BY_ENVIRONMENT_ID,
      args: { environmentId },
    } satisfies FetcherArgs,
    FETCHER<Certificate[]>,
    { fallbackData: [] },
  )

  return {
    certificates: data,
    isError: error,
    isLoading: isLoading,
  }
}

export const getCertificatesByEnvironmentId = async (environmentId: string) => {
  return await invoke<Certificate[]>(TAURI_CMD.GET_CERTIFICATES_BY_ENVIRONMENT_ID, {
    environmentId,
  })
}
// endregion
