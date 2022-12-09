use serde::{
    de::{self, MapAccess, Visitor},
    ser::{SerializeMap, SerializeStruct},
    Deserialize, Serialize,
};
use serde_plain::{derive_display_from_serialize, derive_fromstr_from_deserialize};
use serde_with::{serde_as, skip_serializing_none, BoolFromInt};

fn is_false(v: &bool) -> bool {
    !v
}

/// Put the JSON payload with the notification’s content into the body of your
/// request. The JSON payload must not be compressed and is limited to a maximum
/// size of 4 KB (4096 bytes). For a Voice over Internet Protocol (VoIP)
/// notification, the maximum size is 5 KB (5120 bytes).
#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Payload<T = ()>
where
    T: Serialize,
{
    /// The information for displaying an alert.
    pub alert: Option<Alert>,

    /// The number to display in a badge on your app’s icon. Specify `0` to
    /// remove the current badge, if any.
    pub badge: Option<u32>,

    /// The name of a sound file in your app’s main bundle or in the
    /// `Library/Sounds` folder of your app’s container directory or a
    /// dictionary that contains sound information for critical alerts.
    pub sound: Option<Sound>,

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
    #[serde(default, skip_serializing_if = "is_false")]
    #[serde_as(as = "BoolFromInt")]
    pub content_available: bool,

    /// The notification service app extension flag. If the value is `1`, the
    /// system passes the notification to your notification service app
    /// extension before delivery. Use your extension to modify the
    /// notification’s content. See [Modifying Content in Newly Delivered
    /// Notifications](https://developer.apple.com/documentation/usernotifications/modifying_content_in_newly_delivered_notifications).
    #[serde(default, skip_serializing_if = "is_false")]
    #[serde_as(as = "BoolFromInt")]
    pub mutable_content: bool,

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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Alert {
    /// The title of the notification. Apple Watch displays this string in
    /// the short look notification interface. Specify a string that’s
    /// quickly understood by the user.
    pub title: Option<String>,

    /// Additional information that explains the purpose of the
    /// notification.
    pub subtitle: Option<String>,

    /// The content of the alert message.
    pub body: String,

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

impl<'de> Deserialize<'de> for Alert {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AlertVisitor;

        impl<'de> Visitor<'de> for AlertVisitor {
            type Value = Alert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an alert struct")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Alert {
                    body: v.into(),
                    ..Default::default()
                })
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Alert {
                    body: v,
                    ..Default::default()
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut alert = Alert::default();
                let mut match_body = false;

                while let Some(key) = map.next_key::<&str>()? {
                    match key {
                        "title" => alert.title = map.next_value()?,
                        "subtitle" => alert.subtitle = map.next_value()?,
                        "body" => {
                            alert.body = map.next_value()?;
                            match_body = true;
                        }
                        "title-loc-key" => alert.title_loc_key = map.next_value()?,
                        "title-loc-args" => alert.title_loc_args = map.next_value()?,
                        "subtitle-loc-key" => alert.subtitle_loc_key = map.next_value()?,
                        "subtitle-loc-args" => alert.subtitle_loc_args = map.next_value()?,
                        "loc-key" => alert.loc_key = map.next_value()?,
                        "loc-args" => alert.loc_args = map.next_value()?,
                        "launch-image" => alert.launch_image = map.next_value()?,
                        field => {
                            return Err(de::Error::unknown_field(
                                field,
                                &[
                                    "title",
                                    "subtitle",
                                    "body",
                                    "title-loc-key",
                                    "title-loc-args",
                                    "subtitle-loc-key",
                                    "subtitle-loc-args",
                                    "loc-key",
                                    "loc-args",
                                    "launch-image",
                                ],
                            ));
                        }
                    }
                }

                if !match_body {
                    return Err(de::Error::missing_field("body"));
                }

                Ok(alert)
            }
        }

        deserializer.deserialize_any(AlertVisitor)
    }
}

