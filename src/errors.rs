use std::fmt::Display;

use thiserror::Error;

/// Errors that can occur when sending requests (client validation or HTTP errors).
#[derive(Debug, Error)]
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

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidPasswordFormat(msg) => {
                write!(f, "Invalid password format: {}", msg)
            }
            ValidationError::InvalidAliasFormat(msg) => write!(f, "Invalid alias format: {}", msg),
            ValidationError::InvalidUrlFormat(msg) => write!(f, "Invalid URL format: {}", msg),
            ValidationError::InvalidMaxClicks(value) => {
                write!(f, "Max-clicks must be a positive integer, got: {}", value)
            }
            ValidationError::InvalidEmojiSequence(seq) => {
                write!(f, "Invalid emoji sequence: {}", seq)
            }
        }
    }
}

/// Errors that can occur when interacting with the spoo.me API.
#[derive(Debug, Error)]
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
    /// The rate limit for the API has been exceeded.
    RateLimitExceeded,
    /// Other unexpected errors from the API.
    Other(String),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::UrlError => write!(f, "Invalid URL format"),
            ApiError::AliasError => write!(f, "Alias already in use or invalid"),
            ApiError::PasswordError => write!(f, "Incorrect password provided"),
            ApiError::MaxClicksError => write!(f, "Invalid max clicks value"),
            ApiError::EmojiError => write!(f, "Invalid or already used emoji sequence"),
            ApiError::RateLimitExceeded => write!(f, "Rate limit exceeded for the API"),
            ApiError::Other(msg) => write!(f, "API error: {}", msg),
        }
    }
}

/// Errors that can occur when using the URL shortener client.
#[derive(Debug, Error)]
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

impl Display for UrlShortenerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UrlShortenerError::Validation(err) => write!(f, "Validation error: {}", err),
            UrlShortenerError::Api(err) => write!(f, "API error: {:?}", err),
            UrlShortenerError::Http(err) => write!(f, "HTTP error: {}", err),
            UrlShortenerError::Json(err) => write!(f, "JSON error: {}", err),
            UrlShortenerError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}
