use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = Alphanumeric.sample_string(&mut thread_rng(), 60);
    Keys::new(secret.as_bytes())
});
