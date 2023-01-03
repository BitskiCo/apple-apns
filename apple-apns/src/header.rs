use http::header::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_plain::{derive_display_from_serialize, derive_fromstr_from_deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Payload size limit for message types other than VoIP.
pub const PAYLOAD_SIZE_LIMIT: usize = 4096;

/// Payload size limit for the VoIP message type.
pub const VOIP_PAYLOAD_SIZE_LIMIT: usize = 5120;

/// (Required for watchOS 6 and later; recommended for macOS, iOS, tvOS, and
/// iPadOS) The value of this header must accurately reflect the contents of
/// your notification’s payload. If there’s a mismatch, or if the header is
/// missing on required systems, APNs may return an error, delay the delivery of
/// the notification, or drop it altogether.
pub static APNS_PUSH_TYPE: HeaderName = HeaderName::from_static("apns-push-type");

/// A canonical UUID that is the unique ID for the notification. If an error
/// occurs when sending the notification, APNs includes this value when
/// reporting the error to your server. Canonical UUIDs are 32 lowercase
/// hexadecimal digits, displayed in five groups separated by hyphens in the
/// form 8-4-4-4-12. For example: 123e4567-e89b-12d3-a456-4266554400a0. If you
/// omit this header, APNs creates a UUID for you and returns it in its
/// response.
pub static APNS_ID: HeaderName = HeaderName::from_static("apns-id");

/// The date at which the notification is no longer valid. This value is a UNIX
/// epoch expressed in seconds (UTC). If the value is nonzero, APNs stores the
/// notification and tries to deliver it at least once, repeating the attempt as
/// needed until the specified date. If the value is 0, APNs attempts to deliver
/// the notification only once and doesn’t store it.
///
/// A single APNs attempt may involve retries over multiple network interfaces
/// and connections of the destination device. Often these retries span over
/// some time period, depending on the network characteristics. In addition, a
/// push notification may take some time on the network after APNs sends it to
/// the device. APNs uses best efforts to honor the expiry date without any
/// guarantee. If the value is nonzero, the notification may be delivered after
/// the mentioned date. If the value is 0, the notification may be delivered
/// with some delay.
pub static APNS_EXPIRATION: HeaderName = HeaderName::from_static("apns-expiration");

/// The priority of the notification. If you omit this header, APNs sets the
/// notification priority to 10.
///
/// Specify 10 to send the notification immediately.
///
/// Specify 5 to send the notification based on power considerations on the
/// user’s device.
///
/// Specify 1 to prioritize the device’s power considerations over all other
/// factors for delivery, and prevent awakening the device.
pub static APNS_PRIORITY: HeaderName = HeaderName::from_static("apns-priority");

/// The topic for the notification. In general, the topic is your app’s bundle
/// ID/app ID. It can have a suffix based on the type of push notification. If
/// you’re using a certificate that supports PushKit VoIP or watchOS
/// complication notifications, you must include this header with bundle ID of
/// you app and if applicable, the proper suffix. If you’re using token-based
/// authentication with APNs, you must include this header with the correct
/// bundle ID and suffix combination. To learn more about app ID, see [Register
/// an App ID](https://help.apple.com/developer-account/#/dev1b35d6f83).
pub static APNS_TOPIC: HeaderName = HeaderName::from_static("apns-topic");

/// An identifier you use to coalesce multiple notifications into a single
/// notification for the user. Typically, each notification request causes a new
/// notification to be displayed on the user’s device. When sending the same
/// notification more than once, use the same value in this header to coalesce
/// the requests. The value of this key must not exceed 64 bytes.
pub static APNS_COLLAPSE_ID: HeaderName = HeaderName::from_static("apns-collapse-id");

/// Use the `alert` push type for notifications that trigger a user
/// interaction—for example, an alert, badge, or sound. If you set this push
/// type, the `apns-topic` header field must use your app’s bundle ID as the
/// topic. For more information, see [Generating a remote
/// notification](https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/generating_a_remote_notification).
///
/// If the notification requires immediate action from the user, set
/// notification priority to 10; otherwise use 5.
///
/// The `alert` push type is required on watchOS 6 and later. It is recommended
/// on macOS, iOS, tvOS, and iPadOS.
pub static ALERT: HeaderValue = HeaderValue::from_static("alert");

/// Use the `background` push type for notifications that deliver content in the
/// background, and don’t trigger any user interactions. If you set this push
/// type, the `apns-topic header` field must use your app’s bundle ID as the
/// topic. Always use priority 5. Using priority 10 is an error. For more
/// information, see [Pushing Background Updates to Your
/// App](https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/pushing_background_updates_to_your_app).
///
/// The `background` push type is required on watchOS 6 and later. It is
/// recommended on macOS, iOS, tvOS, and iPadOS.
pub static BACKGROUND: HeaderValue = HeaderValue::from_static("background");

/// Use the `location` push type for notifications that request a user’s
/// location. If you set this push type, the `apns-topic` header field must use
/// your app’s bundle ID with `.location-query` appended to the end. For more
/// information, see Creating a location push service extension.
///
/// The `location` push type is recommended for iOS and iPadOS. It isn’t
/// available on macOS, tvOS, and watchOS.
///
/// If the location query requires an immediate response from the Location Push
/// Service Extension, set notification `apns-priority` to 10; otherwise, use 5.
///
/// The `location` push type supports only token-based authentication.
pub static LOCATION: HeaderValue = HeaderValue::from_static("location");

/// Use the `voip` push type for notifications that provide information about an
/// incoming Voice-over-IP (VoIP) call. For more information, see [Responding to
/// VoIP Notifications from
/// PushKit](https://developer.apple.com/documentation/pushkit/responding_to_voip_notifications_from_pushkit).
///
/// If you set this push type, the apns-topic header field must use your app’s
/// bundle ID with `.voip` appended to the end. If you’re using
/// certificate-based authentication, you must also register the certificate for
/// VoIP services. The topic is then part of the 1.2.840.113635.100.6.3.4 or
/// 1.2.840.113635.100.6.3.6 extension.
///
/// The `voip` push type is not available on watchOS. It is recommended on
/// macOS, iOS, tvOS, and iPadOS.
pub static VOIP: HeaderValue = HeaderValue::from_static("voip");

/// Use the `complication` push type for notifications that contain update
/// information for a watchOS app’s complications. For more information, see
/// Keeping Your Complications Up to Date.
///
/// If you set this push type, the `apns-topic` header field must use your app’s
/// bundle ID with `.complication` appended to the end. If you’re using
/// certificate-based authentication, you must also register the certificate for
/// WatchKit services. The topic is then part of the 1.2.840.113635.100.6.3.6
/// extension.
///
/// The `complication` push type is recommended for watchOS and iOS. It is not
/// available on macOS, tvOS, and iPadOS.
pub static COMPLICATION: HeaderValue = HeaderValue::from_static("complication");

/// Use the `fileprovider` push type to signal changes to a File Provider
/// extension. If you set this push type, the `apns-topic` header field must use
/// your app’s bundle ID with `.pushkit.fileprovider` appended to the end. For
/// more information, see Using push notifications to signal changes.
///
/// The `fileprovider` push type is not available on watchOS. It is recommended
/// on macOS, iOS, tvOS, and iPadOS.
pub static FILEPROVIDER: HeaderValue = HeaderValue::from_static("fileprovider");

/// Use the `mdm` push type for notifications that tell managed devices to
/// contact the MDM server. If you set this push type, you must use the topic
/// from the UID attribute in the subject of your MDM push certificate. For more
/// information, see [Device
/// Management](https://developer.apple.com/documentation/devicemanagement).
///
/// The mdm push type is not available on watchOS. It is recommended on macOS,
/// iOS, tvOS, and iPadOS.
pub static MDM: HeaderValue = HeaderValue::from_static("mdm");

/// Send the notification immediately.
pub static PRIORITY_IMMEDIATE: HeaderValue = HeaderValue::from_static("10");

/// Send the notification based on power considerations on the user’s device
pub static PRIORITY_CONSIDER_POWER: HeaderValue = HeaderValue::from_static("5");

/// Prioritize the device’s power considerations over all other factors for
/// delivery, and prevent awakening the device.
pub static PRIORITY_PRIORITIZE_POWER: HeaderValue = HeaderValue::from_static("1");

/// The `apns-push-type` header field has the following valid values. The
/// descriptions below describe when and how to use these values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PushType {
    /// Use the `alert` push type for notifications that trigger a user
    /// interaction—for example, an alert, badge, or sound. If you set this push
    /// type, the `apns-topic` header field must use your app’s bundle ID as the
    /// topic. For more information, see [Generating a remote
    /// notification](https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/generating_a_remote_notification).
    ///
    /// If the notification requires immediate action from the user, set
    /// notification priority to 10; otherwise use 5.
    ///
    /// The `alert` push type is required on watchOS 6 and later. It is
    /// recommended on macOS, iOS, tvOS, and iPadOS.
    Alert,

    /// Use the `background` push type for notifications that deliver content in
    /// the background, and don’t trigger any user interactions. If you set this
    /// push type, the `apns-topic header` field must use your app’s bundle ID
    /// as the topic. Always use priority 5. Using priority 10 is an error. For
    /// more information, see [Pushing Background Updates to Your
    /// App](https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/pushing_background_updates_to_your_app).
    ///
    /// The `background` push type is required on watchOS 6 and later. It is
    /// recommended on macOS, iOS, tvOS, and iPadOS.
    Background,

    /// Use the `location` push type for notifications that request a user’s
    /// location. If you set this push type, the `apns-topic` header field must
    /// use your app’s bundle ID with `.location-query` appended to the end. For
    /// more information, see Creating a location push service extension.
    ///
    /// The `location` push type is recommended for iOS and iPadOS. It isn’t
    /// available on macOS, tvOS, and watchOS.
    ///
    /// If the location query requires an immediate response from the Location
    /// Push Service Extension, set notification `apns-priority` to 10;
    /// otherwise, use 5.
    ///
    /// The location push type supports only token-based authentication.
    Location,

    /// Use the `voip` push type for notifications that provide information
    /// about an incoming Voice-over-IP (VoIP) call. For more information, see
    /// [Responding to VoIP Notifications from
    /// PushKit](https://developer.apple.com/documentation/pushkit/responding_to_voip_notifications_from_pushkit).
    ///
    /// If you set this push type, the apns-topic header field must use your
    /// app’s bundle ID with `.voip` appended to the end. If you’re using
    /// certificate-based authentication, you must also register the certificate
    /// for VoIP services. The topic is then part of the
    /// 1.2.840.113635.100.6.3.4 or 1.2.840.113635.100.6.3.6 extension.
    ///
    /// The voip push type is not available on watchOS. It is recommended on
    /// macOS, iOS, tvOS, and iPadOS.
    Voip,

    /// Use the `complication` push type for notifications that contain update
    /// information for a watchOS app’s complications. For more information, see
    /// Keeping Your Complications Up to Date.
    ///
    /// If you set this push type, the `apns-topic` header field must use your
    /// app’s bundle ID with `.complication` appended to the end. If you’re
    /// using certificate-based authentication, you must also register the
    /// certificate for WatchKit services. The topic is then part of the
    /// 1.2.840.113635.100.6.3.6 extension.
    ///
    /// The `complication` push type is recommended for watchOS and iOS. It is
    /// not available on macOS, tvOS, and iPadOS.
    Complication,

    /// Use the `fileprovider` push type to signal changes to a File Provider
    /// extension. If you set this push type, the `apns-topic` header field must
    /// use your app’s bundle ID with `.pushkit.fileprovider` appended to the
    /// end. For more information, see Using push notifications to signal
    /// changes.
    ///
    /// The `fileprovider` push type is not available on watchOS. It is
    /// recommended on macOS, iOS, tvOS, and iPadOS.
    Fileprovider,

    /// Use the `mdm` push type for notifications that tell managed devices to
    /// contact the MDM server. If you set this push type, you must use the
    /// topic from the UID attribute in the subject of your MDM push
    /// certificate. For more information, see [Device
    /// Management](https://developer.apple.com/documentation/devicemanagement).
    ///
    /// The mdm push type is not available on watchOS. It is recommended on
    /// macOS, iOS, tvOS, and iPadOS.
    Mdm,
}

