#[derive(Debug, serde::Deserialize)]
pub(crate) struct GetTaskResultResponseDetail {
    #[serde(deserialize_with = "crate::util::deserialize_from_str")]
    pub id: u64,
    pub title: String,
    pub play_url: String,
    pub download_url: String,
    pub duration: u64,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct GetTaskResultResponseData {
    pub detail: GetTaskResultResponseDetail,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct GetTaskResultResponse {
    pub data: GetTaskResultResponseData,
}
