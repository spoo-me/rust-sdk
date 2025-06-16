const URL_REGEX: &str = r#"^(ftp|http|https):\/\/[^ "]+$"#;
const ALIAS_REGEX: &str = r"^[a-zA-Z0-9_-]*$";

/// Validate password format (≥8 chars, contains letter, digit, '@' or '.', no consecutive special chars).
pub fn is_valid_password(pw: &str) -> bool {
    let len_ok = pw.len() >= 8;
    let has_letter = pw.chars().any(|c| c.is_alphabetic());
    let has_digit = pw.chars().any(|c| c.is_ascii_digit());
    let has_special = pw.chars().any(|c| c == '@' || c == '.');
    let no_consec =
        !pw.contains("..") && !pw.contains("@@") && !pw.contains("@.") && !pw.contains(".@");
    len_ok && has_letter && has_digit && has_special && no_consec
}

/// Validate URL format (http/https/ftp, no base url or ".." in path).
#[cfg(feature = "custom_url")]
pub fn is_valid_url(url: &str, base_url: &str) -> bool {
    let re = regex::Regex::new(URL_REGEX).unwrap();
    re.is_match(url) && !url.contains(base_url) && !url.contains("..")
}

/// Validate URL format (http/https/ftp, no "spoo.me" or ".." in path).
#[cfg(not(feature = "custom_url"))]
pub fn is_valid_url(url: &str) -> bool {
    let re = regex::Regex::new(URL_REGEX).unwrap();
    re.is_match(url) && !url.contains("spoo.me") && !url.contains("..")
}

/// Validate alias format (alphanumeric, underscores, hyphens, max 15 chars).
pub fn is_valid_alias(alias: &str) -> bool {
    let re = regex::Regex::new(ALIAS_REGEX).unwrap();
    re.is_match(alias) && !alias.is_empty() && alias.len() <= 15
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        assert!(is_valid_password("Valid@123"));
        assert!(!is_valid_password("short"));
        assert!(!is_valid_password("NoSpecial123"));
        assert!(!is_valid_password("NoDigit@"));
        assert!(!is_valid_password("Invalid@@@"));
    }

    #[cfg(feature = "custom_url")]
    #[test]
    fn test_valid_url() {
        assert!(is_valid_url("https://example.com", "spoo.me"));
        assert!(is_valid_url("ftp://example.com", "spoo.me"));
        assert!(!is_valid_url("https://spoo.me/test", "spoo.me"));
        assert!(!is_valid_url("https://example.com/..", "spoo.me"));
    }

    #[cfg(not(feature = "custom_url"))]
    #[test]
    fn test_valid_url() {
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("ftp://example.com"));
        assert!(is_valid_url("https://example.com/long/url"));
        assert!(!is_valid_url("https://spoo.me/test"));
        assert!(!is_valid_url("https://example.com/.."));
    }

    #[test]
    fn test_valid_alias() {
        assert!(is_valid_alias("valid_alias"));
        assert!(is_valid_alias("valid123"));
        assert!(!is_valid_alias("invalid alias"));
        assert!(!is_valid_alias("too_long_alias_12345"));
        assert!(!is_valid_alias(""));
    }
}
