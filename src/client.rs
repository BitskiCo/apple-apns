use std::time::Duration;

use reqwest::Url;
use reqwest_middleware::ClientWithMiddleware;
use serde::Serialize;

use crate::payload::*;
use crate::reason::Reason;
use crate::request::ApnsRequest;
use crate::result::Result;

pub const DEVELOPMENT_SERVER: &str = "https://api.sandbox.push.apple.com";
pub const PRODUCTION_SERVER: &str = "https://api.push.apple.com";

pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Clone)]
pub struct ApnsClientBuilder<'a> {
    pub server: &'a str,
    pub user_agent: &'a str,
    // pub provider_token: Option<&'a str>,
}

impl<'a> Default for ApnsClientBuilder<'a> {
    fn default() -> Self {
        Self {
            server: PRODUCTION_SERVER,
            user_agent: USER_AGENT,
            // provider_token: None,
        }
    }
}

impl<'a> ApnsClientBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> Result<ApnsClient> {
        let base_url = format!("{}/3/device/", self.server).parse()?;

        let client = reqwest::Client::builder()
                .user_agent(self.user_agent)
                .pool_idle_timeout(None)
                .http2_prior_knowledge()
                .http2_keep_alive_interval(Some(Duration::from_secs(60 * 60)))
                .http2_keep_alive_timeout(Duration::from_secs(60))
                .http2_keep_alive_while_idle(true)
                // .min_tls_version(Version::TLS_1_2)
                ;

        // if let Some(provider_token) = self.provider_token {
        //     let mut headers = HeaderMap::new();
        //     let mut auth_value: HeaderValue = format!("bearer {provider_token}").parse()?;
        //     auth_value.set_sensitive(true);
        //     headers.insert(AUTHORIZATION, auth_value);
        //     client = client.default_headers(headers);
        // }

        let client = client.build()?;
        let client = reqwest_middleware::ClientBuilder::new(client).build();

        Ok(ApnsClient { base_url, client })
    }
}

#[derive(Debug, Clone)]
pub struct ApnsClient {
    base_url: Url,
    client: ClientWithMiddleware,
}

impl ApnsClient {
    pub fn builder<'a>() -> ApnsClientBuilder<'a> {
        ApnsClientBuilder::new()
    }

    pub async fn post<T>(&self, request: ApnsRequest<T>) -> Result<()>
    where
        T: Serialize,
    {
        let url = self.base_url.join(&request.device_token)?;
        let (headers, request): (_, ApnsPayload<T>) = request.try_into()?;

        let res = self
            .client
            .post(url)
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            let reason: Reason = res.json::<Reason>().await?;
            Err(reason.into())
        }
    }
}
