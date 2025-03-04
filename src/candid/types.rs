use std::collections::HashMap;

use candid::CandidType;
use serde::{Deserialize, Serialize};

/// 有的名字作为 key 需要加双引号
fn wrapped_key_word(name: &str) -> String {
    if match name {
        "bool" => true,
        "nat" => true,
        "int" => true,
        "nat8" => true,
        "nat16" => true,
        "nat32" => true,
        "nat64" => true,
        "int8" => true,
        "int16" => true,
        "int32" => true,
        "int64" => true,
        "float32" => true,
        "float64" => true,
        "null" => true,
        "text" => true,
        "principal" => true,
        "vec" => true,
        "opt" => true,
        "record" => true,
        "variant" => true,
        // "tuple" => true, // 不是关键字
        "unknown" => true,
        "empty" => true,
        "reserved" => true,
        "func" => true,
        "service" => true,
        "rec" => true, // 可能是关键字
        _ => false,
    } || name.contains(' ')
        || name.contains('-')
        || name.contains('\\')
    {
        format!("\"{}\"", name)
    } else {
        name.to_string()
    }
}

/// 有可能有名称
#[derive(Debug, Clone, CandidType, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct WrappedCandidTypeName {
    /// 有时候有名字
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl WrappedCandidTypeName {
    pub(super) fn from(name: Option<String>) -> Self {
        Self { name }
    }
}

/// 有可能有名称
#[derive(Debug, Clone, CandidType, Serialize, Deserialize, Eq, PartialEq)]
pub struct WrappedCandidTypeSubtype {
    /// 子类型
    #[serde(rename = "subtype")]
    pub subtype: Box<WrappedCandidType>,

    /// 有时候有名字
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 有可能有名称
#[derive(Debug, Clone, CandidType, Serialize, Deserialize, Eq, PartialEq)]
pub struct WrappedCandidTypeRecord {
    /// 子类型
    #[serde(rename = "subitems")]
    pub subitems: Vec<(String, WrappedCandidType)>,

    /// 有时候有名字
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl WrappedCandidTypeRecord {
    /// 文本
    pub fn to_text(&self) -> String {
        let Self { subitems, .. } = self;

        format!(
            "record {{ {} }}",
            subitems
                .iter()
                .map(|(name, subtype)| format!(
                    "{} : {}",
                    wrapped_key_word(name),
                    subtype.to_text()
                ))
                .collect::<Vec<_>>()
                .join("; ")
        )
    }
}

/// 有可能有名称
#[derive(Debug, Clone, CandidType, Serialize, Deserialize, Eq, PartialEq)]
pub struct WrappedCandidTypeVariant {
    /// 子类型
    #[serde(rename = "subitems")]
    pub subitems: Vec<(String, Option<WrappedCandidType>)>,

    /// 有时候有名字
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl WrappedCandidTypeVariant {
    /// 文本
    pub fn to_text(&self) -> String {
        let Self { subitems, .. } = self;

        format!(
            "variant {{ {} }}",
            subitems
                .iter()
                .map(|(name, subtype)| {
                    if let Some(subtype) = subtype {
                        format!("{} : {}", wrapped_key_word(name), subtype.to_text())
                    } else {
                        wrapped_key_word(name)
                    }
                })
                .collect::<Vec<_>>()
                .join("; ")
        )
    }
}

/// 有可能有名称
#[derive(Debug, Clone, CandidType, Serialize, Deserialize, Eq, PartialEq)]
pub struct WrappedCandidTypeTuple {
    /// 子类型
    #[serde(rename = "subitems")]
    pub subitems: Vec<WrappedCandidType>,

