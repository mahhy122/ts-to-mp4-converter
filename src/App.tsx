// src/App.tsx
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [message, setMessage] = useState("");
  
  const testConnection = async () => {
    try {
      const response = await invoke("ping_from_rust");
      setMessage(response as string);
    } catch (error) {
      console.error(error);
    }
  };
  
  return (
    <main className="container">
      <h1>連携テスト</h1>
      <button onClick={testConnection}>
        Rustの処理を呼び出す
      </button>
      <p style={{ marginTop: "20px", color: "#4caf50", fontWeight: "bold" }}>
        {message}
      </p>
    </main>
  );
}

export default App;