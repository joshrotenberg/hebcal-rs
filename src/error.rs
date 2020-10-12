use serde::Deserialize;
use std::fmt;
use thiserror::Error;

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct HebCalError {
    pub error: String,
}

impl fmt::Display for HebCalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.error)
    }
}

impl std::error::Error for HebCalError {}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Argument error: {error:?}")]
    HebCal { error: HebCalError },
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("URL Parse error: {0}")]
    UrlParser(#[from] url::ParseError),
    #[error("Unknown error")]
    Unknown,
}
