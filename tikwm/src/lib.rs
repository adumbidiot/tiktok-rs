mod client;
pub mod model;
mod util;

pub use self::client::Client;

/// The library error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("http error")]
    Reqwest(#[from] reqwest::Error),
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn download_video() {
        let url = "https://www.tiktok.com/@barstoolsports/video/7542975753311358221";
        let client = Client::new();

        let create_response = client
            .create_download_task(url)
            .await
            .expect("failed to request download");

        let response = client
            .get_task_result(&create_response.task_id)
            .await
            .expect("failed to get task result");

        let data = client
            .client
            .get(response.download_url)
            .send()
            .await
            .expect("failed to send request")
            .error_for_status()
            .expect("invalid status")
            .bytes()
            .await
            .expect("failed to download");
        std::fs::write("out.mp4", data).expect("failed to write");
    }
}
