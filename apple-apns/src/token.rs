use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::result::Result;

/// JWT refresh period.
///
/// For security, APNs requires you to refresh your token regularly. Refresh
/// your token no more than once every 20 minutes and no less than once every 60
/// minutes. APNs rejects any request whose token contains a timestamp that is
/// more than one hour old. Similarly, APNs reports an error if you recreate
/// your tokens more than once every 20 minutes.
pub const JWT_REFRESH_PERIOD: Duration = Duration::from_secs(30 * 60);

#[derive(Debug, Serialize, Deserialize)]
struct Claims<'a> {
    iss: &'a str,
    iat: u64,
}

struct Token {
    jwt: Arc<String>,
    create_time: SystemTime,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            jwt: Default::default(),
            create_time: UNIX_EPOCH,
        }
    }
}

/// JWT token factory.
pub struct TokenFactory {
    key: EncodingKey,
    header: Header,
    iss: String,
    cache: RwLock<Token>,
}

impl TokenFactory {
    /// Creates a new [`TokenFactory`].
    pub fn new(key_id: &str, key_pem: &[u8], team_id: &str) -> Result<Self> {
        let key = EncodingKey::from_ec_pem(key_pem)?;
        let header = Header {
            alg: Algorithm::ES256,
            kid: Some(key_id.into()),
            ..Default::default()
        };

        let iss = team_id.into();

        let factory = TokenFactory {
            key,
            header,
            iss,
            cache: Default::default(),
        };

        // Crash OK: RwLock returns an error only if the lock is poisoned. The
        // lock is poisoned if the thread holding the write lock panics. No
        // threads are holding the write lock at this point.
        *factory.cache.write().unwrap() = factory.create_token()?;

        Ok(factory)
    }

    /// Gets a JWT that is valid for at least 30 minutes.
    pub fn get(&self) -> Result<Arc<String>> {
        // Crash OK: RwLock returns an error only if the lock is poisoned. The
        // lock is poisoned if the thread holding the write lock panics. There
        // are no panics in this file.
        let token = self.cache.read().unwrap();

        // Return the JWT if it is younger than the refresh period.
        if SystemTime::now().duration_since(token.create_time)? < JWT_REFRESH_PERIOD {
            Ok(token.jwt.clone())
        } else {
            self.refresh_token()
        }
    }

    fn create_token(&self) -> Result<Token> {
        let create_time = SystemTime::now();

        let iat = create_time.duration_since(UNIX_EPOCH)?.as_secs();

        let claims = Claims {
            iss: &self.iss,
            iat,
        };

        let jwt = jsonwebtoken::encode(&self.header, &claims, &self.key)?;

        Ok(Token {
            jwt: Arc::new(jwt),
            create_time,
        })
    }

    fn refresh_token(&self) -> Result<Arc<String>> {
        // Crash OK: RwLock returns an error only if the lock is poisoned.
        // The lock is poisoned if the thread holding the write lock panics.
        let mut cache = self.cache.write().unwrap();

        // Return early if another thread already refreshed the JWT.
        if SystemTime::now().duration_since(cache.create_time)? < JWT_REFRESH_PERIOD {
            return Ok(cache.jwt.clone());
        }

        // Refresh the JWT.
        let token = self.create_token()?;
        let jwt = token.jwt.clone();
        *cache = token;

        Ok(jwt)
    }
}
