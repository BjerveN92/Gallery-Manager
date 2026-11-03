use tauri::State;

use crate::cloudinary::auth::{generate_signature, timestamp};
use crate::cloudinary::client::CloudinaryClient;

#[tauri::command]
pub async fn delete_image(
    client: State<'_, CloudinaryClient>,
    public_id: String,
) -> Result<serde_json::Value, String> {
    let timestamp = timestamp().to_string();
    let signature = generate_signature(
        &format!("public_id={public_id}&timestamp={timestamp}"),
        &client.api_secret,
    );

    let request = client.client.post(client.url("image/destroy")).form(&[
        ("public_id", public_id.as_str()),
        ("timestamp", timestamp.as_str()),
        ("api_key", client.api_key.as_str()),
        ("signature", signature.as_str()),
    ]);
    client.send(request).await
}
