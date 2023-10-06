use once_cell::sync::Lazy;
use scraper::Html;
use scraper::Selector;
use std::collections::HashMap;

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
}

/// Sigi state
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SigiState {
    /*
    /// ?
    #[serde(rename = "AppContext")]
    pub app_context: serde_json::Value,

    /// ?
    #[serde(rename = "ItemModule")]
    pub item_module: ItemModule,
    */
    /// Extra k/vs
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
