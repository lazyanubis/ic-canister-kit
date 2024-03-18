use std::collections::HashMap;

use candid::CandidType;
use serde::{Deserialize, Serialize};

/// 自定义的包装 Candid 类型
#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum WrappedCandidType {
    // 基本类型
    /// bool
    Bool,
    /// nat
    Nat,
    /// int
    Int,
    /// nat8
    Nat8,
    /// nat16
    Nat16,
    /// nat32
    Nat32,
    /// nat32
    Nat64,
    /// int8
    Int8,
    /// int16
    Int16,
    /// int32
    Int32,
    /// int64
    Int64,
    /// float32
    Float32,
    /// float64
    Float64,
    /// null
    Null,
    /// text
    Text,
    /// principal
    Principal,
    // Blob, // 一律以 vec nat8 替代
    // 子类型
    /// vec T
    Vec(Box<WrappedCandidType>),
    /// opt T
    Opt(Box<WrappedCandidType>),
    // 多个子类型
    /// record { .. } // name=T
    Record(Vec<(String, WrappedCandidType)>),
    /// variant { .. }
    Variant(Vec<(String, Option<WrappedCandidType>)>),
    /// tuple record { .. } // T
    Tuple(Vec<WrappedCandidType>),
    // 特殊类型
    /// unknown
    Unknown,
    /// empty
    Empty, // 没有值的类型, 是其他类型的子类型
    /// reserved
    Reserved, // 占位不使用的类型
    /// func
    Func {
        /// args
        args: Vec<WrappedCandidType>,
        /// results
        results: Vec<WrappedCandidType>,
        /// annotation update query
        annotation: FunctionAnnotation,
    },
    /// service
    Service {
        /// init args
        args: Vec<WrappedCandidType>,
        /// methods
        methods: Vec<(String, WrappedCandidType)>, // 子类型一定是函数
    },
    /// rec
    Rec(Box<WrappedCandidType>, u32), // 循环类型中的主类型
    /// ref
    Reference(u32), // 循环类型中的引用类型
}

/// 函数的注解
#[derive(CandidType, Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum FunctionAnnotation {
    /// 正常的修改函数
    None,
    /// 查询函数, 可以用查询机制简化消耗的 cycles
    Query,
    /// 用于不关心返回值的函数, 触发即忘场景
    Oneway,
}

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
        || name.contains('\\')
    {
        format!("\"{}\"", name)
    } else {
        name.to_string()
    }
}

impl WrappedCandidType {
    /// 文本
    pub fn to_text(&self) -> String {
        match self {
            Self::Bool => String::from("bool"),
            Self::Nat => String::from("nat"),
            Self::Int => String::from("int"),
            Self::Nat8 => String::from("nat8"),
            Self::Nat16 => String::from("nat16"),
            Self::Nat32 => String::from("nat32"),
            Self::Nat64 => String::from("nat64"),
            Self::Int8 => String::from("int8"),
            Self::Int16 => String::from("int16"),
            Self::Int32 => String::from("int32"),
            Self::Int64 => String::from("int64"),
            Self::Float32 => String::from("float32"),
            Self::Float64 => String::from("float64"),
            Self::Null => String::from("null"),
            Self::Text => String::from("text"),
            Self::Principal => String::from("principal"),
            Self::Vec(subtype) => format!("vec {}", subtype.to_text()),
            Self::Opt(subtype) => format!("opt {}", subtype.to_text()),
            Self::Record(subitems) => format!(
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
            ),
            Self::Variant(subitems) => format!(
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
            ),
            Self::Tuple(subitems) => format!(
                "variant {{ {} }}",
                subitems
                    .iter()
                    .map(|subtype| subtype.to_text())
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
            Self::Unknown => String::from("unknown"),
            Self::Empty => String::from("empty"),
            Self::Reserved => String::from("reserved"),
            Self::Func {
                args,
                results,
                annotation,
            } => format!(
                "func ({}) -> ({}){}",
                args.iter()
                    .map(|t| t.to_text())
                    .collect::<Vec<_>>()
                    .join(", "),
                results
                    .iter()
                    .map(|t| t.to_text())
                    .collect::<Vec<_>>()
                    .join(", "),
                match annotation {
                    FunctionAnnotation::None => "",
                    FunctionAnnotation::Query => " query",
                    FunctionAnnotation::Oneway => " oneway",
                }
            ),
            Self::Service { args, methods } => format!(
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
                        func.to_text()
                    ))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
            Self::Rec(subtype, id) => format!("μrec_{}.{}", id, subtype.to_text()),
            Self::Reference(id) => format!("rec_{}", id),
        }
    }

    /// 转化为方法
    pub fn to_methods(&self) -> Result<HashMap<String, String>, String> {
        match self {
            Self::Service { methods, .. } => Ok(methods
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
                .collect()),
            _ => Err("must be service".into()),
        }
    }
}
