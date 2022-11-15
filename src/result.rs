use crate::apns::reason::Reason;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Apns(#[from] Reason),

    #[error("interruption level does not match sound critical flag")]
    CriticalSound,

    #[error(transparent)]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    ReqwestMiddleware(#[from] reqwest_middleware::Error),

    #[error(transparent)]
    Url(#[from] url::ParseError),

    #[error("unknown")]
    Unknown,
}
