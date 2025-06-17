use spoo_me::utils::*;

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
