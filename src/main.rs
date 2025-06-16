use spoome::{
    client::UrlShortenerClient,
    requests::{EmojiRequest, ShortenRequest, StatsRequest},
};

#[cfg(not(feature = "blocking"))]
#[tokio::main]
async fn main() {
    let client = UrlShortenerClient::new();

    let shorten_req = ShortenRequest::new("https://example.com/long/url")
        // .alias("testing_alias")
        .password("Example@123")
        .max_clicks(100)
        .block_bots(true);

    let shorten_resp = client.shorten(shorten_req).await;

    if shorten_resp.is_err() {
        eprintln!("Error shortening URL: {:?}", shorten_resp);
    } else {
        println!("Short URL: {}", shorten_resp.unwrap().short_url);
    }

    let emoji_req = EmojiRequest::new("https://example.com/another/long/url")
        // .emojies("🚀🌟")
        .password("Emoji@123")
        .max_clicks(50);
    let emoji_resp = client.emoji(emoji_req).await;
    if emoji_resp.is_err() {
        eprintln!("Error shortening URL with emoji: {:?}", emoji_resp);
    } else {
        println!("Short URL with emoji: {}", emoji_resp.unwrap().short_url);
    }

    let stats_req = StatsRequest::new("ga");
    let stats_resp = client.stats(stats_req).await;
    if stats_resp.is_err() {
        eprintln!("Error fetching stats: {:?}", stats_resp);
    } else {
        let stats = stats_resp.unwrap();
        println!("Stats for 'ga':");
        println!("Total clicks: {}", stats.total_clicks);
        println!("Total unique clicks: {}", stats.total_unique_clicks);
        if let Some(counter) = &stats.counter {
            println!("Clicks per day: {:?}", counter);
        }
    }
}

#[cfg(feature = "blocking")]
fn main() {
    let client = UrlShortenerClient::new();

    let shorten_req = ShortenRequest::new("https://example.com/long/url")
        // .alias("testing_alias")
        .password("Example@123")
        .max_clicks(100)
        .block_bots(true);

    let shorten_resp = client.shorten_blocking(shorten_req);

    if shorten_resp.is_err() {
        eprintln!("Error shortening URL: {:?}", shorten_resp);
    } else {
        println!("Short URL: {}", shorten_resp.unwrap().short_url);
    }

    let emoji_req = EmojiRequest::new("https://example.com/another/long/url")
        // .emojies("🚀🌟")
        .password("Emoji@123")
        .max_clicks(50);
    let emoji_resp = client.emoji_blocking(emoji_req);
    if emoji_resp.is_err() {
        eprintln!("Error shortening URL with emoji: {:?}", emoji_resp);
    } else {
        println!("Short URL with emoji: {}", emoji_resp.unwrap().short_url);
    }

    let stats_req = StatsRequest::new("ga");
    let stats_resp = client.stats_blocking(stats_req);
    if stats_resp.is_err() {
        eprintln!("Error fetching stats: {:?}", stats_resp);
    } else {
        let stats = stats_resp.unwrap();
        println!("Stats for 'ga':");
        println!("Total clicks: {}", stats.total_clicks);
        println!("Total unique clicks: {}", stats.total_unique_clicks);
        if let Some(counter) = &stats.counter {
            println!("Clicks per day: {:?}", counter);
        }
    }
}
