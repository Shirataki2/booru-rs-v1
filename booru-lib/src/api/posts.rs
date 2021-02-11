use derive_builder::Builder;
use crate::http::{BooruClient, API_ENDPOINT};
use crate::models::{Post};
use crate::models::serde::SpaceDelimited;
use crate::error::BooruError;

#[derive(Debug, Clone, Serialize)]
pub enum ResponseFormat {
    #[serde(rename = "json")] Json,
    #[serde(rename = "html")] Html,
    #[serde(rename = "xml")] Xml,
    #[serde(rename = "atom")] Atom,
}

#[derive(Debug, Clone, Serialize, Default, Builder)]
#[builder(setter(into))]
pub struct PostsFetchRequest {
    #[builder(default = "20")]
    limit: u64,
    #[builder(default = "1")]
    page: u64,
    #[builder(default)]
    tags: SpaceDelimited,
    #[builder(default)]
    random: bool,
    #[builder(default)]
    format: Option<ResponseFormat>,
    #[builder(default)]
    md5: Option<String>,
}

pub trait Posts {
    fn get_posts(&self, query: PostsFetchRequest) -> Result<Vec<Post>, BooruError>;
}

impl Posts for BooruClient {
    fn get_posts(&self, query: PostsFetchRequest) -> Result<Vec<Post>, BooruError> {
        let url = API_ENDPOINT.join("posts.json")?;
        let req = self.get_request_with_account(url)?.json(&query);
        let res = req.send()?;
        match res.status().as_u16() {
            200 => Ok(res.json()?),
            _ => Err(BooruError::APIError(res.text()?)),
        }
    }
}
