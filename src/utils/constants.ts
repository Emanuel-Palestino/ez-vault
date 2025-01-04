import { invoke, InvokeArgs } from '@tauri-apps/api/core'

export const FETCHER = <T>(args: {
  cmd: TAURI_CMD
  args?: InvokeArgs
  //options?: InvokeOptions
}) => invoke<T>(args.cmd, args.args)

export const TAURI_CMD = {
  GET_ENVIRONMENTS: 'command_get_environments',
  GET_APPS: 'command_get_apps',
  GET_PORTS: 'command_get_ports',
  GET_PORTS_BY_APP_ID: 'command_get_ports_by_app_id',
  GET_CREDENTIALS_BY_APP_ID: 'command_get_credentials_by_app_id',
  GET_SECRETS_BY_APP_ID: 'command_get_secrets_by_app_id',
  CREATE_ENVIRONMENT: 'command_create_environment',
  CREATE_APP: 'command_create_app',
  CREATE_PORT: 'command_create_port',
  CREATE_CREDENTIAL: 'command_create_credential',
  CREATE_SECRET: 'command_create_secret',
} as const
// Create a type with the values of the object keys
export type TAURI_CMD = (typeof TAURI_CMD)[keyof typeof TAURI_CMD]
