use chrono::{DateTime, FixedOffset};
use derive_more::{Display, From};

#[derive(Display, From, Debug, Clone, Deserialize)]
pub struct UserId(u64);

#[derive(Display, From, Debug, Clone, Deserialize)]
pub struct PostId(u64);

#[derive(Display, From, Debug, Clone, Deserialize)]
pub struct ArtistId(u64);

pub type Timestamp = DateTime<FixedOffset>;

#[derive(Debug, Clone, Deserialize)]
pub enum Rating {
    #[serde(rename = "s")]
    Safe,
    #[serde(rename = "q")]
    Questionable,
    #[serde(rename = "e")]
    Explicit,
    Unknown,
}

impl Default for Rating {
    fn default() -> Rating {
        Rating::Unknown
    }
}