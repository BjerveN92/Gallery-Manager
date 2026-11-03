use tauri::State;

use crate::cloudinary::client::CloudinaryClient;

#[tauri::command]
pub async fn list_images(
    client: State<'_, CloudinaryClient>,
    folder: String,
) -> Result<serde_json::Value, String> {
    let request = client
        .client
        .get(client.url("resources/by_asset_folder"))
        .basic_auth(&client.api_key, Some(&client.api_secret))
        .query(&[
            ("asset_folder", folder.as_str()),
            ("metadata", "true"),
            ("max_results", "500"),
        ]);
    client.send(request).await
}
