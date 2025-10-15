use crate::model::json::GetTaskResultResponse as GetTaskResultResponseJson;

#[derive(Debug, serde::Deserialize)]
pub struct GetTaskResultResponse {
    /// The video id
    pub id: u64,

    /// The video title
    pub title: String,

    /// The play url
    pub play_url: String,

    /// The download url
    pub download_url: String,
}

impl From<GetTaskResultResponseJson> for GetTaskResultResponse {
    fn from(value: GetTaskResultResponseJson) -> Self {
        Self {
            id: value.data.detail.id,
            title: value.data.detail.title,
            play_url: value.data.detail.play_url,
            download_url: value.data.detail.download_url,
        }
    }
}
