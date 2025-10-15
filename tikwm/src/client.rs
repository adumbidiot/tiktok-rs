use crate::Error;
use crate::model::CreateDownloadTaskResponse;
use crate::model::GetTaskResultResponse;
use crate::model::json::GetTaskResultResponse as GetTaskResultResponseJson;
use crate::model::json::SubmitTaskResponse;

/// A Client
#[derive(Debug)]
pub struct Client {
    /// The inner http client
    pub client: reqwest::Client,
}

impl Client {
    /// Make a new client.
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Create a video download task on the server.
    pub async fn create_download_task(
        &self,
        video_url: &str,
    ) -> Result<CreateDownloadTaskResponse, Error> {
        let url = "https://www.tikwm.com/api/video/task/submit";
        let response: SubmitTaskResponse = self
            .client
            .post(url)
            .form(&[("url", video_url), ("web", "1")])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response.into())
    }

    /// Get a task result.
    pub async fn get_task_result(&self, task_id: &str) -> Result<GetTaskResultResponse, Error> {
        let url = format!("https://www.tikwm.com/api/video/task/result?task_id={task_id}");
        let response: GetTaskResultResponseJson = self
            .client
            .get(url.as_str())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response.into())
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
