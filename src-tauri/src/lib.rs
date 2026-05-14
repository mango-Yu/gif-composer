mod gif_export;

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditionInfo {
    pub pro: bool,
    pub max_images: Option<u32>,
}

#[tauri::command]
fn edition_info() -> EditionInfo {
    EditionInfo {
        pro: true,
        max_images: None,
    }
}

#[tauri::command]
fn create_gif(paths: Vec<String>, delay_ms: u32, output_path: String) -> Result<(), String> {
    gif_export::create_gif_from_paths(&paths, delay_ms, &output_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![edition_info, create_gif])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
