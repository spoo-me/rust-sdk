use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

/// Response for URL-shortening endpoints (`/` and `/emoji`).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortenResponse {
    /// The resulting shortened URL (full URL).
    pub short_url: String,
    /// The domain name used for the short URL - currently always "spoo.me".
    pub domain: String,
    /// The URL that was shortened.
    pub original_url: String,
}

/// Request payload for `POST /` (shorten URL).
#[derive(Debug, Serialize, Default, Clone)]
pub struct ShortenRequest {
    pub(crate) url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) password: Option<String>,
    #[serde(rename = "max-clicks", skip_serializing_if = "Option::is_none")]
    pub(crate) max_clicks: Option<u32>,
    #[serde(rename = "block-bots", skip_serializing_if = "Option::is_none")]
    pub(crate) block_bots: Option<bool>,
}

impl ShortenRequest {
    /// Creates a new ShortenRequest with the mandatory `url`.
    pub fn new<U: Into<String>>(url: U) -> Self {
        ShortenRequest {
            url: url.into(),
            ..Default::default()
        }
    }
    /// Optional custom alias (must follow API rules)
    pub fn alias<A: Into<String>>(mut self, alias: A) -> Self {
        self.alias = Some(alias.into());
        self
    }
    /// Optional password (must follow API rules).
    pub fn password<P: Into<String>>(mut self, password: P) -> Self {
        self.password = Some(password.into());
        self
    }
    /// Optional max-clicks (must be positive).
    pub fn max_clicks(mut self, max: u32) -> Self {
        self.max_clicks = Some(max);
        self
    }
    /// Optional block bots flag.
    pub fn block_bots(mut self, flag: bool) -> Self {
        self.block_bots = Some(flag);
        self
    }
}

/// Request payload for `POST /emoji` (uses emojis as slug).
#[derive(Debug, Serialize, Default, Clone)]
pub struct EmojiRequest {
    pub(crate) url: String,
    #[serde(rename = "emojies", skip_serializing_if = "Option::is_none")]
    pub(crate) emojies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) password: Option<String>,
    #[serde(rename = "max-clicks", skip_serializing_if = "Option::is_none")]
    pub(crate) max_clicks: Option<u32>,
    #[serde(rename = "block-bots", skip_serializing_if = "Option::is_none")]
    pub(crate) block_bots: Option<bool>,
}

impl EmojiRequest {
    /// Creates a new EmojiRequest with the mandatory `url`.
    pub fn new<U: Into<String>>(url: U) -> Self {
        EmojiRequest {
            url: url.into(),
            ..Default::default()
        }
    }
    /// Optional emoji sequence (must follow API rules).
    pub fn emojies<E: Into<String>>(mut self, seq: E) -> Self {
        self.emojies = Some(seq.into());
        self
    }
    /// Optional password (must follow API rules).
    pub fn password<P: Into<String>>(mut self, password: P) -> Self {
        self.password = Some(password.into());
        self
    }
    /// Optional max-clicks (must be positive).
    pub fn max_clicks(mut self, max: u32) -> Self {
        self.max_clicks = Some(max);
        self
    }
    /// Optional block bots flag.
    pub fn block_bots(mut self, flag: bool) -> Self {
        self.block_bots = Some(flag);
        self
    }
}

/// Response struct for `POST /emoji`, containing the shortened URL.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmojiResponse {
    /// The resulting shortened URL (full URL).
    pub short_url: String,
    /// The domain name used for the short URL - currently always "spoo.me".
    pub domain: String,
    /// The URL that was shortened.
    pub original_url: String,
}

/// Request payload for `POST /stats/{shortCode}`.
#[derive(Debug, Serialize, Default, Clone)]
pub struct StatsRequest {
    #[serde(skip_serializing)]
    pub(crate) short_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) password: Option<String>,
}

