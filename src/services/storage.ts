import { invoke } from "@tauri-apps/api/core";
import { Credential, Secret } from "../types/storage";

export async function saveCredential(credential: Credential) {
  await invoke("save_credential", { 
    url: credential.url,
    username: credential.username,
    password: credential.password,
    note: credential.note,
   });
}

export async function saveSecret(secret: Secret) {
  await invoke("save_secret", {
    key: secret.key,
    value: secret.value,
    note: secret.note,
  });
}