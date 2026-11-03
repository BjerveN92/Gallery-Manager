use tauri::State;

use crate::cloudinary::client::CloudinaryClient;

#[tauri::command]
pub async fn update_order(
    client: State<'_, CloudinaryClient>,
    public_id: String,
    order: u32,
) -> Result<serde_json::Value, String> {
    let _ = (&client, public_id, order);
    // TODO:
    // - use `client` (managed state)
    // - timestamp = auth::timestamp()
    // - signature = auth::generate_signature("metadata=order=..&public_id=..&timestamp=..", api_secret)
    // - POST to the Cloudinary metadata endpoint for this asset
    //     form: public_id, metadata="order={order}", timestamp, api_key, signature
    // - parse the response body as serde_json::Value
    todo!()
}
