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

impl Default for Cache {
    fn default() -> Self {
        Self {
            jwt: Default::default(),
            create_time: UNIX_EPOCH,
        }
    }
}

pub struct TokenFactory {
    key: EncodingKey,
    header: Header,
    iss: String,
    cache: RwLock<Cache>,
}

impl TokenFactory {
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

        *factory.cache.write().unwrap() = factory.create()?;

        Ok(factory)
    }

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
