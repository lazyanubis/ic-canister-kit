use std::collections::HashMap;

use candid::CandidType;
use serde::{Deserialize, Serialize};

pub use ic_management_canister_types::{
    HttpHeader, HttpMethod, HttpRequestArgs, HttpRequestResult, TransformArgs, TransformContext,
};

use crate::{canister::types::CanisterCallError, identity::CanisterId};

// ========================= HTTP 相关结构体 =========================

/// 最长的响应体 3M, 留点空间给其他数据 此处大概 2.9375 MB
pub const MAX_RESPONSE_LENGTH: usize = 1024 * 1024 * 3 - 1024 * 64;

/// http 请求的结构体
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct CustomHttpRequest {
    /// 请求路径
    pub url: String,

    /// 请求类型
    pub method: String,

    /// 请求头
    pub headers: HashMap<String, String>,

    /// 请求体
    pub body: Vec<u8>,
}

/// 流式响应的传递 token
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct StreamingCallbackToken {
    /// url 定位哪个请求
    pub path: String,

    /// 继续请求需要使用的标识
    pub token: HashMap<String, String>,
}

/// 流式响应的响应体
#[derive(CandidType, Serialize, Deserialize, Debug, Clone, Default)]
pub struct StreamingCallbackHttpResponse {
    ///  响应体
    pub body: Vec<u8>,

    ///  是否要继续流式响应
    pub token: Option<StreamingCallbackToken>,
}

impl StreamingCallbackHttpResponse {
    /// 空响应体
    pub fn empty() -> Self {
        Self::default()
    }
}

/// 定义流回调函数
#[allow(missing_docs)]
mod callback {
    use super::*;

    candid::define_function!(pub HttpRequestStreamingCallback : (StreamingCallbackToken) -> (StreamingCallbackHttpResponse) query);
}
pub use callback::HttpRequestStreamingCallback;

/// 流式响应的启动策略
#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum StreamingStrategy {
    /// 回调函数
    Callback {
        /// 回调方法
        callback: HttpRequestStreamingCallback, // 回调方法

        /// 回调参数token，用于识别哪个请求的回调
        token: StreamingCallbackToken,
    },
}

/// http 响应的结构体
#[derive(CandidType, Debug, Clone)]
pub struct CustomHttpResponse {
    /// 响应状态码
    pub status_code: u16,

    /// 响应头
    pub headers: HashMap<String, String>,

    /// 响应体
    pub body: Vec<u8>,

    /// 如果有额外的数据需要通过流的方式继续传输 每个 http 请求最大只能支持 3M 的响应数据，因此太大的话需要采用此种方式
    pub streaming_strategy: Option<StreamingStrategy>, // 如果需要使用流式响应

    /// 升级
    pub upgrade: Option<bool>,
}

// ====================== http 请求 ======================

/// 可以调用罐子自身的 query 方法解析响应体
pub fn http_transform(response: TransformArgs) -> HttpRequestResult {
    let mut t = response.response;
    t.headers = vec![];
    t
}

// ====================== 对外发起 http 请求 ======================

/// http 请求
pub async fn do_http_request(
    arg: HttpRequestArgs,
    cycles: u128,
) -> super::types::CanisterCallResult<HttpRequestResult> {
    let cost = ic_cdk::management_canister::cost_http_request(&arg);
    if cycles < cost {
        return Err(CanisterCallError {
            canister_id: CanisterId::anonymous(),
            method: "ic#http_request".to_string(),
            message: "Insufficient cycles".to_string(),
        });
    }
    ic_cdk::management_canister::http_request(&arg)
        .await
        .map_err(|err| CanisterCallError {
            canister_id: CanisterId::anonymous(),
            method: "ic#http_request".to_string(),
            message: err.to_string(),
        })
}

/// 带有转换函数的 http 请求
#[allow(clippy::future_not_send)]
pub async fn do_http_request_with_closure(
    arg: HttpRequestArgs,
    cycles: u128,
    transform_func: impl FnOnce(HttpRequestResult) -> HttpRequestResult + 'static,
) -> super::types::CanisterCallResult<HttpRequestResult> {
    let cost = ic_cdk::management_canister::cost_http_request(&arg);
    if cycles < cost {
        return Err(CanisterCallError {
            canister_id: CanisterId::anonymous(),
            method: "ic#http_request".to_string(),
            message: "Insufficient cycles".to_string(),
        });
    }
    ic_cdk::management_canister::http_request_with_closure(&arg, transform_func)
        .await
        .map_err(|err| CanisterCallError {
            canister_id: CanisterId::anonymous(),
            method: "ic#http_request".to_string(),
            message: err.to_string(),
        })
}
