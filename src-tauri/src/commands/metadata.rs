#[tauri::command]
pub async fn update_order(public_id: String, order: u32) -> Result<serde_json::Value, String> {
    let _ = (public_id, order);
    // TODO:
    // - CloudinaryClient::new()?
    // - timestamp = auth::timestamp()
    // - signature = auth::generate_signature("metadata=order=..&public_id=..&timestamp=..", api_secret)
    // - POST to the Cloudinary metadata endpoint for this asset
    //     form: public_id, metadata="order={order}", timestamp, api_key, signature
    // - parse the response body as serde_json::Value
    todo!()
}
