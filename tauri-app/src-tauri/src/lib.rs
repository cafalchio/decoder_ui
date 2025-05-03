use cdr_decoder::test_cdr_extraction;

#[tauri::command]
fn run_extraction(name: &str) -> String {
    let result: String = test_cdr_extraction(name.to_string());
    result
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Running tauri");
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(
            tauri::generate_handler![run_extraction]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