impl Default for PushType {
    fn default() -> Self {
        Self::Alert
    }
}

impl From<PushType> for HeaderValue {
    fn from(apns_push_type: PushType) -> Self {
        match apns_push_type {
            PushType::Alert => ALERT.clone(),
            PushType::Background => BACKGROUND.clone(),
            PushType::Location => LOCATION.clone(),
            PushType::Voip => VOIP.clone(),
            PushType::Complication => COMPLICATION.clone(),
            PushType::Fileprovider => FILEPROVIDER.clone(),
            PushType::Mdm => MDM.clone(),
        }
    }
}

derive_fromstr_from_deserialize!(PushType);
derive_display_from_serialize!(PushType);

impl PushType {
    pub fn payload_size_limit(&self) -> usize {
        if *self == PushType::Voip {
            VOIP_PAYLOAD_SIZE_LIMIT
        } else {
            PAYLOAD_SIZE_LIMIT
        }
    }
}

/// Push notification priority.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum Priority {
    /// Send the notification immediately.
    Immediate = 10,

    /// Send the notification based on power considerations on the user’s device
    ConsiderPower = 5,

    /// Prioritize the device’s power considerations over all other factors for
    /// delivery, and prevent awakening the device.
    PrioritizePower = 1,
}

impl Default for Priority {
    fn default() -> Self {
        Self::Immediate
    }
}

impl From<Priority> for HeaderValue {
    fn from(this: Priority) -> Self {
        match this {
            Priority::Immediate => PRIORITY_IMMEDIATE.clone(),
            Priority::ConsiderPower => PRIORITY_CONSIDER_POWER.clone(),
            Priority::PrioritizePower => PRIORITY_PRIORITIZE_POWER.clone(),
        }
    }
}

derive_fromstr_from_deserialize!(Priority);
derive_display_from_serialize!(Priority);
