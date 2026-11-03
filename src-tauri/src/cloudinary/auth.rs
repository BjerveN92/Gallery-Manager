/// Signs upload/destroy params: SHA1(params + api_secret), hex encoded.
///
/// `params` is the already-sorted `key=value&key=value` string.
pub fn generate_signature(params: &str, api_secret: &str) -> String {
    let _ = (params, api_secret);
    // TODO: Sha1::new(), update(params), update(api_secret), hex::encode(finalize())
    todo!()
}

/// Current unix epoch time in seconds.
pub fn timestamp() -> u64 {
    // TODO: SystemTime::now().duration_since(UNIX_EPOCH).as_secs()
    todo!()
}
