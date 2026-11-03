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

    /// Full URL for an API path, e.g. `url("image/upload")`.
    pub fn url(&self, path: &str) -> String {
        format!("https://api.cloudinary.com/v1_1/{}/{path}", self.cloud_name)
    }

    /// Sends the request and returns the JSON body, turning HTTP errors into
    /// `Err` with Cloudinary's own error message.
    pub async fn send(&self, request: reqwest::RequestBuilder) -> Result<serde_json::Value, String> {
        let response = request
            .send()
            .await
            .map_err(|e| format!("request to Cloudinary failed: {e}"))?;

        let status = response.status();
        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("invalid response from Cloudinary ({status}): {e}"))?;

        if !status.is_success() {
            let message = body["error"]["message"].as_str().unwrap_or("unknown error");
            return Err(format!("Cloudinary returned {status}: {message}"));
        }
        Ok(body)
    }
}

fn read_env(name: &str) -> Result<String, String> {
    match env::var(name) {
        Ok(v) if !v.trim().is_empty() => Ok(v),
        _ => Err(format!("missing environment variable {name} (see .env.example)")),
    }
}
