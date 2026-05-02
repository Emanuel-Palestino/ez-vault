import { invoke, InvokeArgs } from '@tauri-apps/api/core'

export type FetcherArgs = {
  cmd: TAURI_CMD
  args?: InvokeArgs
  //options?: InvokeOptions
}

export const FETCHER = <Data>(args: FetcherArgs) =>
  invoke<Data>(args.cmd, args.args)

export const TAURI_CMD = {
  // Session / setup
  CHECK_SETUP_STATUS: 'command_check_setup_status',
  CREATE_VAULT: 'command_create_vault',
  JOIN_VAULT: 'command_join_vault',
  UNLOCK: 'command_unlock',
  LOCK: 'command_lock',
  GET_CURRENT_USER: 'command_get_current_user',
  INVITE_USER: 'command_invite_user',
  GET_VERSION: 'command_get_version',
  // Environments
  GET_ENVIRONMENTS: 'command_get_environments',
  CREATE_ENVIRONMENT: 'command_create_environment',
  DELETE_ENVIRONMENT: 'command_delete_environment',
  // Apps
  GET_APPS: 'command_get_apps',
  CREATE_APP: 'command_create_app',
  DELETE_APP: 'command_delete_app',
  // Credentials
  GET_CREDENTIALS_BY_APP_ID: 'command_get_credentials_by_app_id',
  CREATE_CREDENTIAL: 'command_create_credential',
  DELETE_CREDENTIAL: 'command_delete_credential',
  // Secrets
  GET_SECRETS_BY_APP_ID: 'command_get_secrets_by_app_id',
  CREATE_SECRET: 'command_create_secret',
  DELETE_SECRET: 'command_delete_secret',
  // Certificates
  GET_CERTIFICATES_BY_ENVIRONMENT_ID:
    'command_get_certificates_by_environment_id',
  CREATE_CERTIFICATE: 'command_create_certificate',
  DELETE_CERTIFICATE: 'command_delete_certificate',
} as const
// Create a type with the values of the object keys
export type TAURI_CMD = (typeof TAURI_CMD)[keyof typeof TAURI_CMD]
