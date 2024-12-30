export type BaseType = {
  created_at_ts: number
  updated_at_ts: number
  note?: string
}

export type Environment = BaseType & {
  id: string
  name: string
}

export type App = BaseType & {
  id: string
  url: string // js URL type
  environments_ids: string[]
  name: string
  labels: string[]
  bounded_context?: string
}

export type Port = BaseType & {
  id: string
  app_id: string
  value: number
}

export type Credential = BaseType & {
  id: string
  app_id: string
  username: string
  password: string
}

export type Secret = BaseType & {
  id: string
  app_id: string
  key: string
  value: string
}
