export type BaseType = {
  created_at_ts: number
  updated_at_ts: number
  note: string
}

export type NewEnvironment = Omit<
  Environment,
  'id' | 'created_at_ts' | 'updated_at_ts'
>

export type Environment = BaseType & {
  id: string
  name: string
}

export type NewApp = Omit<App, 'id' | 'created_at_ts' | 'updated_at_ts'>

export type App = BaseType & {
  id: string
  url: string // js URL type
  environments: Environment[]
  name: string
  labels: string[]
  bounded_context: string
}

export type NewPort = Omit<
  Port,
  'id' | 'created_at_ts' | 'updated_at_ts' | 'app'
> & { app_id: string }

export type Port = BaseType & {
  id: string
  app: App
  value: number
}

export type NewCredential = Omit<
  Credential,
  'id' | 'created_at_ts' | 'updated_at_ts' | 'app'
> & { app_id: string }

export type Credential = BaseType & {
  id: string
  app: App
  username: string
  password: string
}

export type NewSecret = Omit<
  Secret,
  'id' | 'created_at_ts' | 'updated_at_ts' | 'app'
> & { app_id: string }

export type Secret = BaseType & {
  id: string
  app: App
  key: string
  value: string
}
