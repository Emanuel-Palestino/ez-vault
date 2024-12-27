import { saveCredential, saveSecret } from './services/storage'

export function App() {
  return (
    <main>
      <h1>ezVault</h1>

      <button
        onClick={() => {
          saveCredential({
            username: 'john.doe',
            password: 'password',
            app_id: '1234567890',
            created_at_ts: 1234567890,
            updated_at_ts: 1234567890,
          })
        }}
      >
        Save Credential
      </button>

      <button
        onClick={() => {
          saveSecret({
            key: 'API_KEY',
            value: '1234567890',
            description: 'This is a note',
            app_id: '1234567890',
            created_at_ts: 1234567890,
            updated_at_ts: 1234567890,
          })
        }}
      >
        Save Secret
      </button>
    </main>
  )
}
