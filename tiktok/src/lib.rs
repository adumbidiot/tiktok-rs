mod client;
mod model;

pub use self::client::Client;
pub use self::model::InvalidPostPageError;
pub use self::model::PostPage;
pub use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Reqwest HTTP error
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    /// A Tokio task failed to join
    #[error(transparent)]
    TokioJoin(#[from] tokio::task::JoinError),

    /// Failed to parse a [`PostPage`]
    #[error("invalid post page")]
    InvalidPostPage(#[from] InvalidPostPageError),
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn download_post() {
        let urls = ["https://www.tiktok.com/t/ZTRQsJaw1/"];
        let client = Client::new();
        for url in urls {
            let post = client.get_post(url).await.expect("failed to get post");
            let _item_id = post
                .sigi_state
                .item_module
                .posts
                .keys()
                .next()
                .expect("missing item_id");
            let _download_url = post.get_video_download_url().expect("missing download url");
            // dbg!(download_url);
            
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
}
