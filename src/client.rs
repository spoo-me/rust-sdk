/// A client for the URL shortener API.
use crate::{
    errors::{ApiError, UrlShortenerError, ValidationError},
    requests::{
        EmojiRequest, EmojiResponse, ExportRequest, ExportResponse, ShortenRequest,
        ShortenResponse, StatsRequest, StatsResponse,
    },
    utils::{is_valid_alias, is_valid_max_clicks, is_valid_password, is_valid_url},
};

/// A client for the URL shortener API.
///
/// This client can be used in both async and blocking modes, depending on the feature flags.
///
/// # Example usage:
/// ```rust
/// use spoo_me::client::UrlShortenerClient;
/// use spoo_me::requests::ShortenRequest;
/// use spoo_me::errors::UrlShortenerError;
///
/// #[cfg(not(feature = "blocking"))]
/// #[tokio::main]
/// async fn main() -> Result<(), UrlShortenerError> {
///     let client = UrlShortenerClient::new();
///     let request = ShortenRequest::new("https://example.com/long/url")
///         .password("Example@123")
///         .max_clicks(100)
///         .block_bots(true);
///
///     let response = client.shorten(request).await?;
///     println!("Shortened URL: {}", response.short_url);
///     Ok(())
/// }
///
/// #[cfg(feature = "blocking")]
/// fn main() -> Result<(), UrlShortenerError> {
///     let client = UrlShortenerClient::new();
///     let request = ShortenRequest::new("https://example.com/long/url")
///         .password("Example@123")
///         .max_clicks(100)
///         .block_bots(true);
///
///     let response = client.shorten_blocking(request)?;
///     println!("Shortened URL: {}", response.short_url);
///     Ok(())
/// }
#[derive(Debug, Clone)]
pub struct UrlShortenerClient {
    base_url: String,
    #[cfg(not(feature = "blocking"))]
    client: reqwest::Client,
    #[cfg(feature = "blocking")]
    client: reqwest::blocking::Client,
}

