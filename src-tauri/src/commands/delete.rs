use tauri::State;

use crate::cloudinary::client::CloudinaryClient;

#[tauri::command]
pub async fn delete_image(
    client: State<'_, CloudinaryClient>,
    public_id: String,
) -> Result<serde_json::Value, String> {
    let _ = (&client, public_id);
    // TODO:
    // - use `client` (managed state)
    // - timestamp = auth::timestamp()
    // - signature = auth::generate_signature("public_id=..&timestamp=..", api_secret)
    // - POST https://api.cloudinary.com/v1_1/{cloud_name}/resources/image/destroy
    //     form: public_id, timestamp, api_key, signature
    // - parse the response body as serde_json::Value
    todo!()
}
