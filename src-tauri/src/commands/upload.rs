use tauri::State;

use crate::cloudinary::client::CloudinaryClient;

#[tauri::command]
pub async fn upload_image(
    client: State<'_, CloudinaryClient>,
    file_path: String,
    folder: String,
    order: u32,
) -> Result<serde_json::Value, String> {
    let _ = (&client, file_path, folder, order);
    // TODO:
    // - use `client` (managed state)
    // - timestamp = auth::timestamp()
    // - signature = auth::generate_signature("folder=..&metadata=order=..&timestamp=..", api_secret)
    // - POST https://api.cloudinary.com/v1_1/{cloud_name}/image/upload as multipart:
    //     file (read from file_path), folder, metadata="order={order}",
    //     timestamp, api_key, signature
    // - parse the response body as serde_json::Value
    todo!()
}
