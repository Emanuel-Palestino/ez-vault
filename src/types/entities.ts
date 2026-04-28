// IMPORTANT NOTE: If changes are made to this file, you must to sync the changes with the rust types module

export type BaseType = {
  id: string
  createdAtTs: number
  updatedAtTs: number
  note: string
  deleted: boolean
}

export type Environment = BaseType & {
  name: string
}
export type EnvironmentCreate = Omit<
  Environment,
  'id' | 'createdAtTs' | 'updatedAtTs' | 'deleted'
>

export type App = BaseType & {
  url: string // js URL type
  environmentId: string
  name: string
  labels: string[]
}
export type AppCreate = Omit<
  App,
  'id' | 'createdAtTs' | 'updatedAtTs' | 'deleted'
>

export type Credential = BaseType & {
  appId: string
  context: string
  username: string
  password: string | null // password can be null if the app doesn't require a password, e.g. API key, mfa app
  url: string | null
}
export type CredentialCreate = Omit<
  Credential,
  'id' | 'createdAtTs' | 'updatedAtTs' | 'deleted'
>

export type Secret = BaseType & {
  appId: string
  key: string
  value: string
}

export type SecretCreate = Omit<
  Secret,
  'id' | 'createdAtTs' | 'updatedAtTs' | 'deleted'
>

export type Certificate = BaseType & {
  name: string
  fileName: string
  fileExtension: string
  value: string
  environmentId: string
  labels: string[]
}
export type CertificateCreate = Omit<
  Certificate,
  'id' | 'createdAtTs' | 'updatedAtTs' | 'deleted'
>
