// src-tauri/src/lib.rs
use std::process::Command;

// FFmpegを呼び出して変換する関数
#[tauri::command]
fn convert_ts_to_mp4(input_path: String, output_path: String) -> Result<String, String> {
    // // ffmpeg -i input.ts -c:v libx264 -c:a aac -y output.mp4
    let output = Command::new("ffmpeg")
    .arg("-i")
    .arg(&input_path)
    .arg("-c:v")
    .arg("libx264")
    .arg("-c:a")
    .arg("aac")
    .arg("-y")
    .arg(&output_path)
    .output()
    .map_err(|e| format!("FFmpegの起動に失敗しました: {}", e))?;

    if output.status.success() {
        Ok(format!("変換が完了しました。保存先 : {}", output_path))
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
    Err(error_message.to_string())
    }

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_dialog::init()) // ダイアログプラグインを有効化
    // 先ほど作った convert_ts_to_mp4 をハンドラーに登録
    .invoke_handler(tauri::generate_handler![convert_ts_to_mp4])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}