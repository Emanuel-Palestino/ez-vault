import { invoke } from '@tauri-apps/api/core'
import { TAURI_CMD } from '../utils/constants'

export interface CreateVaultParams {
  email: string
  tursoUrl: string
  tursoToken: string
  masterPassword: string
}

export interface JoinVaultParams {
  email: string
  tursoUrl: string
  tursoToken: string
  masterPassword: string
  inviteCode: string
}

export const createVault = (params: CreateVaultParams) =>
  invoke<void>(TAURI_CMD.CREATE_VAULT, {
    email: params.email,
    tursoUrl: params.tursoUrl,
    tursoToken: params.tursoToken,
    masterPassword: params.masterPassword,
  })

export const joinVault = (params: JoinVaultParams) =>
  invoke<void>(TAURI_CMD.JOIN_VAULT, {
    email: params.email,
    tursoUrl: params.tursoUrl,
    tursoToken: params.tursoToken,
    masterPassword: params.masterPassword,
    inviteCode: params.inviteCode,
  })

export const unlock = (masterPassword: string) =>
  invoke<void>(TAURI_CMD.UNLOCK, { masterPassword })

export const inviteUser = (inviteeEmail: string) =>
  invoke<string>(TAURI_CMD.INVITE_USER, { inviteeEmail })
