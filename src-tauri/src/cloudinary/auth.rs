use sha1::{Digest, Sha1};
use std::time::{SystemTime, UNIX_EPOCH};

fn generate_signature(params: &str, api_secret: &str) -> String {
    let to_sign = format!("{}{}", params, api_secret);
    let hash = Sha1::digest(to_sign.as_bytes());
    hex::encode(hash)
}

fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
