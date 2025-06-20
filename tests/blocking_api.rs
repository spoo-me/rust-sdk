#[cfg(feature = "blocking")]
#[cfg(test)]
mod blocking_tests {
    use spoo_me::{
        client::UrlShortenerClient,
        requests::{EmojiRequest, ExportFormat, ExportRequest, ShortenRequest, StatsRequest},
    };

    #[test]
    fn test_shorten() {
        let client = UrlShortenerClient::new();
        let request = ShortenRequest::new("https://example.com")
            .password("Test@123")
            .max_clicks(10)
            .block_bots(true);

        let response = client.shorten_blocking(request);
        assert!(
            response.is_ok(),
            "Failed to shorten URL: {:?}",
            response.err()
        );

        let shortened_url = response.unwrap();
        assert!(
            shortened_url.short_url.starts_with("https://spoo.me/"),
            "Shortened URL does not start with expected base URL"
        );
    }

    #[test]
    fn test_emoji() {
        let client = UrlShortenerClient::new();
        let request = EmojiRequest::new("https://example.com")
            .password("Test@123")
            .max_clicks(10)
            .block_bots(true);

        let response = client.emoji_blocking(request);
        assert!(
            response.is_ok(),
            "Failed to create emoji URL: {:?}",
            response.err()
        );

        let emoji_url = response.unwrap();
        assert!(
            emoji_url.short_url.starts_with("https://spoo.me/"),
            "Emoji URL does not start with expected base URL"
        );
    }

    #[test]
    fn test_stats() {
        let client = UrlShortenerClient::new();
        let request = StatsRequest::new("ga"); // Code used for uptime tracking

        let response = client.stats_blocking(request);
        assert!(
            response.is_ok(),
            "Failed to get stats: {:?}",
            response.err()
        );

        let stats = response.unwrap();
        assert_eq!(
            stats.url, "https://google.com",
            "Stats URL does not match expected"
        );
        assert!(
            stats.total_clicks > 0,
            "Stats should have positive click count" // Not all should, but this will
        );
        assert!(
            stats.creation_date.is_some(),
            "Stats should have a creation date"
        );
    }

    #[test]
    fn test_export_json() {
        let client = UrlShortenerClient::new();
        let request = ExportRequest::new("ga", ExportFormat::JSON);
        let response = client.export_blocking(request);

        assert!(
            response.is_ok(),
            "Failed to export stats as JSON: {:?}",
            response.err()
        );
    }

    #[test]
    fn test_export_csv() {
        let client = UrlShortenerClient::new();
        let request = ExportRequest::new("ga", ExportFormat::CSV);
        let response = client.export_blocking(request);

        assert!(
            response.is_ok(),
            "Failed to export stats as JSON: {:?}",
            response.err()
        );
    }

    #[test]
    fn test_export_xlsx() {
        let client = UrlShortenerClient::new();
        let request = ExportRequest::new("ga", ExportFormat::XLSX);
        let response = client.export_blocking(request);

        assert!(
            response.is_ok(),
            "Failed to export stats as JSON: {:?}",
            response.err()
        );
    }

    #[test]
    fn test_export_xml() {
        let client = UrlShortenerClient::new();
        let request = ExportRequest::new("ga", ExportFormat::XML);
        let response = client.export_blocking(request);

        assert!(
            response.is_ok(),
            "Failed to export stats as JSON: {:?}",
            response.err()
        );
    }
}
