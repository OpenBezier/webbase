use actix_web::{http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct NoneBodyData {}

/// Client使用的通用格式返回数据类型，message可为String或泛型T
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientRsp {
    pub status: bool,
    pub code: u16,
    pub message: serde_json::Value,
}

/// Client使用的正确返回类型数据格式定义
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OkData<T: Serialize + Debug> {
    pub status: bool,
    pub code: u16,
    pub message: T,
}

/// Client使用的错误返回类型数据格式定义
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrData {
    pub status: bool,
    pub code: u16,
    pub message: String,
}

/// 服务端使用的回复响应泛型数据类型
#[derive(Debug)]
pub struct Response<T: Serialize + Debug> {
    pub code: StatusCode,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize + Debug> Response<T> {
    #[allow(dead_code)]
    pub fn success(data: T) -> Self {
        Self {
            code: StatusCode::OK,
            message: "".to_string(),
            data: Some(data),
        }
    }

    #[allow(dead_code)]
    pub fn internal_error(message: &str) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.to_string(),
            data: None,
        }
    }

    #[allow(dead_code)]
    pub fn bad_request(message: &str) -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: message.to_string(),
            data: None,
        }
    }

    #[allow(dead_code)]
    pub fn forbidden(message: &str) -> Self {
        Self {
            code: StatusCode::FORBIDDEN,
            message: message.to_string(),
            data: None,
        }
    }

    #[allow(dead_code)]
    pub fn not_acceptable(message: &str) -> Self {
        Self {
            code: StatusCode::NOT_ACCEPTABLE,
            message: message.to_string(),
            data: None,
        }
    }

    #[allow(dead_code)]
    pub fn unauthorized(message: &str) -> Self {
        Self {
            code: StatusCode::UNAUTHORIZED,
            message: message.to_string(),
            data: None,
        }
    }

    #[allow(dead_code)]
    pub fn nofound(message: &str) -> Self {
        Self {
            code: StatusCode::NOT_FOUND,
            message: message.to_string(),
            data: None,
        }
    }

    pub fn finished(&self) -> HttpResponse {
        if self.code.is_success() {
            HttpResponse::Ok().json(serde_json::json!(OkData {
                status: true,
                code: self.code.as_u16(),
                message: self.data.as_ref().unwrap(),
            }))
        } else {
            HttpResponse::Ok().json(serde_json::json!(ErrData {
                status: false,
                code: self.code.as_u16(),
                message: self.message.clone(),
            }))
        }
    }

    pub fn with_response(&self, resp: HttpResponse) -> HttpResponse {
        resp
    }
}

impl<T: Serialize + Debug> std::fmt::Display for Response<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ code: {}, message: {} }}", self.code, self.message)
    }
}

impl<T: Serialize + Debug> actix_web::error::ResponseError for Response<T> {
    fn status_code(&self) -> StatusCode {
        self.code.clone()
    }

    fn error_response(&self) -> HttpResponse {
        // HttpResponse::build(self.code).json(serde_json::json!(ErrData {
        //     status: false,
        //     code: self.code.as_u16(),
        //     message: self.message.clone(),
        // }))
        HttpResponse::Ok().json(serde_json::json!(ErrData {
            status: false,
            code: self.code.as_u16(),
            message: self.message.clone(),
        }))
    }
}
