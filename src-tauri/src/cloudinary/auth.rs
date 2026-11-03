use std::time::{SystemTime, UNIX_EPOCH};

use sha1::{Digest, Sha1};

/// Signs upload/destroy params: SHA1(params + api_secret), hex encoded.
///
/// `params` is the already-sorted `key=value&key=value` string.
pub fn generate_signature(params: &str, api_secret: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(params.as_bytes());
    hasher.update(api_secret.as_bytes());
    hex::encode(hasher.finalize())
}

/// Current unix epoch time in seconds.
pub fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before the unix epoch")
        .as_secs()
}
