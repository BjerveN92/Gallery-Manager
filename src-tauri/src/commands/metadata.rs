use tauri::State;

use crate::cloudinary::auth::{generate_signature, timestamp};
use crate::cloudinary::client::CloudinaryClient;

#[tauri::command]
pub async fn update_order(
    client: State<'_, CloudinaryClient>,
    public_id: String,
    order: u32,
) -> Result<serde_json::Value, String> {
    let timestamp = timestamp().to_string();
    let metadata = format!("order={order}");
    let signature = generate_signature(
        &format!("metadata={metadata}&public_id={public_id}&timestamp={timestamp}&type=upload"),
        &client.api_secret,
    );

    let request = client.client.post(client.url("image/explicit")).form(&[
        ("public_id", public_id.as_str()),
        ("type", "upload"),
        ("metadata", metadata.as_str()),
        ("timestamp", timestamp.as_str()),
        ("api_key", client.api_key.as_str()),
        ("signature", signature.as_str()),
    ]);
    client.send(request).await
}
