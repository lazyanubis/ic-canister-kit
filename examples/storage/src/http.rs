use std::{borrow::Cow, collections::HashMap};

use percent_encoding::percent_decode_str;
use regex::Regex;

use ic_canister_kit::http::MAX_RESPONSE_LENGTH;

use crate::explore::explore;
use crate::stable::State;
use crate::types::*;

// https://github.com/dfinity/examples/blob/8b01d548d8548a9d4558a7a1dbb49234d02d7d03/motoko/http_counter/src/main.mo

// #[ic_cdk::update]
// fn http_request_update(request: CustomHttpRequest) -> CustomHttpResponse {
//     todo!()
// }

// 请求数据
#[ic_cdk::query]
fn http_request(request: CustomHttpRequest) -> CustomHttpResponse {
    crate::stable::with_state(|state| inner_http_request(state, request))
}

#[inline]
fn inner_http_request(state: &State, req: CustomHttpRequest) -> CustomHttpResponse {
    let mut split_url = req.url.split('?');
    let request_headers = req.headers;

    let path = split_url.next().unwrap_or("/"); // 分割出 url，默认是 /
    let path = percent_decode_str(path)
        .decode_utf8()
        .unwrap_or(Cow::Borrowed(path));
    let params = split_url.next().unwrap_or(""); // 请求参数
    let params = percent_decode_str(params)
        .decode_utf8()
        .unwrap_or(Cow::Borrowed(params));

    // ic_cdk::println!("============== path: {} -> {}", req.url, path);
    // for (key, value) in request_headers.iter() {
    //     ic_cdk::println!("header: {}: {}", key, value);
    // }

    let mut code = 200; // 响应码默认是 200
    let mut headers: HashMap<String, String> = HashMap::new();
    let body: Vec<u8>;
    let mut streaming_strategy: Option<StreamingStrategy> = None;

    if path == "/" {
        body = explore(&mut headers, state); // 主页内容
    } else {
        // 根据路径找文件
        let file = state.business_assets_files().get(&path.to_string());
        if let Some(file) = file {
            let asset = state.business_assets_assets().get(&file.hash);
            if let Some(asset) = asset {
                let (_body, _streaming_strategy): (Vec<u8>, Option<StreamingStrategy>) = toast(
                    &path,
                    &params,
                    &request_headers,
                    file,
                    asset,
                    &mut code,
                    &mut headers,
                ); // 有对应的文件
                body = _body;
                streaming_strategy = _streaming_strategy;
            } else {
                body = not_found(&mut code, &mut headers);
            }
        } else {
            body = not_found(&mut code, &mut headers);
        }
    }

    CustomHttpResponse {
        status_code: code,
        headers: headers
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        body,
        streaming_strategy,
        upgrade: None,
    }
}

fn toast(
    path: &str,
    params: &str,
    request_headers: &HashMap<String, String>,
    file: &AssetFile,
    asset: AssetData,
    code: &mut u16,
    headers: &mut HashMap<String, String>,
) -> (Vec<u8>, Option<StreamingStrategy>) {
    // 1. 设置 header
    let (offset, offset_end, streaming_strategy) = set_headers(
        path,
        params,
        request_headers,
        file,
        asset.size as usize,
        code,
        headers,
    );

    // 2. 返回指定的内容
    (
        (asset.data[offset..offset_end]).to_vec(),
        streaming_strategy,
    )
}