impl UrlShortenerClient {
    /// Create a new client
    pub fn new() -> Self {
        UrlShortenerClient {
            base_url: "https://spoo.me".to_string(),
            #[cfg(not(feature = "blocking"))]
            client: reqwest::Client::new(),
            #[cfg(feature = "blocking")]
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Create a new client with a custom base URL
    ///
    /// Requires the `custom_url` feature to be enabled.
    #[cfg(feature = "custom_url")]
    pub fn new_with_base_url<S: Into<String>>(url: S) -> Self {
        UrlShortenerClient {
            base_url: url.into(),
            #[cfg(not(feature = "blocking"))]
            client: reqwest::Client::new(),
            #[cfg(feature = "blocking")]
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Set a custom base URL for the client.
    ///
    /// Requires the `custom_url` feature to be enabled.
    #[cfg(feature = "custom_url")]
    pub fn set_base_url<T: Into<String>>(&mut self, url: T) {
        self.base_url = url.into();
    }

    /// Shorten a URL (async mode).
    #[cfg(not(feature = "blocking"))]
    pub async fn shorten(&self, req: ShortenRequest) -> Result<ShortenResponse, UrlShortenerError> {
        if let Some(ref pw) = req.password {
            if !is_valid_password(pw) {
                return Err(UrlShortenerError::Validation(
                    ValidationError::InvalidPasswordFormat(pw.clone()),
                ));
            }
        }

        #[cfg(feature = "custom_url")]
        if !is_valid_url(&req.url, &self.base_url) {
            return Err(UrlShortenerError::Validation(
                ValidationError::InvalidUrlFormat(req.url.clone()),
            ));
        }
        #[cfg(not(feature = "custom_url"))]
        if !is_valid_url(&req.url) {
            return Err(UrlShortenerError::Validation(
                ValidationError::InvalidUrlFormat(req.url.clone()),
            ));
        }

        if let Some(ref alias) = req.alias {
            if !is_valid_alias(alias) {
                return Err(UrlShortenerError::Validation(
                    ValidationError::InvalidAliasFormat(alias.clone()),
                ));
            }
        }

        if let Some(max_clicks) = req.max_clicks {
            if !is_valid_max_clicks(max_clicks) {
                return Err(UrlShortenerError::Validation(
                    ValidationError::InvalidMaxClicks(max_clicks),
                ));
            }
        }

        let resp = self
            .client
            .post(format!("{}/", self.base_url))
            .header("Accept", "application/json")
            .form(&req)
            .send()
            .await
            .map_err(UrlShortenerError::Http)?;

        let status = resp.status();
        let text = resp.text().await.map_err(UrlShortenerError::Http)?;
        if !status.is_success() {
            if status.as_u16() == 429 {
                return Err(UrlShortenerError::Api(ApiError::RateLimitExceeded));
            }

            if let Ok(err_json) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Some(err) = err_json.get("error").and_then(|e| e.as_str()) {
                    return Err(UrlShortenerError::Api(match err {
                        "UrlError" => ApiError::UrlError,
                        "AliasError" => ApiError::AliasError,
                        "PasswordError" => ApiError::PasswordError,
                        "MaxClicksError" => ApiError::MaxClicksError,
                        "EmojiError" => ApiError::EmojiError,
                        _ => ApiError::UrlError,
                    }));
                }
            }
            return Err(UrlShortenerError::Other(text));
        }

        let result =
            serde_json::from_str::<ShortenResponse>(&text).map_err(UrlShortenerError::Json)?;

        Ok(result)
    }

    /// Shorten a URL (blocking mode).
    #[cfg(feature = "blocking")]
    pub fn shorten_blocking(
        &self,
        req: ShortenRequest,
    ) -> Result<ShortenResponse, UrlShortenerError> {
        if let Some(ref pw) = req.password {
            if !is_valid_password(pw) {
                return Err(UrlShortenerError::Validation(
                    ValidationError::InvalidPasswordFormat(pw.clone()),
                ));
            }
        }

        #[cfg(feature = "custom_url")]
        if !is_valid_url(&req.url, &self.base_url) {
            return Err(UrlShortenerError::Validation(
                ValidationError::InvalidUrlFormat(req.url.clone()),
            ));
        }
        #[cfg(not(feature = "custom_url"))]
        if !is_valid_url(&req.url) {
            return Err(UrlShortenerError::Validation(
                ValidationError::InvalidUrlFormat(req.url.clone()),
            ));
        }

        if let Some(ref alias) = req.alias {
            if !is_valid_alias(alias) {
                return Err(UrlShortenerError::Validation(
                    ValidationError::InvalidAliasFormat(alias.clone()),
                ));
            }
        }

        if let Some(max_clicks) = req.max_clicks {
            if !is_valid_max_clicks(max_clicks) {
                return Err(UrlShortenerError::Validation(
                    ValidationError::InvalidMaxClicks(max_clicks),
                ));
            }
        }

        let resp = self
            .client
            .post(format!("{}/", self.base_url))
            .header("Accept", "application/json")
            .form(&req)
            .send()
            .map_err(UrlShortenerError::Http)?;

        let status = resp.status();
        let text = resp.text().map_err(UrlShortenerError::Http)?;
        if !status.is_success() {
            if status.as_u16() == 429 {
                return Err(UrlShortenerError::Api(ApiError::RateLimitExceeded));
            }

            if let Ok(err_json) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Some(err) = err_json.get("error").and_then(|e| e.as_str()) {
                    return Err(UrlShortenerError::Api(match err {
                        "UrlError" => ApiError::UrlError,
                        "AliasError" => ApiError::AliasError,
                        "PasswordError" => ApiError::PasswordError,
                        "MaxClicksError" => ApiError::MaxClicksError,
                        "EmojiError" => ApiError::EmojiError,
                        _ => ApiError::UrlError,
                    }));
                }
            }
            return Err(UrlShortenerError::Other(text));
        }

        let result =
            serde_json::from_str::<ShortenResponse>(&text).map_err(UrlShortenerError::Json)?;

        Ok(result)
    }

    /// Create an emoji URL (async mode).
    #[cfg(not(feature = "blocking"))]
    pub async fn emoji(&self, req: EmojiRequest) -> Result<EmojiResponse, UrlShortenerError> {
        if let Some(ref pw) = req.password {
            if !is_valid_password(pw) {
                return Err(UrlShortenerError::Validation(
                    ValidationError::InvalidPasswordFormat(pw.clone()),
                ));
            }
        }

        #[cfg(feature = "custom_url")]
        if !is_valid_url(&req.url, &self.base_url) {
            return Err(UrlShortenerError::Validation(
                ValidationError::InvalidUrlFormat(req.url.clone()),
            ));
        }
        #[cfg(not(feature = "custom_url"))]
        if !is_valid_url(&req.url) {
            return Err(UrlShortenerError::Validation(
                ValidationError::InvalidUrlFormat(req.url.clone()),
            ));
        }

        if let Some(max_clicks) = req.max_clicks {
            if !is_valid_max_clicks(max_clicks) {
                return Err(UrlShortenerError::Validation(
                    ValidationError::InvalidMaxClicks(max_clicks),
                ));
            }
        }

        let resp = self
            .client
            .post(format!("{}/emoji", self.base_url))
            .header("Accept", "application/json")
            .form(&req)
            .send()
            .await
            .map_err(UrlShortenerError::Http)?;

        let status = resp.status();
        let text = resp.text().await.map_err(UrlShortenerError::Http)?;
        if !status.is_success() {
            if status.as_u16() == 429 {
                return Err(UrlShortenerError::Api(ApiError::RateLimitExceeded));
            }

            if let Ok(err_json) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Some(err) = err_json.get("error").and_then(|e| e.as_str()) {
                    return Err(UrlShortenerError::Api(match err {
                        "UrlError" => ApiError::UrlError,
                        "AliasError" => ApiError::AliasError,
                        "PasswordError" => ApiError::PasswordError,
                        "MaxClicksError" => ApiError::MaxClicksError,
                        "EmojiError" => ApiError::EmojiError,
                        err => ApiError::Other(err.to_string()),
                    }));
                }
            }
            return Err(UrlShortenerError::Other(text));
        }

        let result =
            serde_json::from_str::<EmojiResponse>(&text).map_err(UrlShortenerError::Json)?;

        Ok(result)
    }

    /// Create an emoji URL (blocking mode).
    #[cfg(feature = "blocking")]
    pub fn emoji_blocking(&self, req: EmojiRequest) -> Result<EmojiResponse, UrlShortenerError> {
        if let Some(ref pw) = req.password {
            if !is_valid_password(pw) {
                return Err(UrlShortenerError::Validation(
                    ValidationError::InvalidPasswordFormat(pw.clone()),
                ));
            }
        }

        #[cfg(feature = "custom_url")]
        if !is_valid_url(&req.url, &self.base_url) {
            return Err(UrlShortenerError::Validation(
                ValidationError::InvalidUrlFormat(req.url.clone()),
            ));
        }
        #[cfg(not(feature = "custom_url"))]
        if !is_valid_url(&req.url) {
            return Err(UrlShortenerError::Validation(
                ValidationError::InvalidUrlFormat(req.url.clone()),
            ));
        }

        if let Some(max_clicks) = req.max_clicks {
            if !is_valid_max_clicks(max_clicks) {
                return Err(UrlShortenerError::Validation(
                    ValidationError::InvalidMaxClicks(max_clicks),
                ));
            }
        }

        let resp = self
            .client
            .post(format!("{}/emoji", self.base_url))
            .header("Accept", "application/json")
            .form(&req)
            .send()
            .map_err(UrlShortenerError::Http)?;

        let status = resp.status();
        let text = resp.text().map_err(UrlShortenerError::Http)?;
        if !status.is_success() {
            if status.as_u16() == 429 {
                return Err(UrlShortenerError::Api(ApiError::RateLimitExceeded));
            }

            if let Ok(err_json) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Some(err) = err_json.get("error").and_then(|e| e.as_str()) {
                    return Err(UrlShortenerError::Api(match err {
                        "UrlError" => ApiError::UrlError,
                        "AliasError" => ApiError::AliasError,
                        "PasswordError" => ApiError::PasswordError,
                        "MaxClicksError" => ApiError::MaxClicksError,
                        "EmojiError" => ApiError::EmojiError,
                        _ => ApiError::UrlError,
                    }));
                }
            }
            return Err(UrlShortenerError::Other(text));
        }

        let result =
            serde_json::from_str::<EmojiResponse>(&text).map_err(UrlShortenerError::Json)?;

        Ok(result)
    }