    /// 有时候有名字
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl WrappedCandidTypeTuple {
    /// 文本
    pub fn to_text(&self) -> String {
        let Self { subitems, .. } = self;

        format!(
            "record {{ {} }}",
            subitems
                .iter()
                .map(|subtype| subtype.to_text())
                .collect::<Vec<_>>()
                .join("; ")
        )
    }
}

/// 函数的注解 update 无注解
#[derive(Debug, Copy, Clone, CandidType, Serialize, Deserialize, Eq, PartialEq)]
pub enum FunctionAnnotation {
    /// 查询函数, 可以用查询机制简化消耗的 cycles
    #[serde(rename = "query")]
    Query,
    /// 用于不关心返回值的函数, 触发即忘场景
    #[serde(rename = "oneway")]
    Oneway,
}

/// 函数结构体
#[derive(Debug, Clone, CandidType, Serialize, Deserialize, Eq, PartialEq)]
pub struct WrappedCandidTypeFunction {
    /// args
    #[serde(rename = "args", skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<WrappedCandidType>,
    /// results
    #[serde(rename = "rets", skip_serializing_if = "Vec::is_empty")]
    pub rets: Vec<WrappedCandidType>,
    /// annotation update query
    #[serde(rename = "annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<FunctionAnnotation>,

    /// 有时候有名字
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl WrappedCandidTypeFunction {
    /// 文本
    pub fn to_text(&self) -> String {
        let Self {
            args,
            rets,
            annotation,
            ..
        } = self;

        format!(
            "func ({}) -> ({}){}",
            args.iter()
                .map(|t| t.to_text())
                .collect::<Vec<_>>()
                .join(", "),
            rets.iter()
                .map(|t| t.to_text())
                .collect::<Vec<_>>()
                .join(", "),
            match annotation.as_ref() {
                Some(annotation) => match annotation {
                    FunctionAnnotation::Query => " query",
                    FunctionAnnotation::Oneway => " oneway",
                },
                None => "",
            }
        )
    }
}

/// service 结构体
#[derive(Debug, Clone, CandidType, Serialize, Deserialize, Eq, PartialEq)]
pub struct WrappedCandidTypeService {
    /// args
    #[serde(rename = "args", skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<WrappedCandidType>,
    /// methods
    #[serde(rename = "methods", skip_serializing_if = "Vec::is_empty")]
    pub methods: Vec<(String, WrappedCandidTypeFunction)>,

