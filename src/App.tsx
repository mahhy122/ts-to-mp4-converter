// src/App.tsx
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog"; // 追加したプラグイン
import "./App.css";

function App() {
  const [filePath, setFilePath] = useState("");
  const [status, setStatus] = useState("");

  // ファイル選択ダイアログを開く関数
  const selectFile = async () => {
    const selected = await open({
      multiple: false,
      filters: [{ name: '動画ファイル', extensions: ['ts'] }]
    });
    // ファイルが選ばれたらパスを保存
    if (selected !== null && !Array.isArray(selected)) {
      setFilePath(selected);
      setStatus("ファイルが選択されました。変換を開始してください。");
    }
  };

  // 変換を実行する関数
  const handleConvert = async () => {
    setStatus("変換中...（しばらくお待ちください）");
    try {
      // 拡張子を .ts から .mp4 に書き換えたパスを作成
      const outPath = filePath.replace(/\.ts$/, ".mp4");
      
      // Rustの関数を呼び出す
      const result = await invoke("convert_ts_to_mp4", { 
        inputPath: filePath, 
        outputPath: outPath 
      });
      
      setStatus(result as string);
    } catch (err) {
      setStatus("エラーが発生しました: " + err);
    }
  };

  return (
    <main className="container">
      <h1>TS to MP4 Converter</h1>
      
      <div style={{ margin: "20px 0" }}>
        <button onClick={selectFile}>TSファイルを選択</button>
        <p style={{ fontSize: "0.9em", color: "#666" }}>
          {filePath ? `選択中: ${filePath}` : "ファイルが選択されていません"}
        </p>
      </div>

      <button onClick={handleConvert} disabled={!filePath}>
        MP4に変換
      </button>

      <p style={{ marginTop: "20px", fontWeight: "bold", whiteSpace: "pre-wrap" }}>
        {status}
      </p>
    </main>
  );
}

export default App;