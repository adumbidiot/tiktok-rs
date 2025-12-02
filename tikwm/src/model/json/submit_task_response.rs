#[derive(Debug, serde::Deserialize)]
pub(crate) struct SubmitTaskResponseDataDetail {
    #[serde(deserialize_with = "crate::util::deserialize_from_str")]
    pub id: u64,
    pub title: String,
    pub download_url: String,
    pub play_url: String,
    pub duration: u64,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct SubmitTaskResponseData {
    pub detail: SubmitTaskResponseDataDetail,
    pub task_id: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct SubmitTaskResponse {
    pub data: SubmitTaskResponseData,
}
