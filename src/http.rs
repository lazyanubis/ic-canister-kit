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

// http 请求的结构体
#[derive(CandidType, Deserialize)]
pub struct CustomHttpRequest {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

// http 响应的结构体
#[derive(CandidType)]
pub struct CustomHttpResponse<'a> {
    pub status_code: u16,
    pub headers: HashMap<&'a str, Cow<'a, str>>,
    pub body: Cow<'a, [u8]>,
}
