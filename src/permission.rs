use crate::token::AccessToken;
use crate::token_rsa::AccessTokenRsa;
use crate::{idmlogin::get_idm, rbac::get_rbac};
use crate::{Response, RpConfig};
use actix_http::header::HeaderMap;
use actix_web::HttpRequest;
use actix_web::{error, Error};
use anyhow::anyhow;
use dashmap::DashMap;
use regex::Regex;
use serde::Serialize;
use std::sync::OnceLock;
use std::{collections::HashMap, fmt::Debug};

#[derive(Default)]
pub struct PermissionMap {
    pub pmap: HashMap<&'static str, (&'static str, &'static str)>, // 权限检查的列表
    pub whitelist: Vec<&'static str>,                              // 白名单API列表
    pub secret: String,   // 解析token的秘钥, 可以为RSA公钥或HS512的秘钥
    pub use_rsa: bool,    // 是否使用RSA方式进行解密，否则使用秘钥的HS512方式
    pub app_name: String, // 当前业务的名字
    pub use_local: bool,  // 使用本地端的RBAC配置，不优先使用IDM服务器的
    pub config: DashMap<String, String>, // 存放系统相关的一些配置信息，业务自行决定使用
}

static PMAP: OnceLock<PermissionMap> = OnceLock::<PermissionMap>::new();

pub fn init_pmap(
    whitelist: Vec<&'static str>,
    pmap: HashMap<&'static str, (&'static str, &'static str)>,
    secret: String,
    app_name: String,
    use_local: bool,
    use_rsa: bool,
) -> &'static PermissionMap {
    PMAP.get_or_init(|| PermissionMap {
        pmap: pmap,
        whitelist: whitelist,
        secret: secret,
        app_name: app_name,
        use_local: use_local,
        use_rsa: use_rsa,
        config: DashMap::default(),
    })
}

pub fn get_pmap() -> &'static PermissionMap {
    PMAP.get().unwrap()
}

impl PermissionMap {
    pub async fn get_rbac_config(&self) -> RpConfig {
        // janus服务不会初始化idm，所有直接返回rbac里面的配置
        if self.use_local {
            return get_rbac().rbac.read().await.clone();
        } else {
            let app_rpinfo = if let Some(rpinfo) = get_idm().rpinfo.get(&self.app_name) {
                rpinfo.value().clone()
            } else {
                get_rbac().rbac.read().await.clone()
            };
            app_rpinfo
        }
    }

    pub async fn check_and_verify(&self, req: &HttpRequest) -> anyhow::Result<AccessToken> {
        let (headers, reqpath) = get_token_and_path(req);
        // tracing::info!("check for {:?}", reqpath);

        if !headers.contains_key("Authorization") {
            return Err(anyhow!("Header中无鉴权信息"));
        }

        let token = headers
            .get("Authorization")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        if !token.contains("Bearer ") || token.len() < 7 {
            return Err(anyhow!("Header中鉴权信息的格式不对"));
        }

        let token = token.split_at(7).1.to_string();
        let access = if self.use_rsa {
            let access = AccessTokenRsa::decode_token(&token, &self.secret);
            if access.is_ok() {
                access.unwrap().to_token()
            } else {
                return Err(anyhow!("解析AccessToken鉴权信息识别"));
            }
        } else {
            let access = AccessToken::decode_token(&token, self.secret.as_str());
            if access.is_ok() {
                access.unwrap()
            } else {
                return Err(anyhow!("解析AccessToken鉴权信息识别"));
            }
        };

        // 非正则表达式进行匹配 白名单接口直接放行
        if self.whitelist.contains(&reqpath.as_str()) {
            return Ok(access);
        }
        // 正则表达式进行匹配 白名单接口直接放行
        for each in self.whitelist.iter() {
            if let Ok(re) = Regex::new(*each) {
                if re.captures(&reqpath).is_some() {
                    return Ok(access);
                }
            }
        }

        let user_account = access.user_account.clone();
        // 优先使用远端IDM-Janus的配置信息
        let app_rpinfo = self.get_rbac_config().await;

        // 非正则表达式进行匹配
        if let Some((page, item)) = self.pmap.get(reqpath.as_str()) {
            let check_status = app_rpinfo.check_user_action(
                user_account.clone(),
                page.to_string(),
                item.to_string(),
            );
            if check_status.0 {
                // 任意一个角色下，包括default，只要判断了page+item是enabled状态，直接返回成功
                return Ok(access);
            }
        }
        // 正则表达式进行匹配
        for (each_route, (page, item)) in self.pmap.iter() {
            if let Ok(re) = Regex::new(*each_route) {
                if re.captures(&reqpath).is_some() {
                    let check_status = app_rpinfo.check_user_action(
                        user_account.clone(),
                        page.to_string(),
                        item.to_string(),
                    );
                    if check_status.0 {
                        // 任意一个角色下，包括default，只要判断了page+item是enabled状态，直接返回成功
                        return Ok(access);
                    }
                }
            }
        }
        return Err(anyhow!("核查所有权限后该用户无对应权限:{:?}", user_account));
    }
}

pub fn get_token_and_path(req: &HttpRequest) -> (HeaderMap, String) {
    let headers = req.headers();
    let reqpath = format!("{} {}", req.method(), req.path());
    (headers.clone(), reqpath)
}

pub async fn check_and_verify<T: Serialize + Debug>(
    req: &HttpRequest,
) -> Result<AccessToken, Error> {
    let access = get_pmap().check_and_verify(&req).await.map_err(|e| {
        tracing::error!("token permission error: {:?}", e);
        let rsp = Response::<T>::internal_error(format!("Token权限验证错误: {:?}", e).as_str())
            .finished();
        error::InternalError::from_response("", rsp)
    })?;
    Ok(access)
}

pub fn create_error<T: Serialize + Debug>(e: anyhow::Error, err: &str) -> Error {
    tracing::error!("error of {}:{:?}", err, e);
    let rsp = Response::<T>::internal_error(format!("{}:{:?}", err, e).as_str()).finished();
    error::InternalError::from_response("", rsp).into()
}
