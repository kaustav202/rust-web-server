use crate::{models, Result, errors};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
};
use log::debug;

fn get_secret() -> Vec<u8> {
    std::env::var("JWT_SECRET").unwrap().into_bytes()
}

pub fn get_hashed_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Scrypt
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    password_hash
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(password_hash).unwrap();

    Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub fn get_jwt_for_user(user: &models::User) -> String {
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::seconds(6000))
        .expect("invalid timestamp")
        .timestamp();
    let user_claims = models::Claims {
        sub: user.username.clone(),
        exp: expiration_time as usize
    };

    let token = match encode(
        &Header::default(),
        &user_claims,
        &EncodingKey::from_secret(&get_secret()),
    ) {
        Ok(t) => t,
        Err(_) => panic!(),
    };

    token
}

pub fn with_auth( ) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>|  headers )
        .and_then(authorize)
}


fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> std::result::Result<String, errors::CustomError> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(errors::CustomError::AuthHeaderRequiredError),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(errors::CustomError::AuthHeaderRequiredError),
    };
    if !auth_header.starts_with("Bearer ") {
        return Err(errors::CustomError::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches("Bearer ").to_owned())
}

async fn authorize( headers :  HeaderMap<HeaderValue>) -> Result<String> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<models::Claims>(
                &jwt,
                &DecodingKey::from_secret(&get_secret()),
                &Validation::default(),
            )
            .map_err(|_| reject::custom(errors::CustomError::InvalidJWTTokenError))?;

            debug!("decoded claims: {:?}", &decoded.claims);
            
            Ok(decoded.claims.sub)
        }
        Err(e) => return Err(reject::custom(e)),
    }
}
