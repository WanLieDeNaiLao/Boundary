// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn test_rust(name: String) -> String {
    format!("你好，{}！这句话来自 Rust 底层。", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![test_rust])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}