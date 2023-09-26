use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    NoDistDir(PathBuf),
    Io(std::io::Error),
    Decode(serde_json::Error),
    Request(reqwest::Error),
    UrlParse(url::ParseError),
    StripPrefix(std::path::StripPrefixError),
    PackageParseError(&'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            NoDistDir(path) => write!(f, "Error: The directory {:?} does not exists", path),
            PackageParseError(err) => write!(f, "Package parse error: {}", err),
            StripPrefix(err) => write!(f, "Unable to normalize path: {}", err),
            Io(err) => write!(f, "IO error: {}", err),
            Decode(err) => write!(f, "JSON decode error: {}", err),
            Request(err) => write!(f, "Request error: {}", err),
            UrlParse(err) => write!(f, "URL parse error: {}", err),
        }
    }
}

impl From<std::path::StripPrefixError> for Error {
    fn from(err: std::path::StripPrefixError) -> Self {
        Error::StripPrefix(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::UrlParse(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Decode(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Request(err)
    }
}

impl std::error::Error for Error {}

pub type BundlerResult<T> = Result<T, Error>;
