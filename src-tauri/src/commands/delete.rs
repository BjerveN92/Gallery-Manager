#[tauri::command]
pub async fn delete_image(public_id: String) -> Result<serde_json::Value, String> {
    let _ = public_id;
    // TODO:
    // - CloudinaryClient::new()?
    // - timestamp = auth::timestamp()
    // - signature = auth::generate_signature("public_id=..&timestamp=..", api_secret)
    // - POST https://api.cloudinary.com/v1_1/{cloud_name}/resources/image/destroy
    //     form: public_id, timestamp, api_key, signature
    // - parse the response body as serde_json::Value
    todo!()
}
