mod client;
mod model;

pub use self::client::Client;
pub use self::model::FeedCursor;
pub use self::model::Post;
pub use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Reqwest HTTP error
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    /// A Tokio task failed to join
    #[error(transparent)]
    TokioJoin(#[from] tokio::task::JoinError),
}

#[cfg(test)]
mod test {
    use super::*;

    // Broken URLs.
    // Were they deleted?
    // Old URL format?
    // "https://vm.tiktok.com/TTPdrksrdc/"
    // "https://www.tiktok.com/t/ZTRQsJaw1/"
    const POST_URLS: &[&str] = &["https://www.tiktok.com/@von.jakoba/video/7270331232595021098"];

    // The IID we use has been banned.
    // We need to get another one.
    #[tokio::test]
    #[ignore]
    async fn get_feed_post() {
        let client = Client::new();
        for url in POST_URLS {
            let video_id = Url::parse(url)
                .expect("failed to parse url")
                .path_segments()
                .expect("missing path")
                .next_back()
                .expect("missing video id")
                .parse()
                .expect("invalid video id");
            let feed_cursor = client
                .get_feed(Some(video_id))
                .await
                .expect("failed to get post");
            let entry = feed_cursor.aweme_list.first().expect("missing entry");
            assert!(entry.aweme_id == video_id);

            let download_url = entry
                .video
                .download_addr
                .url_list
                .first()
                .expect("missing download url");
            client
                .client
                .get(download_url.as_str())
                .send()
                .await
                .expect("failed to send request")
                .error_for_status()
                .expect("invalid status code")
                .bytes()
                .await
                .expect("failed to download");
        }
    }
}
