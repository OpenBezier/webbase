use super::ClientRsp;
use anyhow::anyhow;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtEncodeReq {
    pub account: String,
    pub name: String,
    pub appid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtEncodeRsp {
    pub access: String,
    pub refresh: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtAccessRsp {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtDecodeRsp {
    pub account: String,
    pub name: String,
    pub appid: String,
    pub exp: u128,
}

pub async fn keycenter_check_health(url: &String) -> anyhow::Result<()> {
    let strip_url = super::common::strip_url_last_slash(&url);
    let url = format!("{}/api/v1/keycenter/health", strip_url);
    let response = Client::new().get(url).send().await?;
    if response.status().is_success() {
        return Ok(());
    } else {
        return Err(anyhow!("error: {:?}", response.status()));
    }
}

pub async fn keycenter_encode_token(
    url: &String,
    data: JwtEncodeReq,
) -> anyhow::Result<JwtEncodeRsp> {
    let strip_url = super::common::strip_url_last_slash(&url);
    let url = format!("{}/api/v1/keycenter/encode", strip_url);
    let response = Client::new().post(url).json(&data).send().await?;
    if response.status().is_success() {
        match response.json::<ClientRsp>().await {
            Ok(resp) => {
                let status = resp.status;
                if status {
                    let resp_data =
                        serde_json::from_value::<JwtEncodeRsp>(resp.message.clone()).unwrap();
                    return Ok(resp_data);
                } else {
                    let reason = serde_json::from_value::<String>(resp.message.clone()).unwrap();
                    return Err(anyhow!("error reason: {}", reason));
                }
            }
            Err(e) => {
                return Err(anyhow!("error as response data decoding: {:?}", e));
            }
        }
    } else {
        return Err(anyhow!("error: {:?}", response.status()));
    }
}

pub async fn keycenter_access_token(
    url: &String,
    data: JwtEncodeReq,
) -> anyhow::Result<JwtAccessRsp> {
    let strip_url = super::common::strip_url_last_slash(&url);
    let url = format!("{}/api/v1/keycenter/access", strip_url);
    let response = Client::new().post(url).json(&data).send().await?;
    if response.status().is_success() {
        match response.json::<ClientRsp>().await {
            Ok(resp) => {
                let status = resp.status;
                if status {
                    let resp_data =
                        serde_json::from_value::<JwtAccessRsp>(resp.message.clone()).unwrap();
                    return Ok(resp_data);
                } else {
                    let reason = serde_json::from_value::<String>(resp.message.clone()).unwrap();
                    return Err(anyhow!("error reason: {}", reason));
                }
            }
            Err(e) => {
                return Err(anyhow!("error as response data decoding: {:?}", e));
            }
        }
    } else {
        return Err(anyhow!("error: {:?}", response.status()));
    }
}

pub async fn keycenter_refresh_token(
    url: &String,
    data: JwtEncodeReq,
) -> anyhow::Result<JwtAccessRsp> {
    let strip_url = super::common::strip_url_last_slash(&url);
    let url = format!("{}/api/v1/keycenter/refresh", strip_url);
    let response = Client::new().post(url).json(&data).send().await?;
    if response.status().is_success() {
        match response.json::<ClientRsp>().await {
            Ok(resp) => {
                let status = resp.status;
                if status {
                    let resp_data =
                        serde_json::from_value::<JwtAccessRsp>(resp.message.clone()).unwrap();
                    return Ok(resp_data);
                } else {
                    let reason = serde_json::from_value::<String>(resp.message.clone()).unwrap();
                    return Err(anyhow!("error reason: {}", reason));
                }
            }
            Err(e) => {
                return Err(anyhow!("error as response data decoding: {:?}", e));
            }
        }
    } else {
        return Err(anyhow!("error: {:?}", response.status()));
    }
}

pub async fn keycenter_docode_token(url: &String, token: String) -> anyhow::Result<JwtDecodeRsp> {
    let strip_url = super::common::strip_url_last_slash(&url);
    let url = format!("{}/api/v1/keycenter/decode", strip_url);
    let response = Client::new()
        .post(url)
        .json(&JwtAccessRsp { token: token })
        .send()
        .await?;
    if response.status().is_success() {
        match response.json::<ClientRsp>().await {
            Ok(resp) => {
                let status = resp.status;
                if status {
                    let resp_data =
                        serde_json::from_value::<JwtDecodeRsp>(resp.message.clone()).unwrap();
                    return Ok(resp_data);
                } else {
                    let reason = serde_json::from_value::<String>(resp.message.clone()).unwrap();
                    return Err(anyhow!("error reason: {}", reason));
                }
            }
            Err(e) => {
                return Err(anyhow!("error as response data decoding: {:?}", e));
            }
        }
    } else {
        return Err(anyhow!("error: {:?}", response.status()));
    }
}

pub async fn keycenter_check_token_is_expired(
    url: &String,
    token: String,
) -> anyhow::Result<(bool, JwtDecodeRsp)> {
    let data = keycenter_docode_token(url, token).await?;
    let cur_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    if data.exp < cur_time {
        Ok((true, data))
    } else {
        Ok((false, data))
    }
}
