use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};
use time::OffsetDateTime;

use crate::result::Error;

#[derive(Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde_as]
#[skip_serializing_none]
pub struct ApnsResponse {
    /// The error code indicating the reason for the failure.
    pub reason: Option<Reason>,

    /// The time, in milliseconds since Epoch, at which APNs confirmed the token
    /// was no longer valid for the topic. This key is included only when the
    /// error in the `:status` field is 410.
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub timestamp: Option<OffsetDateTime>,
}

impl From<ApnsResponse> for Error {
    fn from(this: ApnsResponse) -> Self {
        if let Some(reason) = this.reason {
            match reason {
                Reason::BadCollapseId => Error::ApnsBadCollapseId,
                Reason::BadDeviceToken => Error::ApnsBadDeviceToken,
                Reason::BadExpirationDate => Error::ApnsBadExpirationDate,
                Reason::BadMessageId => Error::ApnsBadMessageId,
                Reason::BadPriority => Error::ApnsBadPriority,
                Reason::BadTopic => Error::ApnsBadTopic,
                Reason::DeviceTokenNotForTopic => Error::ApnsDeviceTokenNotForTopic,
                Reason::DuplicateHeaders => Error::ApnsDuplicateHeaders,
                Reason::IdleTimeout => Error::ApnsIdleTimeout,
                Reason::InvalidPushType => Error::ApnsInvalidPushType,
                Reason::MissingDeviceToken => Error::ApnsMissingDeviceToken,
                Reason::MissingTopic => Error::ApnsMissingTopic,
                Reason::PayloadEmpty => Error::ApnsPayloadEmpty,
                Reason::TopicDisallowed => Error::ApnsTopicDisallowed,
                Reason::BadCertificate => Error::ApnsBadCertificate {
                    timestamp: this.timestamp.unwrap_or(OffsetDateTime::UNIX_EPOCH),
                },
                Reason::BadCertificateEnvironment => Error::ApnsBadCertificateEnvironment {
                    timestamp: this.timestamp.unwrap_or(OffsetDateTime::UNIX_EPOCH),
                },
                Reason::ExpiredProviderToken => Error::ApnsExpiredProviderToken,
                Reason::Forbidden => Error::ApnsForbidden,
                Reason::InvalidProviderToken => Error::ApnsInvalidProviderToken,
                Reason::MissingProviderToken => Error::ApnsMissingProviderToken,
                Reason::BadPath => Error::ApnsBadPath,
                Reason::MethodNotAllowed => Error::ApnsMethodNotAllowed,
                Reason::ExpiredToken => Error::ApnsExpiredToken,
                Reason::Unregistered => Error::ApnsUnregistered,
                Reason::PayloadTooLarge => Error::ApnsPayloadTooLarge,
                Reason::TooManyProviderTokenUpdates => Error::ApnsTooManyProviderTokenUpdates,
                Reason::TooManyRequests => Error::ApnsTooManyRequests,
                Reason::InternalServerError => Error::ApnsInternalServerError,
                Reason::ServiceUnavailable => Error::ApnsServiceUnavailable,
                Reason::Shutdown => Error::ApnsShutdown,
                Reason::Other(msg) => Error::ApnsOther(msg),
            }
        } else {
            Error::Unknown
        }
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Reason {
    BadCollapseId,
    BadDeviceToken,
    BadExpirationDate,
    BadMessageId,
    BadPriority,
    BadTopic,
    DeviceTokenNotForTopic,
    DuplicateHeaders,
    IdleTimeout,
    InvalidPushType,
    MissingDeviceToken,
    MissingTopic,
    PayloadEmpty,
    TopicDisallowed,
    BadCertificate,
    BadCertificateEnvironment,
    ExpiredProviderToken,
    Forbidden,
    InvalidProviderToken,
    MissingProviderToken,
    BadPath,
    MethodNotAllowed,
    ExpiredToken,
    Unregistered,
    PayloadTooLarge,
    TooManyProviderTokenUpdates,
    TooManyRequests,
    InternalServerError,
    ServiceUnavailable,
    Shutdown,
    Other(String),
}

impl From<Reason> for StatusCode {
    fn from(this: Reason) -> Self {
        match this {
            Reason::BadCollapseId => StatusCode::BAD_REQUEST,
            Reason::BadDeviceToken => StatusCode::BAD_REQUEST,
            Reason::BadExpirationDate => StatusCode::BAD_REQUEST,
            Reason::BadMessageId => StatusCode::BAD_REQUEST,
            Reason::BadPriority => StatusCode::BAD_REQUEST,
            Reason::BadTopic => StatusCode::BAD_REQUEST,
            Reason::DeviceTokenNotForTopic => StatusCode::BAD_REQUEST,
            Reason::DuplicateHeaders => StatusCode::BAD_REQUEST,
            Reason::IdleTimeout => StatusCode::BAD_REQUEST,
            Reason::InvalidPushType => StatusCode::BAD_REQUEST,
            Reason::MissingDeviceToken => StatusCode::BAD_REQUEST,
            Reason::MissingTopic => StatusCode::BAD_REQUEST,
            Reason::PayloadEmpty => StatusCode::BAD_REQUEST,
            Reason::TopicDisallowed => StatusCode::BAD_REQUEST,
            Reason::BadCertificate => StatusCode::FORBIDDEN,
            Reason::BadCertificateEnvironment => StatusCode::FORBIDDEN,
            Reason::ExpiredProviderToken => StatusCode::FORBIDDEN,
            Reason::Forbidden => StatusCode::FORBIDDEN,
            Reason::InvalidProviderToken => StatusCode::FORBIDDEN,
            Reason::MissingProviderToken => StatusCode::FORBIDDEN,
            Reason::BadPath => StatusCode::NOT_FOUND,
            Reason::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            Reason::ExpiredToken => StatusCode::GONE,
            Reason::Unregistered => StatusCode::GONE,
            Reason::PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            Reason::TooManyProviderTokenUpdates => StatusCode::TOO_MANY_REQUESTS,
            Reason::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            Reason::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Reason::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            Reason::Shutdown => StatusCode::SERVICE_UNAVAILABLE,
            Reason::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
