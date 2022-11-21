use crate::reason::Reason;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Apns(#[from] Reason),

    #[error("interruption level does not match sound critical flag")]
    CriticalSound,

    #[error(transparent)]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),

    #[cfg(feature = "jwt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "jwt")))]
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("payload too large: {size} exceeds {limit}")]
    PayloadTooLarge { size: usize, limit: usize },

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    ReqwestMiddleware(#[from] reqwest_middleware::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[cfg(feature = "jwt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "jwt")))]
    #[error(transparent)]
    SystemTime(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    Url(#[from] url::ParseError),

    #[error("unknown")]
    Unknown,
}