    /// 有时候有名字
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl WrappedCandidTypeService {
    /// 文本
    pub fn to_text(&self) -> String {
        let Self { args, methods, .. } = self;

        format!(
            "service :{} {{\n{}\n}}",
            if args.is_empty() {
                "".to_string()
            } else {
                format!(
                    " ({}) ->",
                    args.iter()
                        .map(|t| t.to_text())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            },
            methods
                .iter()
                .map(|(name, func)| format!(
                    "    {} : {};",
                    wrapped_key_word(name),
                    func.to_text().trim_start_matches("func ")
                ))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }

    /// 转化为方法
    pub fn to_methods(&self) -> HashMap<String, String> {
        self.methods
            .iter()
            .map(|(method, candid)| {
                (method.to_string(), {
                    let func = candid.to_text();
                    if let Some(func) = func.strip_prefix("func ") {
                        func.to_string()
                    } else {
                        func
                    }
                })
            })
            .collect()
    }
}

/// 循环 结构体
#[derive(Debug, Clone, CandidType, Serialize, Deserialize, Eq, PartialEq)]
pub struct WrappedCandidTypeRecursion {
    ///  type
    #[serde(rename = "ty")]
    pub ty: Box<WrappedCandidType>,
    /// 分配的序号
    #[serde(rename = "id")]
    pub id: u32,

    /// 有时候有名字
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl WrappedCandidTypeRecursion {
    /// 文本
    pub fn to_text(&self) -> String {
        let Self { ty, id, .. } = self;

        format!("μrec_{}.{}", id, ty.to_text())
    }
}

/// 循环 结构体
#[derive(Debug, Clone, CandidType, Serialize, Deserialize, Eq, PartialEq)]
pub struct WrappedCandidTypeReference {
    /// 分配的序号
    #[serde(rename = "id")]
    pub id: u32,

    /// 有时候有名字
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl WrappedCandidTypeReference {
    /// 文本
    pub fn to_text(&self) -> String {
        let Self { id, .. } = self;

        format!("rec_{}", id,)
    }
}

/// 自定义的包装 Candid 类型
#[derive(Debug, Clone, CandidType, Serialize, Deserialize, Eq, PartialEq)]
pub enum WrappedCandidType {
    // 基本类型
    /// bool
    /// boolean type: true false Motoko Bool / Rust bool / JavaScript true false
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-bool
    #[serde(rename = "bool")]
    Bool(WrappedCandidTypeName),
    /// nat
    /// nature number: Motoko Nat / Rust candid:Nat or u128 / JavaScript BigInt(10000) or 10000n
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-nat
    #[serde(rename = "nat")]
    Nat(WrappedCandidTypeName),
    /// int
    /// integer number: Motoko Int / Rust candid::Int or i128 / JavaScript BigInt(-10000) or -10000n
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-int
    #[serde(rename = "int")]
    Int(WrappedCandidTypeName),
    /// nat8
    /// integer with limit bits
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-natn-and-intn
    #[serde(rename = "nat8")]
    Nat8(WrappedCandidTypeName),
    /// nat16
    /// integer with limit bits
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-natn-and-intn
    #[serde(rename = "nat16")]
    Nat16(WrappedCandidTypeName),
    /// nat32
    /// integer with limit bits
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-natn-and-intn
    #[serde(rename = "nat32")]
    Nat32(WrappedCandidTypeName),
    /// nat64
    /// integer with limit bits
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-natn-and-intn
    #[serde(rename = "nat64")]
    Nat64(WrappedCandidTypeName),
    /// int8
    /// integer with limit bits
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-natn-and-intn
    #[serde(rename = "int8")]
    Int8(WrappedCandidTypeName),
    /// int16
    /// integer with limit bits
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-natn-and-intn
    #[serde(rename = "int16")]
    Int16(WrappedCandidTypeName),
    /// int32
    /// integer with limit bits
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-natn-and-intn
    #[serde(rename = "int32")]
    Int32(WrappedCandidTypeName),
    /// int64
    /// integer with limit bits
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-natn-and-intn
    #[serde(rename = "int64")]
    Int64(WrappedCandidTypeName),
    /// float32
    /// float number: Motoko Float is 64 bits / Rust f32 f64 / JavaScript float
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-float32-and-float64
    #[serde(rename = "float32")]
    Float32(WrappedCandidTypeName),
    /// float64
    /// float number: Motoko Float is 64 bits / Rust f32 f64 / JavaScript float
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-float32-and-float64
    #[serde(rename = "float64")]
    Float64(WrappedCandidTypeName),
    /// null
    /// null type: only value is null Motoko Null / Rust None / JavaScript null
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-null
    #[serde(rename = "null")]
    Null(WrappedCandidTypeName),
    /// text
    /// text type: Motoko Text / Rust String or &str / JavaScript string
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-text
    #[serde(rename = "text")]
    Text(WrappedCandidTypeName),
    /// principal
    /// principal type: like "zwigo-aiaaa-aaaaa-qaa3a-cai" Motoko Principal / candid::Principal / JavaScript Principal.fromText("aaaaa-aa")
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-principal
    #[serde(rename = "principal")]
    Principal(WrappedCandidTypeName),
    // Blob, // 一律以 vec nat8 替代
    // 子类型
    /// vec T
    /// binary data: vec nat8 Motoko Blob / Rust Vec<u8> or &[u8] / JavaScript [1, 2, 3]
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-blob
    /// array of some type: vec {1,3} Motoko [T] / Rust Vec<T> &[T] / JavaScript Array
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-vec-t
    #[serde(rename = "vec")]
    Vec(WrappedCandidTypeSubtype),
    /// opt T
    /// option type: null opt t Motoko ?T / Rust Option<T> / [] [t]
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-opt-t
    #[serde(rename = "opt")]
    Opt(WrappedCandidTypeSubtype),
    // 多个子类型
    /// record { .. } // name=T
    /// object type: record { name="123"; } Motoko record { name: "123" } / Rust struct / JavaScript object
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-record--n--t--
    #[serde(rename = "record")]
    Record(WrappedCandidTypeRecord),
    /// variant { .. }
    /// enumerate type: variant { ok : nat; error : text } / Rust enum / JavaScript { dot: null }
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-variant--n--t--
    #[serde(rename = "variant")]
    Variant(WrappedCandidTypeVariant),
    /// tuple record { .. } // T
    /// tuple type: subitem has no name
    /// JavaScript array value
    #[serde(rename = "tuple")]
    Tuple(WrappedCandidTypeTuple),
    // 特殊类型
    /// unknown
    /// unknown type
    #[serde(rename = "unknown")]
    Unknown(WrappedCandidTypeName),
    /// empty
    /// empty type
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-empty
    #[serde(rename = "empty")]
    Empty(WrappedCandidTypeName), // 没有值的类型, 是其他类型的子类型
    /// reserved
    /// reserved type: some function arguments can be ignore
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-reserved
    #[serde(rename = "reserved")]
    Reserved(WrappedCandidTypeName), // 占位不使用的类型
    /// func
    /// func type
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-func---
    #[serde(rename = "func")]
    Func(WrappedCandidTypeFunction),
    /// service
    /// service type: canister's api
    /// https://internetcomputer.org/docs/current/references/candid-ref/#type-service-
    #[serde(rename = "service")]
    Service(WrappedCandidTypeService),
    /// rec
    /// object type: some subtype or subitem is recursion
    #[serde(rename = "rec")]
    Rec(WrappedCandidTypeRecursion), // 循环类型中的主类型
    /// ref
    #[serde(rename = "ref")]
    Reference(WrappedCandidTypeReference), // 循环类型中的引用类型
}

impl WrappedCandidType {
    /// 文本
    pub fn to_text(&self) -> String {
        match self {
            Self::Bool(_) => String::from("bool"),
            Self::Nat(_) => String::from("nat"),
            Self::Int(_) => String::from("int"),
            Self::Nat8(_) => String::from("nat8"),
            Self::Nat16(_) => String::from("nat16"),
            Self::Nat32(_) => String::from("nat32"),
            Self::Nat64(_) => String::from("nat64"),
            Self::Int8(_) => String::from("int8"),
            Self::Int16(_) => String::from("int16"),
            Self::Int32(_) => String::from("int32"),
            Self::Int64(_) => String::from("int64"),
            Self::Float32(_) => String::from("float32"),
            Self::Float64(_) => String::from("float64"),
            Self::Null(_) => String::from("null"),
            Self::Text(_) => String::from("text"),
            Self::Principal(_) => String::from("principal"),
            Self::Vec(WrappedCandidTypeSubtype { subtype, .. }) => {
                format!("vec {}", subtype.to_text())
            }
            Self::Opt(WrappedCandidTypeSubtype { subtype, .. }) => {
                format!("opt {}", subtype.to_text())
            }
            Self::Record(record) => record.to_text(),
            Self::Variant(variant) => variant.to_text(),
            Self::Tuple(tuple) => tuple.to_text(),
            Self::Unknown(_) => String::from("unknown"),
            Self::Empty(_) => String::from("empty"),
            Self::Reserved(_) => String::from("reserved"),
            Self::Func(func) => func.to_text(),
            Self::Service(service) => service.to_text(),
            Self::Rec(recursion) => recursion.to_text(),
            Self::Reference(reference) => reference.to_text(),
        }
    }
}
