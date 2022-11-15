use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, TimestampMilliSeconds};
use time::OffsetDateTime;

#[serde_as]
#[skip_serializing_none]
#[derive(thiserror::Error, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "reason")]
pub enum Reason {
    #[error("The collapse identifier exceeds the maximum allowed size.")]
    BadCollapseId,

    #[error("The specified device token is invalid. Verify that the request contains a valid token and that the token matches the environment.")]
    BadDeviceToken,

    #[error("The apns-expiration value is invalid.")]
    BadExpirationDate,

    #[error("The apns-id value is invalid.")]
    BadMessageId,

    #[error("The apns-priority value is invalid.")]
    BadPriority,

    #[error("The apns-topic value is invalid.")]
    BadTopic,

    #[error("The device token doesn’t match the specified topic.")]
    DeviceTokenNotForTopic,

    #[error("One or more headers are repeated.")]
    DuplicateHeaders,

    #[error("Idle timeout.")]
    IdleTimeout,

    #[error("The apns-push-type value is invalid.")]
    InvalidPushType,

    #[error("The device token isn’t specified in the request :path. Verify that the :path header contains the device token.")]
    MissingDeviceToken,

    #[error("The apns-topic header of the request isn’t specified and is required. The apns-topic header is mandatory when the client is connected using a certificate that supports multiple topics.")]
    MissingTopic,

    #[error("The message payload is empty.")]
    PayloadEmpty,

    #[error("Pushing to this topic is not allowed.")]
    TopicDisallowed,

    #[error("The certificate is invalid.")]
    BadCertificate {
        /// The time, in milliseconds since Epoch, at which APNs confirmed the token
        /// was no longer valid for the topic. This key is included only when the
        /// error in the `:status` field is 410.
        #[serde_as(as = "Option<TimestampMilliSeconds>")]
        timestamp: Option<OffsetDateTime>,
    },

    #[error("The client certificate is for the wrong environment.")]
    BadCertificateEnvironment {
        /// The time, in milliseconds since Epoch, at which APNs confirmed the token
        /// was no longer valid for the topic. This key is included only when the
        /// error in the `:status` field is 410.
        #[serde_as(as = "Option<TimestampMilliSeconds>")]
        timestamp: Option<OffsetDateTime>,
    },

    #[error("The provider token is stale and a new token should be generated.")]
    ExpiredProviderToken,

    #[error("The specified action is not allowed.")]
    Forbidden,

    #[error("The provider token is not valid, or the token signature can't be verified.")]
    InvalidProviderToken,

    #[error("No provider certificate was used to connect to APNs, and the authorization header is missing or no provider token is specified.")]
    MissingProviderToken,

    #[error("The request contained an invalid :path value.")]
    BadPath,

    #[error("The specified :method value isn’t POST.")]
    MethodNotAllowed,

    #[error("The device token has expired.")]
    ExpiredToken,

    #[error("The device token is inactive for the specified topic. There is no need to send further pushes to the same device token, unless your application retrieves the same device token, see Registering Your App with APNs")]
    Unregistered,

    #[error("The message payload is too large. For information about the allowed payload size, see Create and Send a POST Request to APNs.")]
    PayloadTooLarge,

    #[error("The provider’s authentication token is being updated too often. Update the authentication token no more than once every 20 minutes.")]
    TooManyProviderTokenUpdates,

    #[error("Too many requests were made consecutively to the same device token.")]
    TooManyRequests,

    #[error("An internal server error occurred.")]
    InternalServerError,

    #[error("The service is unavailable.")]
    ServiceUnavailable,

    #[error("The APNs server is shutting down.")]
    Shutdown,

    #[error("unknown")]
    #[serde(other)]
    Unknown,
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
            Reason::BadCertificate { .. } => StatusCode::FORBIDDEN,
            Reason::BadCertificateEnvironment { .. } => StatusCode::FORBIDDEN,
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
            Reason::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
