import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import "./App.css";

function App() {
  const [filePath, setFilePath] = useState("");
  const [status, setStatus] = useState("");
  const [progress, setProgress] = useState(0);

  // Rustからの進捗イベントを受け取る設定
  useEffect(() => {
    const unlisten = listen<{ percentage: number }>("conversion-progress", (event) => {
      setProgress(Math.round(event.payload.percentage));
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  const selectFile = async () => {
    const selected = await open({
      multiple: false,
      filters: [{ name: '動画ファイル', extensions: ['ts'] }]
    });
    if (selected !== null && !Array.isArray(selected)) {
      setFilePath(selected);
      setStatus("ファイルが選択されました。");
    }
  };

  const handleConvert = async () => {
    if (!filePath) return;
    setStatus("変換中...");
    setProgress(0);
    
    try {
      const outputPath = filePath.replace(/\.ts$/, ".mp4");
      // 先ほどエラーになっていた変数をしっかり指定
      const result = await invoke("convert_with_progress", { 
        inputPath: filePath, 
        outputPath: outputPath 
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

      {/* 進捗バー */}
      <div style={{ width: "100%", backgroundColor: "#e0e0e0", borderRadius: "4px", margin: "20px 0" }}>
        <div 
          style={{ 
            width: `${progress}%`, 
            height: "20px", 
            backgroundColor: "#4caf50", 
            borderRadius: "4px",
            transition: "width 0.2s" 
          }} 
        />
      </div>
      <p>{progress}% 完了</p>

      <button onClick={handleConvert} disabled={!filePath}>
        MP4に変換
      </button>

      <p style={{ marginTop: "20px", fontWeight: "bold", whiteSpace: "pre-wrap" }}>
        {status}
      </p>
    </main>
  );
}

// 省略してしまっていた「エクスポート（必須）」
export default App;