use crate::model::json::SubmitTaskResponse;
use std::time::Duration;
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum InvalidJsonError {
    #[error("invalid url")]
    InvalidUrl(#[from] url::ParseError),
}

#[derive(Debug)]
pub struct CreateDownloadTaskResponse {
    /// The id of the video
    pub id: u64,

    /// The title of the video
    pub title: String,

    /// The download url of the video
    pub download_url: Option<Url>,

    /// The play url of the video
    pub play_url: Option<Url>,

    /// The duration of the video.
    pub duration: Duration,

    /// The task id of the download.
    pub task_id: String,
}

impl TryFrom<SubmitTaskResponse> for CreateDownloadTaskResponse {
    type Error = InvalidJsonError;

    fn try_from(value: SubmitTaskResponse) -> Result<Self, Self::Error> {
        let detail = value.data.detail;

        let id = detail.id;
        let title = detail.title;
        let download_url = (!detail.download_url.is_empty())
            .then_some(Url::parse(&detail.download_url))
            .transpose()?;
        let play_url = (!detail.play_url.is_empty())
            .then_some(Url::parse(&detail.play_url))
            .transpose()?;
        let task_id = value.data.task_id;
        let duration = Duration::from_secs(detail.duration);

        Ok(Self {
            id,
            title,
            download_url,
            play_url,
            task_id,
            duration,
        })
    }
}
