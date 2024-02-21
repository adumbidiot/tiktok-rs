use crate::Error;
use crate::FeedCursor;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use url::Url;

const USER_AGENT_STR: &str = "Mozilla/5.0";

static ACCEPT_VALUE: HeaderValue = HeaderValue::from_static("*/*");
static ACCEPT_ENCODING_VALUE: HeaderValue = HeaderValue::from_static("identity;q=1, *;q=0");
static ACCEPT_LANGUAGE_VALUE: HeaderValue = HeaderValue::from_static("en-US,en;q=0.8");

/// A tiktok client
#[derive(Debug, Clone)]
pub struct Client {
    /// The inner HTTP client.
    ///
    /// Should only be used if you want to piggyback off of this for HTTP requests
    pub client: reqwest::Client,
}

impl Client {
    /// Make a new [`Client`].
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::ACCEPT, ACCEPT_VALUE.clone());
        headers.insert(
            reqwest::header::ACCEPT_ENCODING,
            ACCEPT_ENCODING_VALUE.clone(),
        );
        headers.insert(
            reqwest::header::ACCEPT_LANGUAGE,
            ACCEPT_LANGUAGE_VALUE.clone(),
        );

        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT_STR)
            .cookie_store(true)
            // .use_rustls_tls() // native-tls chokes for some reason
            .default_headers(headers)
            .build()
            .expect("failed to build client");

        Self { client }
    }

    /// Get a feed.
    pub async fn get_feed(&self, video_id: Option<u64>) -> Result<FeedCursor, Error> {
        let api_host = "api16-normal-c-useast1a.tiktokv.com";
        let app_name = "trill";
        let version_name = "26.1.3";
        let version_code = "260103";

        let url = format!("https://{api_host}/aweme/v1/feed/");
        // This should always be valid
        let mut url = Url::parse(&url).unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(video_id) = video_id {
                query_pairs.append_pair("aweme_id", itoa::Buffer::new().format(video_id));
            }

            query_pairs.append_pair("version_name", version_name);
            query_pairs.append_pair("version_code", version_code);
            query_pairs.append_pair("build_number", version_name);
            query_pairs.append_pair("manifest_version_code", version_code);
            query_pairs.append_pair("update_version_code", version_code);
        }

        let user_agent = format!("com.ss.android.ugc.{app_name}/{version_code} (Linux; U; Android 13; en_US; Pixel 7; Build/TD1A.220804.031; Cronet/58.0.2991.0)");

        let json = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .header(reqwest::header::USER_AGENT, user_agent)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(json)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