impl Serialize for Alert {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.title.is_none()
            && self.subtitle.is_none()
            && self.launch_image.is_none()
            && self.title_loc_key.is_none()
            && self.title_loc_args.is_none()
            && self.subtitle_loc_key.is_none()
            && self.subtitle_loc_args.is_none()
            && self.loc_key.is_none()
            && self.loc_args.is_none()
        {
            serializer.serialize_str(&self.body)
        } else {
            let mut len = 1;
            if self.title.is_some() {
                len += 1;
            }
            if self.subtitle.is_some() {
                len += 1;
            }
            if self.title_loc_key.is_some() {
                len += 1;
            }
            if self.title_loc_args.is_some() {
                len += 1;
            }
            if self.subtitle_loc_key.is_some() {
                len += 1;
            }
            if self.subtitle_loc_args.is_some() {
                len += 1;
            }
            if self.loc_key.is_some() {
                len += 1;
            }
            if self.loc_args.is_some() {
                len += 1;
            }

            let mut alert = serializer.serialize_map(Some(len))?;

            if let Some(title_loc_key) = &self.title_loc_key {
                alert.serialize_entry("title-loc-key", title_loc_key)?;
                if let Some(title_loc_args) = &self.title_loc_args {
                    alert.serialize_entry("title-loc-args", title_loc_args)?;
                }
            } else if let Some(title) = &self.title {
                alert.serialize_entry("title", title)?;
            }

            if let Some(subtitle_loc_key) = &self.subtitle_loc_key {
                alert.serialize_entry("subtitle-loc-key", subtitle_loc_key)?;
                if let Some(subtitle_loc_args) = &self.subtitle_loc_args {
                    alert.serialize_entry("subtitle-loc-args", subtitle_loc_args)?;
                }
            } else if let Some(subtitle) = &self.subtitle {
                alert.serialize_entry("subtitle", subtitle)?;
            }

            if let Some(loc_key) = &self.loc_key {
                alert.serialize_entry("loc-key", loc_key)?;
                if let Some(loc_args) = &self.loc_args {
                    alert.serialize_entry("loc-args", loc_args)?;
                }
            } else {
                alert.serialize_entry("body", &self.body)?;
            }

            if let Some(launch_image) = &self.launch_image {
                alert.serialize_entry("launch-image", launch_image)?;
            }

            alert.end()
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sound {
    /// The critical alert flag. Set to `1` to enable the critical alert.
    pub critical: bool,

    /// The name of a sound file in your app’s main bundle or in the
    /// `Library/Sounds` folder of your app’s container directory. Specify
    /// the string `default` to play the system sound. For information about
    /// how to prepare sounds, see
    /// [`UNNotificationSound`](https://developer.apple.com/documentation/usernotifications/unnotificationsound).
    pub name: String,

    /// The volume for the critical alert’s sound. Set this to a value
    /// between `0` (silent) and `1` (full volume).
    pub volume: f64,
}

impl Default for Sound {
    fn default() -> Self {
        Self {
            critical: false,
            name: "default".into(),
            volume: 1.,
        }
    }
}

impl<'de> Deserialize<'de> for Sound {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SoundVisitor;

        impl<'de> Visitor<'de> for SoundVisitor {
            type Value = Sound;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a sound struct")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Sound {
                    critical: false,
                    name: v.into(),
                    volume: 0.,
                })
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Sound {
                    critical: false,
                    name: v,
                    volume: 0.,
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut sound = Sound::default();
                let mut match_critical = false;
                let mut match_name = false;
                let mut match_volume = false;

                while let Some(key) = map.next_key::<&str>()? {
                    match key {
                        "critical" => {
                            let critical: i64 = map.next_value()?;
                            sound.critical = critical != 0;
                            match_critical = true;
                        }
                        "name" => {
                            sound.name = map.next_value()?;
                            match_name = true;
                        }
                        "volume" => {
                            sound.volume = map.next_value()?;
                            match_volume = true;
                        }
                        field => {
                            return Err(de::Error::unknown_field(
                                field,
                                &["critical", "name", "volume"],
                            ));
                        }
                    }
                }

                if !match_critical {
                    return Err(de::Error::missing_field("critical"));
                }
                if !match_name {
                    return Err(de::Error::missing_field("name"));
                }
                if !match_volume {
                    return Err(de::Error::missing_field("volume"));
                }

                Ok(sound)
            }
        }

