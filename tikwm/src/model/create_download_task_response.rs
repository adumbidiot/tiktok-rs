use crate::model::json::SubmitTaskResponse;

#[derive(Debug)]
pub struct CreateDownloadTaskResponse {
    /// The id of the video
    pub id: u64,

    /// The title of the video
    pub title: String,

    /// The download url of the video
    pub download_url: Option<String>,

    /// The play url of the video
    pub play_url: Option<String>,

    /// The task id of the download.
    pub task_id: String,
}

impl From<SubmitTaskResponse> for CreateDownloadTaskResponse {
    fn from(value: SubmitTaskResponse) -> Self {
        Self {
            id: value.data.detail.id,
            title: value.data.detail.title,
            download_url: (!value.data.detail.download_url.is_empty())
                .then_some(value.data.detail.download_url),
            play_url: (!value.data.detail.play_url.is_empty())
                .then_some(value.data.detail.play_url),
            task_id: value.data.task_id,
        }
    }
}
