use crate::model::json::GetTaskResultResponse as GetTaskResultResponseJson;
use std::time::Duration;
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum InvalidJsonError {
    #[error("invalid url")]
    InvalidUrl(#[from] url::ParseError),
}

#[derive(Debug, serde::Deserialize)]
pub struct GetTaskResultResponse {
    /// The video id
    pub id: u64,

    /// The video title
    pub title: String,

    /// The play url
    pub play_url: Url,

    /// The download url
    pub download_url: Url,

    /// The video duration
    pub duration: Duration,
}

impl TryFrom<GetTaskResultResponseJson> for GetTaskResultResponse {
    type Error = InvalidJsonError;

    fn try_from(value: GetTaskResultResponseJson) -> Result<Self, Self::Error> {
        let detail = value.data.detail;

        let id = detail.id;
        let title = detail.title;
        let play_url = Url::parse(&detail.play_url)?;
        let download_url = Url::parse(&detail.download_url)?;
        let duration = Duration::from_secs(detail.duration);

        Ok(Self {
            id,
            title,
            play_url,
            download_url,
            duration,
        })
    }
}
