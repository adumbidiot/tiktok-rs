use crate::Error;
use crate::FeedCursor;
use rand::RngExt;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use std::time::SystemTime;
use url::Url;
use uuid::Uuid;

const API_HOST: &str = "api22-normal-c-useast2a.tiktokv.com";

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
        // let app_name = "musical_ly";
        let version_name = "34.1.2";
        let version_code = "2023401020";

        let url = format!("https://{API_HOST}/aweme/v1/feed/");
        // This should always be valid
        let mut url = Url::parse(&url).unwrap();
        {
            let mut rng = rand::rng();
            let epoch_seconds = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|duration| duration.as_secs())
                .unwrap_or(0);
            let hex_slice = [
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
            ];
            let hex_distribution = rand::distr::slice::Choose::new(&hex_slice).unwrap();

            let mut query_pairs = url.query_pairs_mut();

            if let Some(video_id) = video_id {
                query_pairs.append_pair("aweme_id", itoa::Buffer::new().format(video_id));
            }

            query_pairs.append_pair("version_name", version_name);
            query_pairs.append_pair("ab_version", version_name);
            query_pairs.append_pair("version_code", version_code);
            query_pairs.append_pair("build_number", version_name);
            query_pairs.append_pair("manifest_version_code", version_code);
            query_pairs.append_pair("update_version_code", version_code);

            query_pairs.append_pair("iid", "7351149742343391009");
            let device_id = rng.random_range(7250000000000000000_u64..7351147085025500000_u64);
            query_pairs.append_pair("device_id", itoa::Buffer::new().format(device_id));
            query_pairs.append_pair("region", "US");
            query_pairs.append_pair("os", "android");
            query_pairs.append_pair("device_type", "Pixel 7");
            query_pairs.append_pair("device_brand", "Google");
            query_pairs.append_pair("language", "en");
            query_pairs.append_pair("os_version", "13");
            query_pairs.append_pair("ts", itoa::Buffer::new().format(epoch_seconds));
            let last_install_time = epoch_seconds.saturating_sub(rng.random_range(86400..1123200));
            query_pairs.append_pair(
                "last_install_time",
                itoa::Buffer::new().format(last_install_time),
            );
            query_pairs.append_pair("_rticket", itoa::Buffer::new().format(epoch_seconds * 1000));
            query_pairs.append_pair("channel", "googleplay");
            let openudid: String = rng.sample_iter(hex_distribution).take(16).collect();
            query_pairs.append_pair("openudid", openudid.as_str());
            query_pairs.append_pair("aid", "0");
            let cdid = Uuid::new_v4();
            query_pairs.append_pair("cdid", &cdid.to_string());
        }

        let package = format!("com.zhiliaoapp.musically/{version_code}"); // com.ss.android.ugc.{app_name}/{version_code}
        let user_agent = format!("{package} (Linux; U; Android 13; en_US; Pixel 7; Build/TD1A.220804.031; Cronet/58.0.2991.0)");

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
