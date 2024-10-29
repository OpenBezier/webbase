// use actix_http::header::HeaderMap;
// use actix_web::HttpRequest;
use anyhow;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

// #[async_trait::async_trait]
// pub trait TokenPermission {
//     async fn check_and_verify(&self, req: (HeaderMap, String)) -> anyhow::Result<AccessToken>;
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessToken {
    pub user_id: u64,
    pub user_name: String,
    pub user_account: String,
    pub app_id: String,
    pub exp: u128,
}

impl AccessToken {
    pub fn encode_token(
        user_id: u64,
        user_account: &String,
        user_name: &String,
        app_id: &String,
        timeout_hour: u16,
        secret: &str,
    ) -> anyhow::Result<String> {
        let header = Header::new(Algorithm::HS512);
        let my_claims = AccessToken {
            user_id: user_id.clone(),
            user_account: user_account.clone(),
            user_name: user_name.clone(),
            app_id: app_id.clone(),
            exp: (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
                + 1000 * 60 * 60 * (timeout_hour as u128)),
        };
        if let Ok(token) = encode(
            &header,
            &my_claims,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            return anyhow::Ok(token);
        } else {
            return Err(anyhow::anyhow!("encode_token with error"));
        }
    }

    pub fn decode_token(token: &String, secret: &str) -> anyhow::Result<Self> {
        if let Ok(decode_data) = decode::<Self>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS512),
        ) {
            return anyhow::Ok(decode_data.claims);
        } else {
            return Err(anyhow::anyhow!("decode_token with error"));
        }
    }

    pub fn is_expired(&self) -> bool {
        let cur_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        if self.exp < cur_time {
            true
        } else {
            false
        }
    }
}

pub const ACCESS_TOKEN_TIME: u16 = 2 * 24;
pub const REFRESH_TOKEN_TIME: u16 = 7 * 24;

pub fn create_access_token(
    user_id: u64,
    user_account: &String,
    user_name: &String,
    app_id: &String,
    secret: &str,
) -> anyhow::Result<String> {
    AccessToken::encode_token(
        user_id,
        user_account,
        user_name,
        app_id,
        ACCESS_TOKEN_TIME,
        secret,
    )
}

pub fn create_refresh_token(
    user_id: u64,
    user_account: &String,
    user_name: &String,
    app_id: &String,
    secret: &str,
) -> anyhow::Result<String> {
    AccessToken::encode_token(
        user_id,
        user_account,
        user_name,
        app_id,
        REFRESH_TOKEN_TIME,
        secret,
    )
}

// pub fn get_token_and_path(req: &HttpRequest) -> (HeaderMap, String) {
//     let headers = req.headers();
//     let reqpath = format!("{} {}", req.method(), req.path());
//     (headers.clone(), reqpath)
// }