impl StatsRequest {
    /// Create a StatsRequest (optionally with password).
    pub fn new(short_code: &str) -> Self {
        StatsRequest {
            short_code: short_code.to_string(),
            password: None,
        }
    }
    /// Optional password for accessing stats (if set on the short URL).
    pub fn password<P: Into<String>>(mut self, password: P) -> Self {
        self.password = Some(password.into());
        self
    }
}

/// Response struct for `POST /stats/{shortCode}`, containing URL statistics.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatsResponse {
    /// The code of the short URL.
    pub short_code: String,
    /// The original long URL.
    pub url: String,
    /// Total clicks since creation.
    #[serde(rename = "total-clicks")]
    pub total_clicks: u32,
    /// Total unique clicks.
    pub total_unique_clicks: u32,
    /// Creation date (string) of the short link, if available.
    #[serde(rename = "creation-date")]
    pub creation_date: Option<String>,
    /// Whether the link has expired.
    pub expired: Option<bool>,
    /// Last click timestamp (if any).
    #[serde(rename = "last-click")]
    pub last_click: Option<String>,
    /// Last browser used.
    #[serde(rename = "last-click-browser")]
    pub last_click_browser: Option<String>,
    /// Last OS used.
    #[serde(rename = "last-click-os")]
    pub last_click_os: Option<String>,
    /// Max clicks allowed (if set).
    #[serde(rename = "max-clicks")]
    pub max_clicks: Option<u32>,
    /// The password set on the short URL (if any).
    pub password: Option<String>,
    /// Whether bots were blocked.
    pub block_bots: Option<bool>,
    /// Click data per bot type.
    pub bots: Option<HashMap<String, u32>>,
    /// Click data per browser.
    pub browser: Option<HashMap<String, u32>>,
    /// Click data per country.
    pub country: Option<HashMap<String, u32>>,
    /// Clicks per day.
    pub counter: Option<HashMap<String, u32>>,
    /// Unique clicks per browser.
    pub unique_browser: Option<HashMap<String, u32>>,
    /// Unique clicks per country.
    pub unique_country: Option<HashMap<String, u32>>,
    /// Unique clicks per day.
    pub unique_counter: Option<HashMap<String, u32>>,
    /// Unique clicks per OS name.
    pub unique_os_name: Option<HashMap<String, u32>>,
    /// Unique clicks per referrer.
    pub unique_referrer: Option<HashMap<String, u32>>,
}

/// Enum representing the available export formats.
#[derive(Debug, Deserialize, Clone)]
pub enum ExportFormat {
    /// Export as JSON.
    JSON,
    /// Export as CSV, zipped together.
    CSV,
    /// Export as XLSX (Excel format).
    XLSX,
    /// Export as XML.
    XML,
}

impl Display for ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportFormat::JSON => write!(f, "json"),
            ExportFormat::CSV => write!(f, "csv"),
            ExportFormat::XLSX => write!(f, "xlsx"),
            ExportFormat::XML => write!(f, "xml"),
        }
    }
}

/// Request payload for `POST /export/{shortCode}/{exportFormat}`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportRequest {
    /// The short code of the URL to export.
    #[serde(skip_serializing)]
    pub(crate) short_code: String,
    #[serde(skip_serializing)]
    pub(crate) export_format: ExportFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) password: Option<String>,
}

impl ExportRequest {
    /// Creates a new ExportRequest with the mandatory `short_code` and `export_format`.
    pub fn new<S: Into<String>>(short_code: S, export_format: ExportFormat) -> Self {
        ExportRequest {
            short_code: short_code.into(),
            export_format,
            password: None,
        }
    }

    /// Optional password for accessing the export (if set on the short URL).
    pub fn password<P: Into<String>>(mut self, password: P) -> Self {
        self.password = Some(password.into());
        self
    }
}

/// Implementation for creating an export request.
#[derive(Debug, Clone)]
pub struct ExportResponse {
    /// The raw data returned
    pub(crate) data: Vec<u8>,
}

impl ExportResponse {
    /// Writes the export data to a file at the specified path.
    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(path)?;
        file.write_all(&self.data)?;
        Ok(())
    }

    /// Returns the raw data of the export.
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
