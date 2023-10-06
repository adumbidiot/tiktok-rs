use url::Url;

/// A cursor for a feed.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FeedCursor {
    /// The list of posts
    pub aweme_list: Vec<Post>,
}

/// A Post
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Post {
    /// The post id
    #[serde(with = "serde_as_string")]
    pub aweme_id: u64,

    /// The description
    pub desc: Box<str>,

    pub video: Video,
}

/// Video data
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Video {
    /// The default video source?
    pub play_addr: VideoSource,

    /// The download video source?
    pub download_addr: VideoSource,
}

/// A video source
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct VideoSource {
    /// A list of urls?
    pub url_list: Vec<Url>,
}

mod serde_as_string {
    pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        use serde::de::Error;
        use serde::Deserialize;

        let value = String::deserialize(deserializer)?;
        value.parse::<T>().map_err(D::Error::custom)
    }

    pub(crate) fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        T: std::fmt::Display,
    {
        let value = value.to_string();
        serializer.serialize_str(&value)
    }
}