    /// Get statistics for a shortened URL (async mode).
    #[cfg(not(feature = "blocking"))]
    pub async fn stats(&self, req: StatsRequest) -> Result<StatsResponse, UrlShortenerError> {
        if req.short_code.is_empty() {
            return Err(UrlShortenerError::Validation(
                ValidationError::InvalidPasswordFormat("Short code cannot be empty".to_string()),
            ));
        }

        if let Some(ref pw) = req.password {
            if !is_valid_password(pw) {
                return Err(UrlShortenerError::Validation(
                    ValidationError::InvalidPasswordFormat(pw.clone()),
                ));
            }
        }

        if !is_valid_alias(&req.short_code) {
            return Err(UrlShortenerError::Validation(
                ValidationError::InvalidAliasFormat(req.short_code.clone()),
            ));
        }

        let resp = self
            .client
            .post(format!("{}/stats/{}", self.base_url, req.short_code))
            .header("Accept", "application/json")
            .form(&req)
            .send()
            .await
            .map_err(UrlShortenerError::Http)?;

        let status = resp.status();
        let text = resp.text().await.map_err(UrlShortenerError::Http)?;
        if !status.is_success() {
            if status.as_u16() == 429 {
                return Err(UrlShortenerError::Api(ApiError::RateLimitExceeded));
            }

            if let Ok(err_json) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Some(err) = err_json.get("error").and_then(|e| e.as_str()) {
                    return Err(UrlShortenerError::Api(match err {
                        "UrlError" => ApiError::UrlError,
                        "AliasError" => ApiError::AliasError,
                        "PasswordError" => ApiError::PasswordError,
                        "MaxClicksError" => ApiError::MaxClicksError,
                        "EmojiError" => ApiError::EmojiError,
                        _ => ApiError::UrlError,
                    }));
                }
            }
            return Err(UrlShortenerError::Other(text));
        }

