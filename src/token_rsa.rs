use super::AccessToken;
use anyhow;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessTokenRsa {
    pub user_id: u64,
    pub user_name: String,
    pub user_account: String,
    pub app_id: String,
    pub exp: u128,
}

impl AccessTokenRsa {
    pub fn to_token(&self) -> AccessToken {
        AccessToken {
            user_id: self.user_id.clone(),
            user_name: self.user_name.clone(),
            user_account: self.user_account.clone(),
            app_id: self.app_id.clone(),
            exp: self.exp.clone(),
        }
    }

    pub fn encode_token(
        user_id: u64,
        user_account: &String,
        user_name: &String,
        app_id: &String,
        timeout_hour: u16,
        rsa_private: &String,
    ) -> anyhow::Result<String> {
        let header = Header::new(Algorithm::RS512);
        let my_claims = AccessTokenRsa {
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
            &EncodingKey::from_rsa_pem(rsa_private.as_ref())?,
        ) {
            return anyhow::Ok(token);
        } else {
            return Err(anyhow::anyhow!("encode_token with error"));
        }
    }

    pub fn decode_token(token: &String, rsa_public: &String) -> anyhow::Result<Self> {
        if let Ok(decode_data) = decode::<Self>(
            &token,
            &DecodingKey::from_rsa_pem(rsa_public.as_ref())?,
            &Validation::new(Algorithm::RS512),
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
