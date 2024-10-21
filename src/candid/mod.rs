use std::collections::HashMap;

use types::WrappedCandidTypeService;

/// 类型
pub mod types;

/// 解析
pub mod parse;

/// 测试
#[cfg(test)]
pub mod test;

/// 解析 candid
pub fn parse_service_candid(candid: &str) -> Result<WrappedCandidTypeService, String> {
    parse::CandidBuilder::parse_service_candid(candid)
}

/// 解析 candid
pub fn parse_methods(candid: &str) -> Result<HashMap<String, String>, String> {
    let candid = parse::CandidBuilder::parse_service_candid(candid)?;
    Ok(candid.to_methods())
}
