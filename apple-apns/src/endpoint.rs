use std::{fmt::Debug, str::FromStr};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use url::Url;

static PRODUCTION_SERVER: Lazy<Url> =
    Lazy::new(|| Url::parse("https://api.push.apple.com./3/device/").unwrap());

static DEVELOPMENT_SERVER: Lazy<Url> =
    Lazy::new(|| Url::parse("https://api.sandbox.push.apple.com./3/device/").unwrap());

/// Apple Push Notification service endpoint.
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Endpoint {
    Production,
    Development,
    Custom(Url),
}

impl Endpoint {
    pub fn as_url(&self) -> &Url {
        match self {
            Self::Production => &PRODUCTION_SERVER,
            Self::Development => &DEVELOPMENT_SERVER,
            Self::Custom(url) => url,
        }
    }
}

impl Debug for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Production => f.debug_tuple("Production").field(self.as_url()).finish(),
            Self::Development => f.debug_tuple("Development").field(self.as_url()).finish(),
            Self::Custom(url) => f.debug_tuple("Custom").field(url).finish(),
        }
    }
}

impl Default for Endpoint {
    fn default() -> Self {
        Self::Production
    }
}

impl FromStr for Endpoint {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("prod") || s.eq_ignore_ascii_case("production") {
            Ok(Self::Production)
        } else if s.eq_ignore_ascii_case("dev") || s.eq_ignore_ascii_case("development") {
            Ok(Self::Development)
        } else {
            Url::parse(s).map(Self::Custom)
        }
    }
}

impl<'de> Deserialize<'de> for Endpoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, Unexpected};

        let s = String::deserialize(deserializer)?;
        Endpoint::from_str(&s)
            .map_err(|err| Error::invalid_value(Unexpected::Str(&s), &err.to_string().as_str()))
    }
}

impl Serialize for Endpoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_url().as_str())
    }
}
