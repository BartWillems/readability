use displaydoc::Display;
use thiserror::Error;
use url;

#[derive(Debug, Error, Display)]
pub enum Error {
    /// URL Parser error: {0}
    UrlParseError(#[from] url::ParseError),
    /// Invalid Tendril buffer
    InvalidTendrilBuffer,
}
