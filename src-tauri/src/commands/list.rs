use tauri::State;

use crate::cloudinary::client::CloudinaryClient;

#[tauri::command]
pub async fn list_images(
    client: State<'_, CloudinaryClient>,
    folder: String,
) -> Result<serde_json::Value, String> {
    let _ = (&client, folder);
    // TODO:
    // - use `client` (managed state)
    // - GET https://api.cloudinary.com/v1_1/{cloud_name}/resources/image
    //     query: prefix={folder}, metadata=true
    //     basic auth: api_key / api_secret
    // - parse the response body as serde_json::Value
    todo!()
}