fn set_headers(
    path: &str,
    params: &str,
    request_headers: &HashMap<String, String>,
    file: &AssetFile,
    size: usize,
    code: &mut u16,
    headers: &mut HashMap<String, String>,
) -> (usize, usize, Option<StreamingStrategy>) {
    // let mut gzip = false;
    // let mut content_type = "";
    // for (key, value) in file.headers.iter() {
    //     if &key.to_lowercase() == "content-type" {
    //         content_type = value;
    //     }
    //     if &key.to_lowercase() == "content-encoding" && value == "gzip" {
    //         gzip = true;
    //     }
    // }

    // 文件名下载
    if let Ok(reg) = Regex::new(r"attachment=(.*\..*)?(&.*)?$") {
        for cap in reg.captures_iter(params) {
            let mut file_name = cap
                .get(1)
                .map(|m| &params[m.start()..m.end()])
                .unwrap_or("");
            if file_name.is_empty() {
                let s = file.path.split('/');
                for name in s {
                    file_name = name;
                }
            }
            if !file_name.is_empty() {
                headers.insert(
                    "Content-Disposition".into(),
                    format!("attachment; filename=\"{}\"", file_name),
                ); // 下载文件名
            }
        }
    }

    // ! 这个时间库无法编译
    // use chrono::{TimeZone, Utc};
    // let modified = Utc.timestamp_nanos(file.modified as i64);
    // headers.insert("Last-Modified", modified.to_rfc2822().into());

    // 额外增加的请求头
    headers.insert("Accept-Ranges".into(), "bytes".into()); // 支持范围请求
    headers.insert("ETag".into(), file.hash.hex()); // 缓存标识

    // 访问控制
    headers.insert("Access-Control-Allow-Origin".into(), "*".into());
    headers.insert(
        "Access-Control-Allow-Methods".into(),
        "HEAD, GET, POST, OPTIONS".into(),
    );
    headers.insert(
        "Access-Control-Allow-Headers".into(),
        "Origin,Access-Control-Request-Headers,Access-Control-Allow-Headers,DNT,X-Requested-With,X-Mx-ReqToken,Keep-Alive,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Accept,Connection,Cook ie,X-XSRF-TOKEN,X-CSRF-TOKEN,Authorization".into(),
    );
    headers.insert(
        "Access-Control-Expose-Headers".into(),
        "Accept-Ranges,Content-Length,Content-Range,Transfer-Encoding,Connection,Cache-Control,Content-Disposition"
            .into(),
    );
    headers.insert("Access-Control-Max-Age".into(), "86400".into());

    // Range 设置
    let mut start: usize = 0;
    let mut end: usize = size;
    if let Some(range) = {
        let mut range = None;
        for (key, value) in request_headers.iter() {
            if &key.to_lowercase() == "range" {
                range = Some(value.trim());
                break;
            }
        }
        range
    } {
        // bytes=start-end
        if let Some(range) = range.strip_prefix("bytes=") {
            let mut ranges = range.split('-');
            let s = ranges.next();
            let e = ranges.next();
            if let Some(s) = s {
                let s: usize = s.parse().unwrap_or(0);
                if s < size {
                    start = s
                };
            }
            if let Some(e) = e {
                let e: usize = e.parse().unwrap_or(size - 1);
                if start < e && e < size {
                    end = e + 1
                };
            }
        }
    }

    // 独立的请求头内容
    for (key, value) in file.headers.iter() {
        headers.insert(key.into(), value.into());
    }
    // ic_cdk::println!("---------- {} {} ----------", start, end);
    // 如果过长, 需要阶段显示
    let mut streaming_end = end;
    let mut streaming_strategy: Option<StreamingStrategy> = None;
    let range = streaming_end - start;
    if MAX_RESPONSE_LENGTH < range && start + MAX_RESPONSE_LENGTH < end {
        // 响应的范围太大了, 缩短为最大长度, 此时应当开启流式响应
        streaming_end = start + MAX_RESPONSE_LENGTH;
        streaming_strategy = Some(StreamingStrategy::Callback {
            callback: HttpRequestStreamingCallback::new(
                ic_cdk::id(),
                "http_request_streaming_callback".to_string(),
            ),
            token: StreamingCallbackToken {
                path: path.to_string(),
                params: params.to_string(),
                headers: request_headers.clone(),
                start: streaming_end as u64,
                end: end as u64,
            },
        });
        headers.insert("Transfer-Encoding".into(), "chunked".into());
        headers.insert("Connection".into(), "keep-alive".into()); // 保持链接
    }
    // Content-Range: bytes 0-499/10000
    headers.insert(
        "Content-Range".into(),
        format!("bytes {}-{}/{}", start, end - 1, size), // 流式响应也要设置正确的内容范围
    );
    // ! 长度设置了会出错
    // headers.insert("Content-Length", format!("{}", end - start).into()); // ? 这个应该是本次返回的长度

    // 如果是视频可能需要返回其他的
    *code = 200;
    if end < size {
        *code = 206; // 还有内容没给
    }

    (start, streaming_end, streaming_strategy)
}

// 找不到对应的文件
fn not_found(code: &mut u16, headers: &mut HashMap<String, String>) -> Vec<u8> {
    *code = 404;

    headers.insert("Content-Type".into(), "text/plain".into());

    b"Not found"[..].into()
}

// 流式响应回调
#[ic_cdk::query]
fn http_streaming(
    StreamingCallbackToken {
        path,
        params,
        headers,
        start,
        end,
    }: StreamingCallbackToken,
) -> StreamingCallbackHttpResponse {
    // ic_cdk::println!(
    //     "http_request_streaming_callback: {:?} {:?} {:?} {:?} {:?}",
    //     path,
    //     params,
    //     headers,
    //     start,
    //     end,
    // );
    if start == end {
        // 首尾相等, 说明没有数据了
        return StreamingCallbackHttpResponse {
            body: vec![],
            token: None,
        };
    }
    crate::stable::with_state(|state| {
        let file = state.business_assets_files().get(&path);
        if let Some(file) = file {
            let asset = state.business_assets_assets().get(&file.hash);
            if let Some(asset) = asset {
                // 如果过长, 需要阶段显示
                let start = start as usize;
                let end = end as usize;
                let mut streaming_end = end;
                let range = streaming_end - start;
                if MAX_RESPONSE_LENGTH < range && start + MAX_RESPONSE_LENGTH < end {
                    // 响应的范围太大了, 缩短为最大长度, 此时应当继续流式响应
                    streaming_end = start + MAX_RESPONSE_LENGTH;
                }
                return StreamingCallbackHttpResponse {
                    body: (asset.data[start..streaming_end]).to_vec(),
                    token: Some(StreamingCallbackToken {
                        path: path.to_string(),
                        params: params.to_string(),
                        headers: headers.clone(),
                        start: streaming_end as u64, // 继续
                        end: end as u64,
                    }),
                };
            }
        }
        StreamingCallbackHttpResponse {
            body: vec![],
            token: None,
        }
    })
}