use std::env;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, TokenData, Validation, decode, EncodingKey, DecodingKey};
use crate::error;
use crate::models::user::User;

lazy_static! {
    static ref JWT_SECRET: String = {
        env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into())
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub exp: i64,
}

pub trait CanGenerateJwt {
    fn generate_jwt(&self) -> Result<String, error::Error>;
}

impl CanGenerateJwt for User {
    fn generate_jwt(&self) -> Result<String, error::Error> {
        let exp = (Utc::now() + Duration::days(21)).timestamp();
        let claims = Claims { id: self.id, exp };

        let header = Header::default();
        let token = encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET.as_ref()))?;

        Ok(token)
    }
}

pub trait CanDecodeJwt {
    fn decode_jwt(&self) -> Result<TokenData<Claims>, error::Error>;
}

impl CanDecodeJwt for String {
    fn decode_jwt(&self) -> Result<TokenData<Claims>, error::Error> {
        match decode::<Claims>(&self, &DecodingKey::from_secret(JWT_SECRET.as_ref()), &Validation::default()) {
            Ok(res) => Ok(res),
            Err(e) => Err(e.into()),
        }
    }
}
