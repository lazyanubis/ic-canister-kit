use std::collections::HashMap;

use error::ParsedCandidError;
use types::WrappedCandidTypeService;

/// 类型
pub mod types;

/// 错误
pub mod error;

/// 解析
pub mod parse;

/// 测试
#[cfg(test)]
pub mod test;

/// 解析 candid
pub fn parse_service_candid(candid: &str) -> Result<WrappedCandidTypeService, ParsedCandidError> {
    parse::CandidBuilder::parse_service_candid(candid)
}

/// 解析 candid
pub fn parse_methods(candid: &str) -> Result<HashMap<String, String>, ParsedCandidError> {
    let candid = parse::CandidBuilder::parse_service_candid(candid)?;
    Ok(candid.to_methods())
}
