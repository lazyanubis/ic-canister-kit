use candid::CandidType;
use serde::{Deserialize, Serialize};

/// 解析的错误信息
#[derive(Debug, Clone, CandidType, Serialize, Deserialize, Eq, PartialEq)]
pub enum ParsedCandidError {
    /// 一般错误
    Common(String),
    /// 循环引用错误
    EmptyRecRecords,
    /// 循环引用类型重复
    RecRecordRepeated(String),
    /// 循环引用类型不存在
    RecRecordNotExist(String),
    /// 找不到对应的类型
    MissingType(String),
    /// 错误的注释
    WrongComment(String),
    /// 解析错误
    ParsedError(String),
}
