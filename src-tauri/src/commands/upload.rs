use tauri::State;

use crate::cloudinary::auth::{generate_signature, timestamp};
use crate::cloudinary::client::CloudinaryClient;

#[tauri::command]
pub async fn upload_image(
    client: State<'_, CloudinaryClient>,
    file_path: String,
    folder: String,
    order: u32,
) -> Result<serde_json::Value, String> {
    let bytes = std::fs::read(&file_path).map_err(|e| format!("could not read {file_path}: {e}"))?;
    let file_name = std::path::Path::new(&file_path)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "upload".to_string());

    let timestamp = timestamp().to_string();
    let metadata = format!("order={order}");
    let signature = generate_signature(
        &format!("asset_folder={folder}&metadata={metadata}&timestamp={timestamp}"),
        &client.api_secret,
    );

    let form = reqwest::multipart::Form::new()
        .part("file", reqwest::multipart::Part::bytes(bytes).file_name(file_name))
        .text("asset_folder", folder)
        .text("metadata", metadata)
        .text("timestamp", timestamp)
        .text("api_key", client.api_key.clone())
        .text("signature", signature);

    let request = client.client.post(client.url("image/upload")).multipart(form);
    client.send(request).await
}
