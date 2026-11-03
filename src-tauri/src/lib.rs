mod cloudinary;
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load .env at runtime; a missing file is fine (vars may come from the real environment).
    let _ = dotenvy::dotenv();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::list::list_images,
            commands::upload::upload_image,
            commands::delete::delete_image,
            commands::metadata::update_order,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
