export type BaseType = {
  created_at_ts: number
  updated_at_ts: number
}

export type Environment = BaseType & {
  id: string
  name: string
  description?: string
}

export type App = BaseType & {
  id: string
  parent_app_id?: string
  environment_id: string
  name: string
  description?: string
  url: string
}

export type Port = BaseType & {
  id: string
  app_id: string
  name: string
  description?: string
  port: number
}

export type Credential = BaseType & {
  app_id: string
  username: string
  password: string
}

export type Secret = BaseType & {
  app_id: string
  key: string
  value: string
  description?: string
}

export type Token = BaseType & {
  app_id: string
  name: string
  description?: string
  token: string
}