        let result =
            serde_json::from_str::<StatsResponse>(&text).map_err(UrlShortenerError::Json)?;

        Ok(result)
    }

    /// Get statistics for a shortened URL (blocking mode).
    #[cfg(feature = "blocking")]
    pub fn stats_blocking(&self, req: StatsRequest) -> Result<StatsResponse, UrlShortenerError> {
        if req.short_code.is_empty() {
            return Err(UrlShortenerError::Validation(
                ValidationError::InvalidPasswordFormat("Short code cannot be empty".to_string()),
            ));
        }

        if let Some(ref pw) = req.password {
            if !is_valid_password(pw) {
                return Err(UrlShortenerError::Validation(
                    ValidationError::InvalidPasswordFormat(pw.clone()),
                ));
            }
        }

        if !is_valid_alias(&req.short_code) {
            return Err(UrlShortenerError::Validation(
                ValidationError::InvalidAliasFormat(req.short_code.clone()),
            ));
        }

        let resp = self
            .client
            .post(format!("{}/stats/{}", self.base_url, req.short_code))
            .header("Accept", "application/json")
            .form(&req)
            .send()
            .map_err(UrlShortenerError::Http)?;

        let status = resp.status();
        let text = resp.text().map_err(UrlShortenerError::Http)?;
        if !status.is_success() {
            if status.as_u16() == 429 {
                return Err(UrlShortenerError::Api(ApiError::RateLimitExceeded));
            }

            if let Ok(err_json) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Some(err) = err_json.get("error").and_then(|e| e.as_str()) {
                    return Err(UrlShortenerError::Api(match err {
                        "UrlError" => ApiError::UrlError,
                        "AliasError" => ApiError::AliasError,
                        "PasswordError" => ApiError::PasswordError,
                        "MaxClicksError" => ApiError::MaxClicksError,
                        "EmojiError" => ApiError::EmojiError,
                        _ => ApiError::UrlError,
                    }));
                }
            }
            return Err(UrlShortenerError::Other(text));
        }

        let result =
            serde_json::from_str::<StatsResponse>(&text).map_err(UrlShortenerError::Json)?;

        Ok(result)
    }

    /// Export data for a shortened URL (async mode).
    #[cfg(not(feature = "blocking"))]
    pub async fn export(&self, req: ExportRequest) -> Result<ExportResponse, UrlShortenerError> {
        if req.short_code.is_empty() {
            return Err(UrlShortenerError::Validation(
                ValidationError::InvalidAliasFormat(req.short_code),
            ));
        }

        if !is_valid_alias(&req.short_code) {
            return Err(UrlShortenerError::Validation(
                ValidationError::InvalidAliasFormat(req.short_code.clone()),
            ));
        }

        let resp = self
            .client
            .post(format!(
                "{}/export/{}/{}",
                self.base_url, req.short_code, req.export_format
            ))
            .form(&req)
            .send()
            .await
            .map_err(UrlShortenerError::Http)?;

        let status = resp.status();
        if !status.is_success() {
            if status.as_u16() == 429 {
                return Err(UrlShortenerError::Api(ApiError::RateLimitExceeded));
            }

            let text = resp.text().await.map_err(UrlShortenerError::Http)?;
            if let Ok(err_json) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Some(err) = err_json.get("error").and_then(|e| e.as_str()) {
                    return Err(UrlShortenerError::Api(match err {
                        "UrlError" => ApiError::UrlError,
                        "AliasError" => ApiError::AliasError,
                        "PasswordError" => ApiError::PasswordError,
                        "MaxClicksError" => ApiError::MaxClicksError,
                        "EmojiError" => ApiError::EmojiError,
                        _ => ApiError::Other(err.to_string()),
                    }));
                }
            }
            return Err(UrlShortenerError::Other(text));
        }

        let data = resp.bytes().await.map_err(UrlShortenerError::Http)?;
        let result = ExportResponse {
            data: data.to_vec(),
        };

        Ok(result)
    }

    /// Export data for a shortened URL (blocking mode).
    #[cfg(feature = "blocking")]
    pub fn export_blocking(&self, req: ExportRequest) -> Result<ExportResponse, UrlShortenerError> {
        if req.short_code.is_empty() {
            return Err(UrlShortenerError::Validation(
                ValidationError::InvalidAliasFormat(req.short_code),
            ));
        }

        if !is_valid_alias(&req.short_code) {
            return Err(UrlShortenerError::Validation(
                ValidationError::InvalidAliasFormat(req.short_code.clone()),
            ));
        }

        let resp = self
            .client
            .post(format!(
                "{}/export/{}/{}",
                self.base_url, req.short_code, req.export_format
            ))
            .form(&req)
            .send()
            .map_err(UrlShortenerError::Http)?;

        let status = resp.status();
        if !status.is_success() {
            if status.as_u16() == 429 {
                return Err(UrlShortenerError::Api(ApiError::RateLimitExceeded));
            }

            let text = resp.text().map_err(UrlShortenerError::Http)?;
            if let Ok(err_json) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Some(err) = err_json.get("error").and_then(|e| e.as_str()) {
                    return Err(UrlShortenerError::Api(match err {
                        "UrlError" => ApiError::UrlError,
                        "AliasError" => ApiError::AliasError,
                        "PasswordError" => ApiError::PasswordError,
                        "MaxClicksError" => ApiError::MaxClicksError,
                        "EmojiError" => ApiError::EmojiError,
                        _ => ApiError::Other(err.to_string()),
                    }));
                }
            }
            return Err(UrlShortenerError::Other(text));
        }

        let data = resp.bytes().map_err(UrlShortenerError::Http)?;
        let result = ExportResponse {
            data: data.to_vec(),
        };

        Ok(result)
    }
}

impl Default for UrlShortenerClient {
    fn default() -> Self {
        Self::new()
    }
}
