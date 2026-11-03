use std::env;

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
        Ok(Self {
            client: reqwest::Client::new(),
            cloud_name: read_env("CLOUDINARY_CLOUD_NAME")?,
            api_key: read_env("CLOUDINARY_API_KEY")?,
            api_secret: read_env("CLOUDINARY_API_SECRET")?,
        })
    }
}

fn read_env(name: &str) -> Result<String, String> {
    match env::var(name) {
        Ok(v) if !v.trim().is_empty() => Ok(v),
        _ => Err(format!("missing environment variable {name} (see .env.example)")),
    }
}
