use reqwest::{StatusCode, header::ToStrError};
use thiserror::Error;
pub type Result<T> = std::result::Result<T, Error>;

type RequestTokenError = oauth2::RequestTokenError<
    oauth2::HttpClientError<reqwest::Error>,
    oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Oauth2RequestToken(#[from] RequestTokenError),
    #[error("Keycloak responded with an error: {0}")]
    Oauth2UrlParse(#[from] oauth2::url::ParseError),
    #[error(transparent)]
    ToStrError(#[from] ToStrError),
    #[error("Expected Location Header to be reported")]
    NoLocationHeader,
    #[error("Malformed URL reported by Keycloak: {0}")]
    MalformedUrl(String),
    #[error("Keycloak responded status code {0}: {1}")]
    ResponseError(StatusCode, String),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
