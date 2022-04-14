#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    LetsEncryptError(String),
    KeygenError(openssl::error::ErrorStack),
    SerdeError(serde_json::Error),
    StringFromUtf8Error(std::string::FromUtf8Error),
    GeneralError(String),
    HeaderToStrError(reqwest::header::ToStrError),
    ApiProblem(crate::api::ApiProblem),
    ChronoParseError(chrono::ParseError),
    StdIoError(std::io::Error),

    NoChallenge,
    NoContentType,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Self::ReqwestError(e)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Error {
        Self::LetsEncryptError(e)
    }
}

impl From<openssl::error::ErrorStack> for Error {
    fn from(e: openssl::error::ErrorStack) -> Error {
        Self::KeygenError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Self::SerdeError(e)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Error {
        Self::StringFromUtf8Error(e)
    }
}

impl From<reqwest::header::ToStrError> for Error {
    fn from(e: reqwest::header::ToStrError) -> Error {
        Self::HeaderToStrError(e)
    }
}

impl From<crate::api::ApiProblem> for Error {
    fn from(e: crate::api::ApiProblem) -> Error {
        Self::ApiProblem(e)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(e: chrono::ParseError) -> Error {
        Self::ChronoParseError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Self::StdIoError(e)
    }
}
