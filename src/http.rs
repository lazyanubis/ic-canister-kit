use std::{borrow::Cow, collections::HashMap};

use candid::CandidType;
use serde::Deserialize;

pub use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};

pub fn transform(response: TransformArgs) -> HttpResponse {
    let mut t = response.response;
    t.headers = vec![];
    t
}

// ====================== http 请求 ======================

// 最长的响应体 大概 2.9375 MB, 留点空间给其他数据
pub const MAX_RESPONSE_LENGTH: usize = 3145728 - 1024 * 64;

// http 请求的结构体
#[derive(CandidType, Deserialize)]
pub struct CustomHttpRequest {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

// 流式响应的传递 token
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StreamingCallbackToken {
    pub path: String,                     // 定位哪个请求
    pub params: String,                   // 参数属性
    pub headers: HashMap<String, String>, // 请求头
    pub start: u64,                       // 数据起始位置
    pub end: u64,                         // 数据结束位置
}

// 流式响应的响应体
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StreamingCallbackHttpResponse {
    pub body: Vec<u8>,                         // 响应体
    pub token: Option<StreamingCallbackToken>, // 是否要继续流式响应
}

candid::define_function!(pub HttpRequestStreamingCallback : (StreamingCallbackToken) -> (StreamingCallbackHttpResponse) query);

// 流式响应的启动策略
#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum StreamingStrategy {
    Callback {
        callback: HttpRequestStreamingCallback, // 回调方法
        token: StreamingCallbackToken,
    },
}

// http 响应的结构体
#[derive(CandidType)]
pub struct CustomHttpResponse<'a> {
    pub status_code: u16,
    pub headers: HashMap<&'a str, Cow<'a, str>>,
    pub body: Cow<'a, [u8]>,
    pub streaming_strategy: Option<StreamingStrategy>, // 如果需要使用流式响应
}
