use crate::{rbac::RpConfig, ClientRsp};
use anyhow::anyhow;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock};

#[derive(Deserialize, Serialize, Debug)]
pub struct AppReadRbacRsp {
    pub rbac: Option<RpConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginReq {
    pub appname: String,
    pub appsecret: String,
    pub account: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRsp {
    pub access_token: String,
    pub refresh_token: String,
    pub user_name: String,
    pub user_account: String,
    pub rbac: Option<RpConfig>,
}

#[derive(Clone, Debug)]
pub struct IdmManager {
    pub server: String,
    pub client: reqwest::Client,
    pub rpinfo: Arc<DashMap<String, RpConfig>>,
}

static IDM: OnceLock<IdmManager> = OnceLock::<IdmManager>::new();

pub fn init_idm(server: String, app_name: String) -> &'static IdmManager {
    IDM.get_or_init(|| IdmManager::new(server, app_name))
}

pub fn get_idm() -> &'static IdmManager {
    IDM.get().unwrap()
}

impl IdmManager {
    pub fn new(server: String, app_name: String) -> Self {
        let client = reqwest::Client::builder()
            .http1_only()
            .connect_timeout(std::time::Duration::from_secs(5))
            .timeout(std::time::Duration::from_secs(5))
            // .tcp_keepalive(std::time::Duration::from_secs(120))
            .build()
            .unwrap();
        let manager = Self {
            server: server.clone(),
            client: client.clone(),
            rpinfo: Arc::new(DashMap::default()),
        };

        let url = format!("{}/api/v1/janus/app/readrbac/{}", server, app_name);
        let rbac = futures::executor::block_on(async {
            let response = client
                .get(url)
                .send()
                .await
                .map_err(|e| {
                    tracing::error!("IDM服务器连接出错");
                    e
                })
                .unwrap();
            let response = response
                .json::<ClientRsp>()
                .await
                .map_err(|e| {
                    tracing::error!("IDM服务器返回的数据格式有误");
                    e
                })
                .unwrap();
            if response.status {
                let resp_data = serde_json::from_value::<AppReadRbacRsp>(response.message.clone())
                    .map_err(|e| {
                        tracing::error!("IDM服务器返回的RBAC数据格式有误");
                        e
                    })
                    .unwrap();
                resp_data.rbac
            } else {
                panic!("获取IDM服务RBAC信息失败-{}", app_name);
            }
        });

        if rbac.is_some() {
            tracing::warn!("use remote rbac config file");
            manager.rpinfo.insert(app_name, rbac.unwrap());
        }
        manager
    }

    pub async fn app_login(
        &self,
        app_name: String,
        app_secret: String,
        user_name: String,
        user_pass: String,
    ) -> anyhow::Result<LoginRsp> {
        let req_data = LoginReq {
            appname: app_name.clone(),
            appsecret: app_secret,
            account: user_name.clone(),
            password: user_pass.clone(),
        };
        let url = format!("{}/api/v1/janus/app/applogin", self.server);
        let response = self.client.post(url).json(&req_data).send().await?;

        if response.status().is_success() {
            match response.json::<ClientRsp>().await {
                Ok(resp) => {
                    let status = resp.status;
                    if status {
                        let resp_data =
                            serde_json::from_value::<LoginRsp>(resp.message.clone()).unwrap();
                        if resp_data.rbac.is_some() {
                            tracing::warn!("update remote rbac config file");
                            self.rpinfo
                                .insert(app_name, resp_data.rbac.clone().unwrap());
                        }
                        Ok(resp_data)
                    } else {
                        let reason =
                            serde_json::from_value::<String>(resp.message.clone()).unwrap();
                        return Err(anyhow!("login error reason: {}", reason));
                    }
                }
                Err(e) => {
                    return Err(anyhow!("login error as response data decoding: {:?}", e));
                }
            }
        } else {
            return Err(anyhow!("login error: {:?}", response.status()));
        }
    }
}
