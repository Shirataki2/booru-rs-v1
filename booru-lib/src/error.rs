use thiserror::Error;

#[derive(Debug, Error)]
pub enum BooruError {
    #[error("Unauthorized Error! Please check your username and API key!")]
    Unauthorized(reqwest::blocking::Response),
    #[error("Page Not Found")]
    PageNotFound(reqwest::blocking::Response),
    #[error("Unexpected API Error! {0:#?}")]
    APIError(String),

    #[error("Missing Required Fields: {0}")]
    MissingRequiredFields(String),
    #[error("Path is not a directory: {0}")]
    PathIsNotDirectory(String),

    #[error("Invalid Path")]
    PathError,
    #[error("IO Error: {0:#?}")]
    IoError(#[from] std::io::Error),
    #[error("Decode Error: {0:#?}")]
    DecodeError(#[from] toml::de::Error),
    #[error("Decode Error: {0:#?}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Image Error: {0:#?}")]
    ImageError(#[from] image::error::ImageError),
    #[error("URL Parse Error: {0:#?}")]
    UrlParseError(#[from] url::ParseError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
