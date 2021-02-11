use derive_builder::Builder;
use crate::http::{BooruClient, API_ENDPOINT};
use crate::models::{Artist, common::ArtistId};
use crate::models::serde::SpaceDelimited;
use crate::error::BooruError;

#[derive(Debug, Clone, Serialize, Default, Builder)]
#[builder(setter(into))]
pub struct ArtistCreateRequest {
    pub name: String,
    #[builder(default)]
    pub group_name: Option<String>,
    #[builder(default)]
    pub other_names: SpaceDelimited,
    #[builder(default)]
    pub url_string: SpaceDelimited,
    #[builder(default)]
    pub is_deleted: bool,
}

pub trait Artists {
    fn get_artists(&self) -> Result<Vec<Artist>, BooruError>;
    
    fn get_artist<Id: Into<ArtistId>>(&self, id: Id) -> Result<Artist, BooruError>;
    
    fn create_artist(&self, body: ArtistCreateRequest) -> Result<Artist, BooruError>;
}

impl Artists for BooruClient {
    fn get_artists(&self) -> Result<Vec<Artist>, BooruError> {
        let url = API_ENDPOINT.join("artists.json")?;
        let req = self.get_request_with_account(url)?;
        let res = req.send()?;
        match res.status().as_u16() {
            200 => Ok(res.json()?),
            _ => Err(BooruError::APIError(res.text()?)),
        }
    }

    fn get_artist<Id: Into<ArtistId>>(&self, id: Id) -> Result<Artist, BooruError> {
        let id = id.into();
        let url = API_ENDPOINT.join(&format!("artists/{}.json", id))?;
        eprintln!("{:?}", url);
        let req = self.get_request_with_account(url)?;
        let res = req.send()?;
        match res.status().as_u16() {
            200 => Ok(res.json()?),
            _ => Err(BooruError::APIError(res.text()?)),
        }
    }

    fn create_artist(&self, body: ArtistCreateRequest) -> Result<Artist, BooruError> {
        let url = API_ENDPOINT.join("artists.json")?;
        if body.name.len() == 0 {
            return Err(BooruError::MissingRequiredFields("name".into()));
        }
        let req = self.post_request_with_account(url)?.json(&body);
        let res = req.send()?;
        match res.status().as_u16() {
            200 | 204 => Ok(res.json()?),
            _ => Err(BooruError::APIError(res.text()?)),
        }
    }
}
