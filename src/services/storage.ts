import { invoke } from '@tauri-apps/api/core'
import { Credential, Secret } from '../types/etities'

export async function saveCredential(credential: Credential) {
  await invoke('save_credential', {
    url: 'http://localhost:3000',
    username: credential.username,
    password: credential.password,
    note: 'description',
  })
}

export async function saveSecret(secret: Secret) {
  await invoke('save_secret', {
    key: secret.key,
    value: secret.value,
    note: secret.description,
  })
}
