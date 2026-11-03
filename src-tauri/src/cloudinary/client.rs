/// Shared handle for all Cloudinary API calls.
pub struct CloudinaryClient {
    pub client: reqwest::Client,
    pub cloud_name: String,
    pub api_key: String,
    pub api_secret: String,
}

impl CloudinaryClient {
    /// Builds a client from the environment (loaded via dotenvy in `run()`).
    pub fn new() -> Result<Self, String> {
        // TODO:
        // - read CLOUDINARY_CLOUD_NAME, CLOUDINARY_API_KEY, CLOUDINARY_API_SECRET
        //   with std::env::var, mapping a missing var to a descriptive Err(String)
        // - construct reqwest::Client::new()
        todo!()
    }
}
