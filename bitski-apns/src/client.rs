use std::time::Duration;

use http::{header, HeaderValue};
use reqwest::tls::Version;
#[cfg(feature = "rustls")]
use reqwest::{Certificate, Identity};
use reqwest::{ClientBuilder, Url};
use reqwest_middleware::ClientWithMiddleware;
use serde::Serialize;

use crate::payload::*;
use crate::reason::Reason;
use crate::request::ApnsRequest;
use crate::result::{Error, Result};
#[cfg(feature = "jwt")]
use crate::token::TokenFactory;

pub const DEVELOPMENT_SERVER: &str = "https://api.sandbox.push.apple.com";
pub const PRODUCTION_SERVER: &str = "https://api.push.apple.com";

pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[cfg(any(feature = "rustls", feature = "jwt"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "rustls", feature = "jwt"))))]
#[derive(Debug, Clone)]
pub enum Authentication<'a> {
    /// If you’re using certificate-based authentication, you send your provider
    /// certificate to APNs when setting up your TLS connection. For more
    /// information, see [Establishing a Certificate-Based Connection to
    /// APNs](https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/establishing_a_certificate-based_connection_to_apns).
    #[cfg(feature = "rustls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rustls")))]
    Certificate { client_pem: &'a [u8] },

    /// (Required for token-based authentication) The value of this header is
    /// bearer <provider_token>, where <provider_token> is the encrypted token
    /// that authorizes you to send notifications for the specified topic. APNs
    /// ignores this header if you use certificate-based authentication. For
    /// more information, see [Establishing a Token-Based Connection to
    /// APNs](https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/establishing_a_token-based_connection_to_apns).
    #[cfg(feature = "jwt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "jwt")))]
    Token {
        key_id: &'a str,
        key_pem: &'a [u8],
        team_id: &'a str,
    },
}

#[cfg(feature = "rustls")]
#[cfg_attr(docsrs, doc(cfg(feature = "rustls")))]
#[derive(Debug, Clone)]
pub enum CertificateAuthority<'a> {
    Pem(&'a [u8]),
    Der(&'a [u8]),
}

#[derive(Debug, Clone)]
pub struct ApnsClientBuilder<'a> {
    pub server: &'a str,
    pub user_agent: &'a str,

    #[cfg(feature = "rustls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rustls")))]
    pub ca: Option<CertificateAuthority<'a>>,

    #[cfg(any(feature = "rustls", feature = "jwt"))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "rustls", feature = "jwt"))))]
    pub authentication: Option<Authentication<'a>>,
}

impl<'a> Default for ApnsClientBuilder<'a> {
    fn default() -> Self {
        Self {
            server: PRODUCTION_SERVER,
            user_agent: USER_AGENT,

            #[cfg(feature = "rustls")]
            ca: None,

            #[cfg(any(feature = "rustls", feature = "jwt"))]
            authentication: None,
        }
    }
}

impl<'a> ApnsClientBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(&self) -> Result<ApnsClient> {
        let client = self.reqwest_client_builder()?.build()?;
        self.with_reqwest_client(client)
    }

    pub fn with_reqwest_client(&self, client: reqwest::Client) -> Result<ApnsClient> {
        let client = reqwest_middleware::ClientBuilder::new(client).build();
        self.with_reqwest_middleware_client(client)
    }

    pub fn with_reqwest_middleware_client(
        &self,
        client: ClientWithMiddleware,
    ) -> Result<ApnsClient> {
        let base_url = format!("{}/3/device/", self.server).parse()?;

        #[cfg(feature = "jwt")]
        let token_factory = if let Some(Authentication::Token {
            key_id,
            key_pem,
            team_id,
        }) = self.authentication
        {
            Some(TokenFactory::new(key_id, key_pem, team_id)?)
        } else {
            None
        };

        Ok(ApnsClient {
            base_url,
            client,
            #[cfg(feature = "jwt")]
            token_factory,
        })
    }

    pub fn reqwest_client_builder(&self) -> Result<ClientBuilder> {
        #[allow(unused_mut)]
        let mut builder = reqwest::Client::builder()
            .user_agent(self.user_agent)
            .pool_idle_timeout(None)
            .http2_prior_knowledge()
            .http2_keep_alive_interval(Some(Duration::from_secs(60 * 60)))
            .http2_keep_alive_timeout(Duration::from_secs(60))
            .http2_keep_alive_while_idle(true)
            .min_tls_version(Version::TLS_1_2);

        #[cfg(feature = "rustls")]
        if let Some(ca) = &self.ca {
            let cert = match ca {
                CertificateAuthority::Pem(pem) => Certificate::from_pem(pem)?,
                CertificateAuthority::Der(der) => Certificate::from_der(der)?,
            };
            builder = builder.add_root_certificate(cert);
        }

        #[cfg(feature = "rustls")]
        if let Some(Authentication::Certificate { client_pem }) = self.authentication {
            let identity = Identity::from_pem(client_pem)?;
            builder = builder.identity(identity);
        }

        Ok(builder)
    }
}

pub struct ApnsClient {
    base_url: Url,
    client: ClientWithMiddleware,

    #[cfg(feature = "jwt")]
    token_factory: Option<TokenFactory>,
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
        let payload_size_limit = request.apns_push_type.payload_size_limit();
        let (mut headers, payload): (_, ApnsPayload<T>) = request.try_into()?;
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        let body = serde_json::to_vec(&payload)?;
        if body.len() > payload_size_limit {
            return Err(Error::PayloadTooLarge {
                size: body.len(),
                limit: payload_size_limit,
            });
        }

        #[allow(unused_mut)]
        let mut req = self.client.post(url).headers(headers).body(body);

        #[cfg(feature = "jwt")]
        if let Some(token_factory) = &self.token_factory {
            let jwt = token_factory.get()?;
            req = req.bearer_auth(jwt);
        }

        let res = req.send().await?;

        if let Err(err) = res.error_for_status_ref() {
            if let Ok(reason) = res.json::<Reason>().await {
                Err(reason.into())
            } else {
                Err(err.into())
            }
        } else {
            Ok(())
        }
    }
}