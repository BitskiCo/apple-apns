use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::result::Result;

pub const JWT_REFRESH_PERIOD: Duration = Duration::from_secs(20 * 60);

#[derive(Debug, Serialize, Deserialize)]
struct Claims<'a> {
    iss: &'a str,
    iat: u64,
}

struct Cache {
    jwt: Arc<String>,
    create_time: SystemTime,
}

#[derive(Debug, Clone)]
pub struct TokenFactoryBuilder<'a> {
    pub key_id: &'a str,
    pub key_pem: &'a [u8],
    pub team_id: &'a str,
}

impl<'a> TokenFactoryBuilder<'a> {
    pub fn build(&self) -> Result<TokenFactory> {
        let key = EncodingKey::from_ec_pem(self.key_pem)?;
        let header = Header {
            alg: Algorithm::ES256,
            kid: Some(self.key_id.into()),
            ..Default::default()
        };

        let iss = self.team_id.into();

        let cache = RwLock::new(Cache {
            jwt: Default::default(),
            create_time: UNIX_EPOCH,
        });

        Ok(TokenFactory {
            key,
            header,
            iss,
            cache,
        })
    }
}

pub struct TokenFactory {
    key: EncodingKey,
    header: Header,
    iss: String,
    cache: RwLock<Cache>,
}

impl TokenFactory {
    pub fn get(&self) -> Result<Arc<String>> {
        let cache = self.cache.read().unwrap();
        if SystemTime::now().duration_since(cache.create_time)? < JWT_REFRESH_PERIOD {
            Ok(cache.jwt.clone())
        } else {
            let cache = self.create()?;
            let jwt = cache.jwt.clone();
            *self.cache.write().unwrap() = cache;
            Ok(jwt)
        }
    }

    fn create(&self) -> Result<Cache> {
        let create_time = SystemTime::now();

        let iat = create_time.duration_since(UNIX_EPOCH)?.as_secs();

        let claims = Claims {
            iss: &self.iss,
            iat,
        };

        let jwt = jsonwebtoken::encode(&self.header, &claims, &self.key)?;
        Ok(Cache {
            jwt: Arc::new(jwt),
            create_time,
        })
    }
}
