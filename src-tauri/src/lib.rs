// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// src-tauri/src/lib.rs

#[tauri::command]
fn ping_from_rust() -> String {
    "Rustからの通信テスト成功です!".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![ping_from_rust])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
