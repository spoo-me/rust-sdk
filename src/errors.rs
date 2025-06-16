/// Errors that can occur when sending requests (client validation or HTTP errors).
#[derive(Debug)]
pub enum ValidationError {
    /// Password does not meet format requirements.
    InvalidPasswordFormat(String),
    /// Alias does not meet format requirements
    InvalidAliasFormat(String),
    /// URL does not meet format requirements.
    InvalidUrlFormat(String),
    /// Max-clicks must be a positive integer.
    InvalidMaxClicks(u32),
    /// Emoji sequence is invalid
    InvalidEmojiSequence(String),
}

/// Errors that can occur when interacting with the spoo.me API.
#[derive(Debug)]
pub enum ApiError {
    /// The URL does not match the expected format.
    UrlError,
    /// The alias is already in use or invalid.
    AliasError,
    /// The password provided is incorrect.
    PasswordError,
    /// The max clicks value is invalid.
    MaxClicksError,
    /// The emoji sequence is already in use or invalid.
    EmojiError,
}

/// Errors that can occur when using the URL shortener client.
#[derive(Debug)]
pub enum UrlShortenerError {
    /// Validation errors related to the request parameters.
    Validation(ValidationError),
    /// Errors returned by the spoo.me API.
    Api(ApiError),
    /// Errors related to the HTTP request, such as connection issues or timeouts.
    Http(reqwest::Error),
    /// Errors related to JSON serialization or deserialization.
    Json(serde_json::Error),
    /// Other unexpected status codes or errors.
    Other(String),
}
