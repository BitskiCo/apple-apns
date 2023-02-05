use std::path::PathBuf;

use anyhow::{anyhow, Result};
use apple_apns::{Endpoint, InterruptionLevel, Priority, PushType};
use clap::{ArgGroup, Parser};
use humantime::parse_duration;
use time::{format_description::well_known::Iso8601, OffsetDateTime};
use uuid::Uuid;

/// Apple APNS
#[derive(Parser)]
#[command(author, version, about)]
#[command(group(
    ArgGroup::new("authentication")
        .args(["client_pem_file", "key_pem_file"])
        .required(true)
), group(
    ArgGroup::new("certificate")
        .conflicts_with("token")
        .arg("client_pem_file")
        .requires("client_pem_file")
), group(
    ArgGroup::new("token")
        .conflicts_with("certificate")
        .args(["key_id", "key_pem_file", "team_id"])
        .requires_all(["key_id", "key_pem_file", "team_id"])
        .multiple(true)
))]
pub struct Cli {
    #[arg(long, env)]
    pub ca_pem_file: Option<PathBuf>,

    #[arg(long, env)]
    pub client_pem_file: Option<PathBuf>,

    #[arg(long, env)]
    pub key_id: Option<String>,

    #[arg(long, env)]
    pub key_pem_file: Option<String>,

    #[arg(long, env)]
    pub team_id: Option<String>,

    #[arg(long, env)]
    pub endpoint: Option<Endpoint>,

    #[arg(long, env)]
    pub user_agent: Option<String>,

    /// The hex-encoded device token.
    #[arg(long, env)]
    pub device_token: String,

    /// The push type of the notification to send.
    #[arg(long, env, default_value_t = PushType::Alert)]
    pub push_type: PushType,

    /// A canonical UUID that is the unique ID for the notification.
    #[arg(long, env)]
    pub id: Option<Uuid>,

    /// The date at which the notification is no longer valid.
    #[arg(long, env, value_parser = parse_timestamp)]
    pub expiration: Option<OffsetDateTime>,

    /// The priority of the notification.
    #[arg(long, env, default_value_t = Default::default())]
    pub priority: Priority,

    /// The topic for the notification, e.g. bundle ID or app ID.
    #[arg(long, env)]
    pub topic: Option<String>,

    /// An identifier you use to coalesce multiple notifications into a single
    /// notification for the user.
    #[arg(long, env)]
    pub collapse_id: Option<String>,

    /// The title of the notification.
    #[arg(long, env)]
    pub title: Option<String>,

    /// Additional information that explains the purpose of the
    /// notification.
    #[arg(long, env)]
    pub subtitle: Option<String>,

    /// The content of the alert message.
    #[arg(long, env)]
    pub body: Option<String>,

    /// The name of the launch image file to display.
    #[arg(long, env)]
    pub launch_image: Option<String>,

    /// The number to display in a badge on your app’s icon. Specify `0` to
    /// remove the current badge, if any.
    #[arg(long, env)]
    pub badge: Option<u32>,

    /// The name of a sound file in your app’s main bundle or in the
    /// `Library/Sounds` folder of your app’s container directory.
    #[arg(long, env)]
    pub sound: Option<String>,

    /// The volume for the critical alert’s sound. Set this to a value
    /// between `0` (silent) and `1` (full volume).
    #[arg(long, env)]
    pub volume: Option<f64>,

    /// An app-specific identifier for grouping related notifications.
    #[arg(long, env)]
    pub thread_id: Option<String>,

    /// The notification’s type.
    #[arg(long, env)]
    pub category: Option<String>,

    /// The background notification flag.
    #[arg(long, env, default_value_t = false)]
    pub content_available: bool,

    /// The notification service app extension flag.
    #[arg(long, env, default_value_t = false)]
    pub mutable_content: bool,

    /// The identifier of the window brought forward.
    #[arg(long, env)]
    pub target_content_id: Option<String>,

    /// The importance and delivery timing of a notification.
    #[arg(long, env)]
    pub interruption_level: Option<InterruptionLevel>,

    /// The relevance score, a number between `0` and `1`.
    #[arg(long, env)]
    pub relevance_score: Option<f64>,

    /// Additional data to send.
    #[arg(long, env)]
    pub user_info: Option<serde_json::Value>,
}

fn parse_timestamp(arg: &str) -> Result<OffsetDateTime> {
    match OffsetDateTime::parse(arg, &Iso8601::DEFAULT) {
        Ok(timestamp) => Ok(timestamp),
        Err(duration_err) => match parse_duration(arg) {
            Ok(duration) => Ok(OffsetDateTime::now_utc() + duration),
            Err(timestamp_err) => Err(anyhow!("Invalid expiration; invalid timestamp: {timestamp_err}; invalid duration: {duration_err}")),
        },
    }
}
