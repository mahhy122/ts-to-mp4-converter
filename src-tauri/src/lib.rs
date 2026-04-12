use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use tauri::{command, Emitter, Runtime};

#[derive(Clone, serde::Serialize)]
struct ProgressPayload {
  percentage: f64,
}

// ----------------------------------------------------
// 【新規追加】ffprobeを使って動画の長さを秒で取得する関数
// ----------------------------------------------------
fn get_video_duration(input_path: &str) -> Result<f64, String> {
  // ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1:nokey=1 input.ts
  let output = Command::new("ffprobe")
    .arg("-v")
    .arg("error")
    .arg("-show_entries")
    .arg("format=duration")
    .arg("-of")
    .arg("default=noprint_wrappers=1:nokey=1")
    .arg(input_path)
    .output()
    .map_err(|e| format!("ffprobeの起動に失敗しました: {}", e))?;

  if output.status.success() {
    // 取得した秒数（例: "123.45\n"）を数値に変換
    let duration_str = String::from_utf8_lossy(&output.stdout);
    let duration_sec: f64 = duration_str.trim().parse().unwrap_or(1.0);
    
    // FFmpegの進捗ログ(マイクロ秒)と合わせるために100万を掛ける
    Ok(duration_sec * 1_000_000.0)
  } else {
    Err("動画の長さが取得できませんでした".to_string())
  }
}

#[command]
async fn convert_with_progress<R: Runtime>(
  app: tauri::AppHandle<R>,
  input_path: String,
  output_path: String,
) -> Result<String, String> {

  // 1. 変換を始める前に、本当の総時間を取得する（ここが変わりました！）
  let total_duration_us = get_video_duration(&input_path)?;

  // 2. FFmpegを実行
  let mut child = Command::new("ffmpeg")
    .arg("-i")
    .arg(&input_path)
    .arg("-c:v")
    .arg("copy") // 映像は無劣化コピー
    .arg("-c:a")
    .arg("aac")  // 音声は再エンコード
    .arg("-y")
    .arg(&output_path)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .map_err(|e| e.to_string())?;

  let stdout = child.stdout.take().ok_or("Failed to open stdout")?;
  let reader = BufReader::new(stdout);

  for line in reader.lines() {
    if let Ok(l) = line {
      if l.starts_with("out_time_us=") {
        let us = l.replace("out_time_us=", "").parse::<f64>().unwrap_or(0.0);
        // 本物の総時間を使ってパーセンテージを計算
        let percentage = (us / total_duration_us * 100.0).min(100.0);
        app.emit("conversion-progress", ProgressPayload { percentage }).unwrap();
      }
    }
  }

  let status = child.wait().map_err(|e| e.to_string())?;
  if status.success() {
    // 変換完了時に確実に100%にする
    app.emit("conversion-progress", ProgressPayload { percentage: 100.0 }).unwrap();
    Ok("変換完了！".into())
  } else {
    Err("変換失敗".into())
  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_dialog::init())
    .invoke_handler(tauri::generate_handler![convert_with_progress])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}