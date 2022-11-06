use time::OffsetDateTime;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The collapse identifier exceeds the maximum allowed size.")]
    ApnsBadCollapseId,

    #[error("The specified device token is invalid. Verify that the request contains a valid token and that the token matches the environment.")]
    ApnsBadDeviceToken,

    #[error("The apns-expiration value is invalid.")]
    ApnsBadExpirationDate,

    #[error("The apns-id value is invalid.")]
    ApnsBadMessageId,

    #[error("The apns-priority value is invalid.")]
    ApnsBadPriority,

    #[error("The apns-topic value is invalid.")]
    ApnsBadTopic,

    #[error("The device token doesn’t match the specified topic.")]
    ApnsDeviceTokenNotForTopic,

    #[error("One or more headers are repeated.")]
    ApnsDuplicateHeaders,

    #[error("Idle timeout.")]
    ApnsIdleTimeout,

    #[error("The apns-push-type value is invalid.")]
    ApnsInvalidPushType,

    #[error("The device token isn’t specified in the request :path. Verify that the :path header contains the device token.")]
    ApnsMissingDeviceToken,

    #[error("The apns-topic header of the request isn’t specified and is required. The apns-topic header is mandatory when the client is connected using a certificate that supports multiple topics.")]
    ApnsMissingTopic,

    #[error("The message payload is empty.")]
    ApnsPayloadEmpty,

    #[error("Pushing to this topic is not allowed.")]
    ApnsTopicDisallowed,

    #[error("The certificate is invalid.")]
    ApnsBadCertificate { timestamp: OffsetDateTime },

    #[error("The client certificate is for the wrong environment.")]
    ApnsBadCertificateEnvironment { timestamp: OffsetDateTime },

    #[error("The provider token is stale and a new token should be generated.")]
    ApnsExpiredProviderToken,

    #[error("The specified action is not allowed.")]
    ApnsForbidden,

    #[error("The provider token is not valid, or the token signature can't be verified.")]
    ApnsInvalidProviderToken,

    #[error("No provider certificate was used to connect to APNs, and the authorization header is missing or no provider token is specified.")]
    ApnsMissingProviderToken,

    #[error("The request contained an invalid :path value.")]
    ApnsBadPath,

    #[error("The specified :method value isn’t POST.")]
    ApnsMethodNotAllowed,

    #[error("The device token has expired.")]
    ApnsExpiredToken,

    #[error("The device token is inactive for the specified topic. There is no need to send further pushes to the same device token, unless your application retrieves the same device token, see Registering Your App with APNs")]
    ApnsUnregistered,

    #[error("The message payload is too large. For information about the allowed payload size, see Create and Send a POST Request to APNs.")]
    ApnsPayloadTooLarge,

    #[error("The provider’s authentication token is being updated too often. Update the authentication token no more than once every 20 minutes.")]
    ApnsTooManyProviderTokenUpdates,

    #[error("Too many requests were made consecutively to the same device token.")]
    ApnsTooManyRequests,

    #[error("An internal server error occurred.")]
    ApnsInternalServerError,

    #[error("The service is unavailable.")]
    ApnsServiceUnavailable,

    #[error("The APNs server is shutting down.")]
    ApnsShutdown,

    #[error("{0}")]
    ApnsOther(String),

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
