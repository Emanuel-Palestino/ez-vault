import { invoke } from '@tauri-apps/api/core'
import { Credential, Secret } from '../types/entities'

export async function saveCredential(credential: Omit<Credential, 'id'>) {
  await invoke('save_credential', {
    url: 'http://localhost:3000',
    username: credential.username,
    password: credential.password,
    note: 'description',
  })
}

export async function saveSecret(secret: Omit<Secret, 'id'>) {
  await invoke('save_secret', {
    key: secret.key,
    value: secret.value,
    note: secret.description,
  })
}
