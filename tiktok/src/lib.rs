mod client;
mod model;

pub use self::client::Client;
pub use self::model::InvalidPostPageError;
pub use self::model::PostPage;

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
    async fn get_post() {
        let urls = ["https://www.tiktok.com/t/ZTRQsJaw1/"];
        let client = Client::new();
        for url in urls {
            let post = client.get_post(url).await.expect("failed to get post");
        }
    }
}
