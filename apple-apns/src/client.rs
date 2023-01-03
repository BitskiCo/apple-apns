use std::time::Duration;

use reqwest::tls::Version;
#[cfg(feature = "rustls")]
use reqwest::{Certificate, Identity};
use reqwest_middleware::ClientWithMiddleware;
use serde::Serialize;
use url::Url;
use uuid::Uuid;

use crate::endpoint::Endpoint;
use crate::header::APNS_ID;
use crate::payload::*;
use crate::reason::Reason;
use crate::request::Request;
use crate::result::{Error, Result};
#[cfg(feature = "jwt")]
use crate::token::TokenFactory;

/// Default user agent.
pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Authentication options.
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

/// Certificate authority options.
#[cfg(feature = "rustls")]
#[cfg_attr(docsrs, doc(cfg(feature = "rustls")))]
#[derive(Debug, Clone)]
pub enum CertificateAuthority<'a> {
    Pem(&'a [u8]),
    Der(&'a [u8]),
}

/// [`Client`] builder.
#[derive(Debug, Clone)]
pub struct ClientBuilder<'a> {
    pub endpoint: Endpoint,
    pub user_agent: &'a str,

    #[cfg(feature = "rustls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rustls")))]
    pub ca: Option<CertificateAuthority<'a>>,

    #[cfg(any(feature = "rustls", feature = "jwt"))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "rustls", feature = "jwt"))))]
    pub authentication: Option<Authentication<'a>>,
}

impl<'a> Default for ClientBuilder<'a> {
    fn default() -> Self {
        Self {
            endpoint: Endpoint::default(),
            user_agent: USER_AGENT,

            #[cfg(feature = "rustls")]
            ca: None,

            #[cfg(any(feature = "rustls", feature = "jwt"))]
            authentication: None,
        }
    }
}

impl<'a> ClientBuilder<'a> {
    /// Creates a new [`ClientBuilder`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Builds a `Client`.
    pub fn build(&self) -> Result<Client> {
        let client = self.reqwest_client_builder()?.build();
        self.with_reqwest_middleware_client(client)
    }

    /// Builds a `Client` with middleware.
    pub fn build_with_middleware<F>(&self, f: F) -> Result<Client>
    where
        F: FnOnce(&mut reqwest_middleware::ClientBuilder) -> Result<()>,
    {
        let mut builder = self.reqwest_client_builder()?;
        f(&mut builder)?;
        self.with_reqwest_middleware_client(builder.build())
    }

    fn with_reqwest_middleware_client(&self, client: ClientWithMiddleware) -> Result<Client> {
        let base_url = self.endpoint.as_url().clone();

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

        Ok(Client {
            base_url,
            client,
            #[cfg(feature = "jwt")]
            token_factory,
        })
    }

    fn reqwest_client_builder(&self) -> Result<reqwest_middleware::ClientBuilder> {
        #[allow(unused_mut)]
        let mut builder = reqwest::Client::builder()
            .user_agent(self.user_agent)
            .pool_idle_timeout(None)
            .http2_keep_alive_interval(Some(Duration::from_secs(60 * 60)))
            .http2_keep_alive_timeout(Duration::from_secs(60))
            .http2_keep_alive_while_idle(true)
            .min_tls_version(Version::TLS_1_2);

        #[cfg(not(feature = "http1"))]
        {
            builder = builder.http2_prior_knowledge();
        }

        #[cfg(feature = "rustls")]
        {
            // Force rustls
            builder = builder.use_rustls_tls();

            // Add root certificate
            if let Some(ca) = &self.ca {
                let cert = match ca {
                    CertificateAuthority::Pem(pem) => Certificate::from_pem(pem)?,
                    CertificateAuthority::Der(der) => Certificate::from_der(der)?,
                };
                builder = builder.add_root_certificate(cert);
            }

            // Configure certificate authentication
            if let Some(Authentication::Certificate { client_pem }) = self.authentication {
                let identity = Identity::from_pem(client_pem)?;
                builder = builder.identity(identity);
            }
        }

        let client = builder.build()?;
        let builder = reqwest_middleware::ClientBuilder::new(client);
        Ok(builder)
    }
}

/// Apple Push Notification service client.
///
/// The [`Client`] is safe to use from multiple threads. However, [`Client`]
/// uses a [`std::sync::RwLock`] and is not [`Clone`]. To pass [`Client`] to
/// multiple threads, use [`std::sync::Arc`] for OS threads, or [`std::rc::Rc`]
/// for green threads.
pub struct Client {
    base_url: Url,
    client: ClientWithMiddleware,

    #[cfg(feature = "jwt")]
    token_factory: Option<TokenFactory>,
}

impl Client {
    /// Creates a [`ClientBuilder`].
    pub fn builder<'a>() -> ClientBuilder<'a> {
        ClientBuilder::new()
    }

    /// Sends a push notification and returns the APNS ID.
    pub async fn post<T>(&self, request: Request<T>) -> Result<Uuid>
    where
        T: Serialize,
    {
        let url = self.base_url.join(&request.device_token)?;
        let payload_size_limit = request.push_type.payload_size_limit();
        let (headers, payload): (_, Payload<T>) = request.try_into()?;

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
            let apns_id = res
                .headers()
                .get(&APNS_ID)
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok())
                .unwrap_or_default();
            Ok(apns_id)
        }
    }
}
