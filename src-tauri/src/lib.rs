use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use tauri::{command, Emitter, Runtime};

// 【追加1】Windowsで黒い画面（コンソール）を隠すための機能をインポート
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(Clone, serde::Serialize)]
struct ProgressPayload {
  percentage: f64,
}

// ----------------------------------------------------
// ffprobeを使って動画の長さを秒で取得する関数
// ----------------------------------------------------
fn get_video_duration(input_path: &str) -> Result<f64, String> {
  let mut cmd = Command::new("ffprobe");
  cmd.arg("-v")
    .arg("error")
    .arg("-show_entries")
    .arg("format=duration")
    .arg("-of")
    .arg("default=noprint_wrappers=1:nokey=1")
    .arg(input_path);

  // 【追加2】ffprobeの黒い画面を隠す（Windows限定）
  #[cfg(target_os = "windows")]
  cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

  let output = cmd.output().map_err(|e| format!("ffprobeの起動に失敗しました: {}", e))?;

  if output.status.success() {
    let duration_str = String::from_utf8_lossy(&output.stdout);
    let duration_sec: f64 = duration_str.trim().parse().unwrap_or(1.0);
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

  let total_duration_us = get_video_duration(&input_path)?;

  let mut cmd = Command::new("ffmpeg");
  cmd.arg("-i")
    .arg(&input_path)
    // 【追加3】進捗ログを出力するオプションを復活
    .arg("-progress")
    .arg("-")
    .arg("-c:v")
    .arg("copy")
    .arg("-c:a")
    .arg("aac")
    .arg("-y")
    .arg(&output_path)
    .stdout(Stdio::piped())
    // 【追加4】エラーログは読み捨てて、パイプの詰まり（フリーズ）を防ぐ
    .stderr(Stdio::null());

  // 【追加5】ffmpegの黒い画面を隠す（Windows限定）
  #[cfg(target_os = "windows")]
  cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

  let mut child = cmd.spawn().map_err(|e| e.to_string())?;

  let stdout = child.stdout.take().ok_or("Failed to open stdout")?;
  let reader = BufReader::new(stdout);

  for line in reader.lines() {
    if let Ok(l) = line {
      if l.starts_with("out_time_us=") {
        let us = l.replace("out_time_us=", "").parse::<f64>().unwrap_or(0.0);
        let percentage = (us / total_duration_us * 100.0).min(100.0);
        app.emit("conversion-progress", ProgressPayload { percentage }).unwrap();
      }
    }
  }

  let status = child.wait().map_err(|e| e.to_string())?;
  if status.success() {
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