        deserializer.deserialize_any(SoundVisitor)
    }
}

impl Serialize for Sound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.critical {
            let mut sound = serializer.serialize_struct("Sound", 3)?;
            sound.serialize_field("critical", &1)?;
            sound.serialize_field("name", &self.name)?;
            sound.serialize_field("volume", &self.volume.clamp(0., 1.))?;
            sound.end()
        } else {
            self.name.serialize(serializer)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
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

derive_fromstr_from_deserialize!(InterruptionLevel);
derive_display_from_serialize!(InterruptionLevel);

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use serde_json::json;

    use super::*;

    #[derive(Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
    struct TestUserInfo {
        foo: bool,
        bar: i64,
    }

    #[test]
    fn payload_de() {
        assert_eq!(
            serde_json::from_str::<Payload>(
                &json!({
                    "alert": "Hello World!",
                    "badge": 11,
                    "sound": "default",
                    "thread-id": "my-thread-id",
                    "category": "my-category",
                    "content-available": 1,
                    "mutable-content": 1,
                    "target-content-id": "my-target-id",
                    "interruption-level": "active",
                    "relevance-score": 0.5,
                })
                .to_string()
            )
            .unwrap(),
            Payload {
                alert: Some(Alert {
                    body: "Hello World!".into(),
                    ..Default::default()
                }),
                badge: Some(11),
                sound: Some(Sound {
                    critical: false,
                    name: "default".into(),
                    volume: 0.
                }),
                thread_id: Some("my-thread-id".into()),
                category: Some("my-category".into()),
                content_available: true,
                mutable_content: true,
                target_content_id: Some("my-target-id".into()),
                interruption_level: Some(InterruptionLevel::Active),
                relevance_score: Some(0.5),
                user_info: Some(())
            }
        );
        assert_eq!(
            serde_json::from_str::<Payload<TestUserInfo>>(
                &json!({
                    "alert": "Hello World!",
                    "foo": true,
                    "bar": -10,
                })
                .to_string()
            )
            .unwrap(),
            Payload::<TestUserInfo> {
                alert: Some(Alert {
                    body: "Hello World!".into(),
                    ..Default::default()
                }),
                user_info: Some(TestUserInfo {
                    foo: true,
                    bar: -10
                }),
                ..Default::default()
            }
        );
    }

    #[test]
    fn payload_ser() {
        assert_eq!(
            serde_json::to_value(&Payload {
                alert: Some(Alert {
                    body: "Hello World!".into(),
                    ..Default::default()
                }),
                badge: Some(11),
                sound: Some(Sound {
                    critical: false,
                    name: "default".into(),
                    volume: 0.
                }),
                thread_id: Some("my-thread-id".into()),
                category: Some("my-category".into()),
                content_available: true,
                mutable_content: true,
                target_content_id: Some("my-target-id".into()),
                interruption_level: Some(InterruptionLevel::Active),
                relevance_score: Some(0.5),
                user_info: Some(())
            })
            .unwrap(),
            json!({
                "alert": "Hello World!",
                "badge": 11,
                "sound": "default",
                "thread-id": "my-thread-id",
                "category": "my-category",
                "content-available": 1,
                "mutable-content": 1,
                "target-content-id": "my-target-id",
                "interruption-level": "active",
                "relevance-score": 0.5,
            })
        );
        assert_eq!(
            serde_json::to_value(&Payload::<TestUserInfo> {
                alert: Some(Alert {
                    body: "Hello World!".into(),
                    ..Default::default()
                }),
                user_info: Some(TestUserInfo {
                    foo: true,
                    bar: -10
                }),
                ..Default::default()
            })
            .unwrap(),
            json!({
                "alert": "Hello World!",
                "foo": true,
                "bar": -10,
            })
        );
    }

    #[test]
    fn alert_de() {
        assert_eq!(
            serde_json::from_str::<Alert>(&json!("Hello World!").to_string()).unwrap(),
            Alert {
                body: "Hello World!".into(),
                ..Default::default()
            }
        );
        assert_eq!(
            serde_json::from_str::<Alert>(
                &json!({
                    "body": "Hello World!"
                })
                .to_string()
            )
            .unwrap(),
            Alert {
                body: "Hello World!".into(),
                ..Default::default()
            }
        );
        assert_eq!(
            serde_json::from_str::<Alert>(
                &json!({
                    "title": "Title",
                    "subtitle": "Subtitle",
                    "body": "Hello World!",
                    "launch-image": "http://example.com/img.png",
                })
                .to_string()
            )
            .unwrap(),
            Alert {
                title: Some("Title".into()),
                subtitle: Some("Subtitle".into()),
                body: "Hello World!".into(),
                launch_image: Some("http://example.com/img.png".into()),
                ..Default::default()
            }
        );
        assert_eq!(
            serde_json::from_str::<Alert>(
                &json!({
                    "title": "Title",
                    "subtitle": "Subtitle",
                    "body": "Hello World!",
                    "launch-image": "http://example.com/img.png",
                    "title-loc-key": "REQUEST_FORMAT",
                    "title-loc-args": ["Foo", "Bar"],
                    "subtitle-loc-key": "SUBTITLE_FORMAT",
                    "subtitle-loc-args": ["Bar", "Baz"],
                    "loc-key": "BODY_FORMAT",
                    "loc-args": ["Apple", "Pie"],
                })
                .to_string()
            )
            .unwrap(),
            Alert {
                title: Some("Title".into()),
                subtitle: Some("Subtitle".into()),
                body: "Hello World!".into(),
                launch_image: Some("http://example.com/img.png".into()),
                title_loc_key: Some("REQUEST_FORMAT".into()),
                title_loc_args: Some(vec!["Foo".into(), "Bar".into()]),
                subtitle_loc_key: Some("SUBTITLE_FORMAT".into()),
                subtitle_loc_args: Some(vec!["Bar".into(), "Baz".into()]),
                loc_key: Some("BODY_FORMAT".into()),
                loc_args: Some(vec!["Apple".into(), "Pie".into()]),
            }
        );
    }

    #[test]
    fn alert_ser() {
        assert_eq!(
            serde_json::to_string(&Alert {
                body: "Hello World!".into(),
                ..Default::default()
            })
            .unwrap(),
            json!("Hello World!").to_string()
        );
        assert_eq!(
            serde_json::to_value(&Alert {
                title: Some("Title".into()),
                subtitle: Some("Subtitle".into()),
                body: "Hello World!".into(),
                launch_image: Some("http://example.com/img.png".into()),
                ..Default::default()
            })
            .unwrap(),
            json!({
                "title": "Title",
                "subtitle": "Subtitle",
                "body": "Hello World!",
                "launch-image": "http://example.com/img.png",
            })
        );
        assert_eq!(
            serde_json::to_value(&Alert {
                title: Some("Title".into()),
                subtitle: Some("Subtitle".into()),
                body: "Hello World!".into(),
                launch_image: Some("http://example.com/img.png".into()),
                title_loc_key: Some("REQUEST_FORMAT".into()),
                title_loc_args: Some(vec!["Foo".into(), "Bar".into()]),
                subtitle_loc_key: Some("SUBTITLE_FORMAT".into()),
                subtitle_loc_args: Some(vec!["Bar".into(), "Baz".into()]),
                loc_key: Some("BODY_FORMAT".into()),
                loc_args: Some(vec!["Apple".into(), "Pie".into()]),
            })
            .unwrap(),
            json!({
                "title-loc-key": "REQUEST_FORMAT",
                "title-loc-args": ["Foo", "Bar"],
                "subtitle-loc-key": "SUBTITLE_FORMAT",
                "subtitle-loc-args": ["Bar", "Baz"],
                "loc-key": "BODY_FORMAT",
                "loc-args": ["Apple", "Pie"],
                "launch-image": "http://example.com/img.png",
            })
        );
    }

    #[test]
    fn sound_de() {
        assert_eq!(
            serde_json::from_str::<Sound>(&json!("default").to_string()).unwrap(),
            Sound {
                critical: false,
                name: "default".into(),
                volume: 0.
            }
        );
        assert_eq!(
            serde_json::from_str::<Sound>(
                &json!({
                    "critical": 1,
                    "name": "custom",
                    "volume": 0.5,
                })
                .to_string()
            )
            .unwrap(),
            Sound {
                critical: true,
                name: "custom".into(),
                volume: 0.5
            }
        );
        assert_eq!(
            serde_json::from_str::<Sound>(
                &json!({
                    "critical": 0,
                    "name": "default",
                    "volume": 1.,
                })
                .to_string()
            )
            .unwrap(),
            Sound {
                critical: false,
                name: "default".into(),
                volume: 1.
            }
        );
        assert!(serde_json::from_str::<Sound>(
            &json!({
                "name": "default",
                "volume": 1.,
            })
            .to_string()
        )
        .is_err());
    }

    #[test]
    fn sound_ser() {
        assert_eq!(
            serde_json::to_string(&Sound {
                critical: false,
                name: "default".into(),
                volume: 0.
            })
            .unwrap(),
            json!("default").to_string(),
        );
        assert_eq!(
            serde_json::to_string(&Sound {
                critical: true,
                name: "custom".into(),
                volume: 0.5
            })
            .unwrap(),
            json!({
                "critical": 1,
                "name": "custom",
                "volume": 0.5,
            })
            .to_string()
        );
        assert_eq!(
            serde_json::to_string(&Sound {
                critical: true,
                name: "default".into(),
                volume: 1.
            })
            .unwrap(),
            json!({
                "critical": 1,
                "name": "default",
                "volume": 1.,
            })
            .to_string()
        );
        assert_eq!(
            serde_json::to_string(&Sound {
                critical: true,
                name: "default".into(),
                volume: 2.
            })
            .unwrap(),
            json!({
                "critical": 1,
                "name": "default",
                "volume": 1.,
            })
            .to_string()
        );
    }

    #[test]
    fn interruption_level_de() {
        assert_eq!(
            serde_json::from_str::<InterruptionLevel>("\"active\"").unwrap(),
            InterruptionLevel::Active
        );
        assert_eq!(
            serde_json::from_str::<InterruptionLevel>("\"critical\"").unwrap(),
            InterruptionLevel::Critical
        );
        assert_eq!(
            serde_json::from_str::<InterruptionLevel>("\"passive\"").unwrap(),
            InterruptionLevel::Passive
        );
        assert_eq!(
            serde_json::from_str::<InterruptionLevel>("\"time-sensitive\"").unwrap(),
            InterruptionLevel::TimeSensitive
        );
        assert!(serde_json::from_str::<InterruptionLevel>("\"invalid\"").is_err());
    }

    #[test]
    fn interruption_level_ser() {
        assert_eq!(
            serde_json::to_string(&InterruptionLevel::Active).unwrap(),
            "\"active\""
        );
        assert_eq!(
            serde_json::to_string(&InterruptionLevel::Critical).unwrap(),
            "\"critical\""
        );
        assert_eq!(
            serde_json::to_string(&InterruptionLevel::Passive).unwrap(),
            "\"passive\""
        );
        assert_eq!(
            serde_json::to_string(&InterruptionLevel::TimeSensitive).unwrap(),
            "\"time-sensitive\""
        );
    }

    #[test]
    fn interruption_level_from_str() {
        assert_eq!(
            InterruptionLevel::from_str("active").unwrap(),
            InterruptionLevel::Active
        );
        assert_eq!(
            InterruptionLevel::from_str("critical").unwrap(),
            InterruptionLevel::Critical
        );
        assert_eq!(
            InterruptionLevel::from_str("passive").unwrap(),
            InterruptionLevel::Passive
        );
        assert_eq!(
            InterruptionLevel::from_str("time-sensitive").unwrap(),
            InterruptionLevel::TimeSensitive
        );
        assert!(InterruptionLevel::from_str("invalid").is_err());
    }

    #[test]
    fn interruption_level_to_str() {
        assert_eq!(InterruptionLevel::Active.to_string(), "active");
        assert_eq!(InterruptionLevel::Critical.to_string(), "critical");
        assert_eq!(InterruptionLevel::Passive.to_string(), "passive");
        assert_eq!(
            InterruptionLevel::TimeSensitive.to_string(),
            "time-sensitive"
        );
    }
}
