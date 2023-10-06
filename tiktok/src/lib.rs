mod client;
mod model;

pub use self::client::Client;
pub use self::model::FeedCursor;
pub use self::model::InvalidScrapedPostPageError;
pub use self::model::ScrapedPostPage;
pub use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Reqwest HTTP error
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    /// A Tokio task failed to join
    #[error(transparent)]
    TokioJoin(#[from] tokio::task::JoinError),

    /// Failed to parse a [`ScrapedPostPage`]
    #[error("invalid post page")]
    InvalidScrapedPostPage(#[from] InvalidScrapedPostPageError),
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

    #[tokio::test]
    async fn scrape_post() {
        let client = Client::new();
        for url in POST_URLS {
            let post = client.scrape_post(url).await.expect("failed to get post");
            let _item_id = post
                .sigi_state
                .item_module
                .posts
                .keys()
                .next()
                .expect("missing item_id");
        }
    }

    #[tokio::test]
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
            dbg!(&feed_cursor);
        }
        /*
        client
            .client
            .get(download_url.as_str())
            .send()
            .await
            .expect("failed to send request")
            .error_for_status()
            .expect("invalid status code").bytes().await;*/
    }
}
