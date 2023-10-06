use once_cell::sync::Lazy;
use scraper::Html;
use scraper::Selector;
use std::collections::HashMap;
use url::Url;

static SIGI_PERSISTED_DATA_SCRIPT_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("#SIGI_STATE").unwrap());

/// An error that may occur while parsing html
#[derive(thiserror::Error, Debug)]
pub enum FromHtmlError {
    #[error("missing sigi state element")]
    MissingSigiStateElement,

    #[error("missing sigi state")]
    MissingSigiState,

    #[error("invalid sigi state")]
    InvalidSigiState(#[source] serde_json::Error),
}

/// A post page
#[derive(Debug)]
pub struct PostPage {
    /// ?
    pub sigi_state: SigiState,
}

impl PostPage {
    /// Parse a [`PostPage`] from html.
    pub(crate) fn from_html(html: &Html) -> Result<Self, FromHtmlError> {
        let sigi_state_script_str = html
            .select(&SIGI_PERSISTED_DATA_SCRIPT_SELECTOR)
            .next()
            .and_then(|el| el.text().next())
            .ok_or(FromHtmlError::MissingSigiStateElement)?;

        let sigi_state: SigiState =
            serde_json::from_str(sigi_state_script_str).map_err(FromHtmlError::InvalidSigiState)?;

        Ok(Self { sigi_state })
    }

    /// Get the item module post for this post page.
    pub fn get_item_module_post(&self) -> Option<&ItemModulePost> {
        self.sigi_state.item_module.posts.values().next()
    }

    /// Get the video download url for the current post, if it exists.
    pub fn get_video_download_url(&self) -> Option<&Url> {
        let item_module_post = self.get_item_module_post()?;
        let video = &item_module_post.video;

        Some(&video.download_addr)
    }
}

/// Sigi state
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SigiState {
    /// ?
    #[serde(rename = "AppContext")]
    pub app_context: serde_json::Value,

    /// ?
    #[serde(rename = "ItemModule")]
    pub item_module: ItemModule,

    /// Extra k/vs
    #[serde(flatten)]
    pub extra: HashMap<Box<str>, serde_json::Value>,
}

/// ?
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ItemModule {
    /// Posts
    #[serde(flatten)]
    pub posts: HashMap<String, ItemModulePost>,
}

/// ?
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ItemModulePost {
    /// The post id
    pub id: Box<str>,

    /// Post author
    pub author: Box<str>,

    /// Video description
    pub desc: Box<str>,

    /// Nickname?
    pub nickname: Box<str>,

    /// Stats
    pub stats: serde_json::Value,

    /// Video data
    pub video: ItemModulePostVideo,

    /// Extra k/vs
    #[serde(flatten)]
    pub extra: HashMap<Box<str>, serde_json::Value>,
}

/// ?
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ItemModulePostVideo {
    /// Bitrate
    pub bitrate: u32,

    /// Video codec type
    #[serde(rename = "codecType")]
    pub codec_type: Box<str>,

    /// a url?
    pub cover: Url,

    /// video definition?
    pub definition: Box<str>,

    /// The download address?
    #[serde(rename = "downloadAddr")]
    pub download_addr: Url,

    /// video duration, in seconds
    pub duration: u64,

    /// The video ID
    pub id: Box<str>,

    /// The video quality?
    #[serde(rename = "videoQuality")]
    pub video_quality: Box<str>,

    /// main url?
    #[serde(rename = "playAddr")]
    pub play_addr: Url,

    /// Height
    pub height: u64,

    /// Width
    pub width: u64,

    /// Video ratio
    pub ratio: Box<str>,

    /// Video format
    pub format: Box<str>,

    /// A list of values that are empty strings or urls?
    #[serde(rename = "shareCover")]
    pub share_cover: Vec<serde_json::Value>,

    /// ?
    #[serde(rename = "originCover")]
    pub origin_cover: Url,

    /// ?
    #[serde(rename = "encodedType")]
    pub encoded_type: Box<str>,

    /// A `Url` or an empty string?
    #[serde(rename = "reflowCover")]
    pub reflow_cover: Box<str>,

    /// ?
    #[serde(rename = "dynamicCover")]
    pub dynamic_cover: Box<str>,

    /// ?
    #[serde(rename = "encodeUserTag")]
    pub encode_user_tag: Box<str>,

    /// Extra k/vs
    #[serde(flatten)]
    pub extra: HashMap<Box<str>, serde_json::Value>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_sigi_state() {
        let data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test_data/",
            "ZTRQsJaw1.json"
        ));

        let _data: SigiState = serde_json::from_str(data).expect("failed to parse");
    }
}
