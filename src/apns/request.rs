use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

/// Put the JSON payload with the notification’s content into the body of your
/// request. The JSON payload must not be compressed and is limited to a maximum
/// size of 4 KB (4096 bytes). For a Voice over Internet Protocol (VoIP)
/// notification, the maximum size is 5 KB (5120 bytes).
#[derive(Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde_as]
#[skip_serializing_none]
pub struct ApnsPayload<T>
where
    T: Serialize,
{
    /// The information for displaying an alert.
    pub alert: Option<ApnsAlert>,

    /// The number to display in a badge on your app’s icon. Specify `0` to
    /// remove the current badge, if any.
    pub badge: Option<u32>,

    /// The name of a sound file in your app’s main bundle or in the
    /// `Library/Sounds` folder of your app’s container directory or a
    /// dictionary that contains sound information for critical alerts.
    pub sound: Option<ApnsSound>,

    /// An app-specific identifier for grouping related notifications. This
    /// value corresponds to the
    /// [`threadIdentifier`](https://developer.apple.com/documentation/usernotifications/unmutablenotificationcontent/1649872-threadidentifier)
    /// property in the `UNNotificationContent` object.
    pub thread_id: Option<String>,

    /// The notification’s type. This string must correspond to the
    /// [`identifier`](https://developer.apple.com/documentation/usernotifications/unnotificationcategory/1649276-identifier)
    /// of one of the `UNNotificationCategory` objects you register at launch
    /// time. See [Declaring Your Actionable Notification
    /// Types](https://developer.apple.com/documentation/usernotifications/declaring_your_actionable_notification_types).
    pub category: Option<String>,

    /// The background notification flag. To perform a silent background update,
    /// specify the value `1` and don’t include the `alert`, `badge`, or `sound`
    /// keys in your payload. See [Pushing Background Updates to Your
    /// App](https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/pushing_background_updates_to_your_app).
    #[serde_as(as = "BoolFromInt")]
    pub content_available: Option<bool>,

    /// The notification service app extension flag. If the value is `1`, the
    /// system passes the notification to your notification service app
    /// extension before delivery. Use your extension to modify the
    /// notification’s content. See [Modifying Content in Newly Delivered
    /// Notifications](https://developer.apple.com/documentation/usernotifications/modifying_content_in_newly_delivered_notifications).
    #[serde_as(as = "BoolFromInt")]
    pub mutable_content: Option<bool>,

    /// The identifier of the window brought forward. The value of this key will
    /// be populated on the
    /// [`UNNotificationContent`](https://developer.apple.com/documentation/usernotifications/unnotificationcontent)
    /// object created from the push payload. Access the value using the
    /// [`UNNotificationContent`](https://developer.apple.com/documentation/usernotifications/unnotificationcontent)
    /// object’s
    /// [`targetContentIdentifier`](https://developer.apple.com/documentation/usernotifications/unnotificationcontent/3235764-targetcontentidentifier)
    /// property.
    pub target_content_id: Option<String>,

    /// The importance and delivery timing of a notification. The string values
    /// `passive`, `active`, `time-sensitive`, or `critical` correspond to the
    /// [`UNNotificationInterruptionLevel`](https://developer.apple.com/documentation/usernotifications/unnotificationinterruptionlevel)
    /// enumeration cases.
    pub interruption_level: Option<InterruptionLevel>,

    /// The relevance score, a number between `0` and `1`, that the system uses
    /// to sort the notifications from your app. The highest score gets featured
    /// in the notification summary. See
    /// [`relevanceScore`](https://developer.apple.com/documentation/usernotifications/unnotificationcontent/3821031-relevancescore).
    pub relevance_score: Option<f64>,

    /// Additional data to send.
    #[serde(flatten)]
    pub user_info: Option<T>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ApnsAlert {
    Body(String),
    Alert(Alert),
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[skip_serializing_none]
pub struct Alert {
    /// The title of the notification. Apple Watch displays this string in
    /// the short look notification interface. Specify a string that’s
    /// quickly understood by the user.
    pub title: Option<String>,

    /// Additional information that explains the purpose of the
    /// notification.
    pub subtitle: Option<String>,

    /// The content of the alert message.
    pub body: Option<String>,

    /// The name of the launch image file to display. If the user chooses to
    /// launch your app, the contents of the specified image or storyboard
    /// file are displayed instead of your app’s normal launch image.
    pub launch_image: Option<String>,

    /// The key for a localized `title` string. Specify this key instead of
    /// the title key to retrieve the title from your app’s
    /// `Localizable.strings` files. The value must contain the name of a
    /// key in your strings file.
    pub title_loc_key: Option<String>,

    /// An array of strings containing replacement values for variables in
    /// your title string. Each `%@` character in the string specified by
    /// the `title-loc-key` is replaced by a value from this array. The
    /// first item in the array replaces the first instance of the `%@`
    /// character in the string, the second item replaces the second
    /// instance, and so on.
    pub title_loc_args: Option<Vec<String>>,

    /// The key for a localized `subtitle` string. Use this key, instead of
    /// the subtitle key, to retrieve the subtitle from your app’s
    /// `Localizable.strings` file. The value must contain the name of a key
    /// in your strings file.
    pub subtitle_loc_key: Option<String>,

    /// An array of strings containing replacement values for variables in
    /// your title string. Each `%@` character in the string specified by
    /// `subtitle-loc-key` is replaced by a value from this array. The first
    /// item in the array replaces the first instance of the `%@` character in
    /// the string, the second item replaces the second instance, and so on.
    pub subtitle_loc_args: Option<Vec<String>>,

    /// The key for a localized message string. Use this key, instead of the
    /// body key, to retrieve the message text from your app’s
    /// `Localizable.strings` file. The value must contain the name of a key
    /// in your strings file.
    pub loc_key: Option<String>,

    /// An array of strings containing replacement values for variables in
    /// your message text. Each `%@` character in the string specified by
    /// `loc-key` is replaced by a value from this array. The first item in
    /// the array replaces the first instance of the `%@` character in the
    /// string, the second item replaces the second instance, and so on.
    pub loc_args: Option<Vec<String>>,
}

impl From<Alert> for ApnsAlert {
    fn from(this: Alert) -> Self {
        if this.title.is_none()
            && this.subtitle.is_none()
            && this.launch_image.is_none()
            && this.title_loc_key.is_none()
            && this.title_loc_args.is_none()
            && this.subtitle_loc_key.is_none()
            && this.subtitle_loc_args.is_none()
            && this.loc_key.is_none()
            && this.loc_args.is_none()
        {
            if let Some(body) = this.body {
                return ApnsAlert::Body(body);
            }
        }
        ApnsAlert::Alert(this)
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ApnsSound {
    /// The name of a sound file in your app’s main bundle or in the
    /// `Library/Sounds` folder of your app’s container directory. Specify the
    /// string `default` to play the system sound. Use this key for regular
    /// notifications. For critical alerts, use the sound dictionary instead.
    /// For information about how to prepare sounds, see
    /// [`UNNotificationSound`](https://developer.apple.com/documentation/usernotifications/unnotificationsound).
    Name(String),

    Sound(Sound),
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde_as]
#[skip_serializing_none]
pub struct Sound {
    /// The critical alert flag. Set to `1` to enable the critical alert.
    #[serde_as(as = "BoolFromInt")]
    critical: Option<bool>,

    /// The name of a sound file in your app’s main bundle or in the
    /// `Library/Sounds` folder of your app’s container directory. Specify
    /// the string `default` to play the system sound. For information about
    /// how to prepare sounds, see
    /// [`UNNotificationSound`](https://developer.apple.com/documentation/usernotifications/unnotificationsound).
    name: Option<String>,

    /// The volume for the critical alert’s sound. Set this to a value
    /// between `0` (silent) and `1` (full volume).
    volume: Option<f64>,
}

impl From<Sound> for ApnsSound {
    fn from(this: Sound) -> Self {
        if this.critical.is_none() && this.volume.is_none() {
            if let Some(name) = this.name {
                return ApnsSound::Name(name);
            }
        }
        ApnsSound::Sound(this)
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InterruptionLevel {
    /// The system presents the notification immediately, lights up the screen,
    /// and can play a sound.
    Active,

    /// The system presents the notification immediately, lights up the screen,
    /// and bypasses the mute switch to play a sound.
    Critical,

    /// The system adds the notification to the notification list without
    /// lighting up the screen or playing a sound.
    Passive,

    /// The system presents the notification immediately, lights up the screen,
    /// and can play a sound, but won’t break through system notification
    /// controls.
    TimeSensitive,
}
