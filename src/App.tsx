import { saveCredential, saveSecret } from "./services/storage";

export function App() {
  return (
    <main>
      <h1>ezVault</h1>

      <button
        onClick={() => {
          saveCredential({
            url: "https://example.com",
            username: "john.doe",
            password: "password",
            note: "This is a note",
          })
        }}
      >
        Save Credential
      </button>

      <button
        onClick={() => {
          saveSecret({
            key: "API_KEY",
            value: "1234567890",
            note: "This is a note",
          })
        }}
      >
        Save Secret
      </button>
    </main>
  );
}