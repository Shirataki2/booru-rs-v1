use reqwest::blocking::{Client, RequestBuilder};
use url::Url;
use once_cell::sync::Lazy;
use crate::config::Config;
use crate::error::BooruError;
use crate::models::UserProfile;

pub static API_ENDPOINT: Lazy<Url> = Lazy::new(|| {
    Url::parse("https://danbooru.donmai.us/").unwrap()
});

#[derive(Debug, Clone)]
pub struct BooruClient {
    client: Client,
    config: Option<Config>,
}

impl BooruClient {
    fn build_client() -> Result<Client, BooruError> {
        let client = Client::builder()
            .build()?;
        Ok(client)
    }

    pub(crate) fn get_request_with_account(&self, url: Url) -> Result<RequestBuilder, BooruError> {
        let req = if let Some(Config { account: Some(account) }) = &self.config {
            self.client.get(url)
                .query(&[
                    ("login", &account.username),
                    ("api_key", &account.api_key),
                ])
        } else {
            self.client.get(url)
        };
        Ok(req)
    }

    pub(crate) fn post_request_with_account(&self, url: Url) -> Result<RequestBuilder, BooruError> {
        let req = if let Some(Config { account: Some(account) }) = &self.config {
            self.client.post(url)
                .query(&[
                    ("login", &account.username),
                    ("api_key", &account.api_key),
                ])
        } else {
            self.client.post(url)
        };
        Ok(req)
    }

    pub fn from_config(config: &Config) -> Result<Self, BooruError> {
        Ok(BooruClient {
            client: Self::build_client()?,
            config: Some(config.clone()),
        })
    }

    pub fn profile(&self) -> Result<UserProfile, BooruError> {
        let url = API_ENDPOINT.join("profile.json")?;
        let req = self.get_request_with_account(url)?;
        let resp = req.send()?;

        match resp.status().as_u16() {
            401 => Err(BooruError::Unauthorized(resp)),
            200 => Ok(resp.json()?),
            _ => unreachable!(),
        }
    }
}

