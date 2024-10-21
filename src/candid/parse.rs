use std::collections::HashMap;

use super::{error::ParsedCandidError, types::*};

#[derive(Debug, Clone)]
struct InnerCandidTypeFunction {
    args: Vec<InnerCandidType>,
    rets: Vec<InnerCandidType>,
    annotation: Option<FunctionAnnotation>,
}

#[derive(Debug, Clone)]
struct InnerCandidTypeService {
    args: Vec<InnerCandidType>,
    methods: Vec<(String, InnerCandidTypeFunction)>,
}

#[derive(Debug, Clone)]
enum InnerCandidType {
    Bool,
    Nat,
    Int,
    Nat8,
    Nat16,
    Nat32,
    Nat64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    Null,
    Text,
    Principal,
    Blob,
    Vec(Box<InnerCandidType>),
    Opt(Box<InnerCandidType>),
    Record(Vec<(String, InnerCandidType)>),
    Variant(Vec<(String, Option<InnerCandidType>)>),
    Tuple(Vec<InnerCandidType>),
    Unknown,
    Empty,
    Reserved,
    Func(InnerCandidTypeFunction),
    Service(InnerCandidTypeService),
    Reference(String), // 循环类型中的引用类型
}

#[derive(Debug, Clone)]
struct RecRecord {
    names: Vec<String>,
    records: HashMap<String, u32>,
}
impl RecRecord {
    fn new() -> Self {
        Self {
            names: Vec::new(),
            records: HashMap::new(),
        }
    }
    fn contains(&self, name: &String) -> bool {
        self.names.contains(name)
    }
    fn push(&mut self, name: String) {
        self.names.push(name)
    }
    fn pop(&mut self) -> Result<String, ParsedCandidError> {
        self.names.pop().ok_or(ParsedCandidError::EmptyRecRecords)
    }
    fn id(&self, name: &String) -> Option<u32> {
        self.records.get(name).copied()
    }
    fn insert(&mut self, name: String) -> Result<u32, ParsedCandidError> {
        if self.records.contains_key(&name) {
            return Err(ParsedCandidError::RecRecordRepeated(name));
        }
        let id = self.records.len() as u32;
        self.records.insert(name, id);
        Ok(id)
    }
    fn remove(&mut self, name: &String) -> Result<u32, ParsedCandidError> {
        self.records
            .remove(name)
            .ok_or_else(|| ParsedCandidError::RecRecordNotExist(name.clone()))
    }
}

#[derive(Debug, Clone)]
pub(super) struct CandidBuilder {
    chars: Vec<char>,
    length: usize,
    cursor: usize,
    inner_types: HashMap<String, InnerCandidType>, // 临时类型
    wrapped_types: HashMap<String, WrappedCandidType>, // 已经确定的类型, 循环类型不确定, 不应放入
    service: Option<WrappedCandidTypeService>,
}

impl CandidBuilder {
    fn new(candid: &str) -> Self {
        // 需要移除注释
        // /* */ 类型的注释等代码里面解决
        let candid = candid
            .split('\n')
            .map(|s| s.to_string())
            .filter(|s| !s.trim().starts_with("//")) // 不要以 // 开头的整行注释
            .map(|s| s.split("//").next().unwrap_or_default().to_string()) // 不要以 // 开头的行尾注释
            .collect::<Vec<_>>()
            .join("\n");

        let chars = candid.chars().collect::<Vec<_>>();
        let length = chars.len();

        CandidBuilder {
            chars,
            length,
            cursor: 0,
            inner_types: HashMap::new(),
            wrapped_types: HashMap::new(),
            service: None,
        }
    }

    // 是否还有指定长度
    #[inline]
    fn has(&self, n: usize) -> bool {
        self.cursor + n <= self.length
    }
    // 剩下的字符串
    fn remain(&self, cursor: Option<usize>) -> String {
        let cursor = cursor.unwrap_or(self.cursor);
        let chars = &self.chars[cursor..];
        chars.iter().copied().collect()
    }
    // 下个字段是否是指定的字符序列
    fn is_next(&self, types: &[char]) -> bool {
        if !self.has(types.len()) {
            return false;
        }
        for (i, c) in types.iter().enumerate() {
            if *c == self.chars[self.cursor + i] {
                continue;
            }
            return false;
        }
        true
    }
    // 排序
    fn sort_list<T>(&self, mut list: Vec<(String, T)>) -> Vec<(String, T)> {
        list.sort_by(|a, b| a.0.cmp(&b.0));
        list
    }

    // 跳过指定字符
    fn inner_trim_start_blank_or_chars(&mut self, chars: &[char]) {
        while self.has(1) {
            let current = self.chars[self.cursor];
            if current == ' ' || current == '\t' {
                self.cursor += 1
            } else if chars.contains(&current) {
                self.cursor += 1;
            } else {
                break;
            }
        }
    }

    // 跳过无效字符 和 注释
    fn trim_start_blank_or_chars(&mut self, chars: &[char]) -> Result<(), ParsedCandidError> {
        self.inner_trim_start_blank_or_chars(chars);
        loop {
            if self.is_next(&['/', '*']) {
                self.cursor += 2;
                while !self.is_next(&['*', '/']) {
                    if !self.has(1) {
                        return Err(ParsedCandidError::WrongComment(
                            "Can not find */ for end comment".into(),
                        ));
                    }
                    self.cursor += 1;
                }
                self.cursor += 2;
                self.inner_trim_start_blank_or_chars(chars);
                self.trim_start_blank_or_newline_semicolon()?;
                self.inner_trim_start_blank_or_chars(chars);
                continue;
            }
            break;
        }
        self.inner_trim_start_blank_or_chars(chars);
        Ok(())
    }

    fn trim_start_blank(&mut self) -> Result<(), ParsedCandidError> {
        self.trim_start_blank_or_chars(&[])
    }
    fn trim_start_blank_or_semicolon(&mut self) -> Result<(), ParsedCandidError> {
        self.trim_start_blank_or_chars(&[';'])
    }
    fn trim_start_blank_or_colon(&mut self) -> Result<(), ParsedCandidError> {
        self.trim_start_blank_or_chars(&[':'])
    }
    fn trim_start_blank_or_comma(&mut self) -> Result<(), ParsedCandidError> {
        self.trim_start_blank_or_chars(&[','])
    }
    fn trim_start_blank_or_newline_semicolon(&mut self) -> Result<(), ParsedCandidError> {
        self.trim_start_blank_or_chars(&['\r', '\n', ';'])
    }
    fn trim_start_blank_or_newline(&mut self) -> Result<(), ParsedCandidError> {
        self.trim_start_blank_or_chars(&['\r', '\n'])
    }

    // 读取下一个标识符
    fn read_name(&mut self) -> Result<String, ParsedCandidError> {
        self.trim_start_blank()?;
        let cursor = self.cursor;
        let mark = self.is_next(&['"']); // 是否双引号标识
        if mark {
            self.cursor += 1;
        }
        let mut name = String::new();
        loop {
            if !self.has(1) {
                break;
            }
            let current = self.chars[self.cursor];
            if mark {
                match current {
                    '\\' => {
                        if !self.has(2) {
                            return Err(ParsedCandidError::ParsedError(format!(
                                "1. can not read string at: {}",
                                self.remain(Some(cursor))
                            )));
                        }
                        let current2 = self.chars[self.cursor + 1];
                        self.cursor += 2;
                        name.push(current2); // 下一个字符一定加入
                    }
                    '"' => {
                        self.cursor += 1; // 结束
                        break;
                    }
                    _ => {
                        self.cursor += 1;
                        name.push(current);
                    }
                }
            } else {
                match current {
                    ' ' | ':' | ',' | ';' | '{' | '}' | '(' | ')' | '\t' | '\n' | '\r' => break,
                    _ => {
                        self.cursor += 1;
                        name.push(current)
                    }
                }
            }
        }
        if name.is_empty() {
            return Err(ParsedCandidError::ParsedError(format!(
                "2. can not read string at: {}",
                self.remain(Some(cursor))
            )));
        }
        Ok(name)
    }
    // 检查并移除 'type '
    fn trim_type(&mut self) -> Result<bool, ParsedCandidError> {
        self.trim_start_blank_or_newline_semicolon()?;
        if !self.is_next(&['t', 'y', 'p', 'e', ' ']) {
            return Ok(false);
        }
        self.cursor += 5;
        Ok(true)
    }
    // 检查并移除 'query'
    fn trim_query(&mut self) -> Result<bool, ParsedCandidError> {
        self.trim_start_blank_or_newline()?;
        if !self.is_next(&['q', 'u', 'e', 'r', 'y']) {
            return Ok(false);
        }
        self.cursor += 5;
        Ok(true)
    }
    // 检查并移除 'oneway'
    fn trim_oneway(&mut self) -> Result<bool, ParsedCandidError> {
        self.trim_start_blank_or_newline()?;
        if !self.is_next(&['o', 'n', 'e', 'w', 'a', 'y']) {
            return Ok(false);
        }
        self.cursor += 6;
        Ok(true)
    }
    // 检查并移除 'service'
    fn trim_service(&mut self) -> Result<bool, ParsedCandidError> {
        self.trim_start_blank_or_semicolon()?;
        if !self.is_next(&['s', 'e', 'r', 'v', 'i', 'c', 'e']) {
            return Ok(false);
        }
        self.cursor += 7;
        Ok(true)
    }
    // 检查并移除指定字符
    fn remove_char(&mut self, ch: char) -> Result<(), ParsedCandidError> {
        self.trim_start_blank_or_newline()?;
        if !self.has(1) || self.chars[self.cursor] != ch {
            return Err(ParsedCandidError::ParsedError(format!(
                "next char must be {} at {}",
                ch,
                self.remain(None)
            )));
        }
        self.cursor += 1;
        Ok(())
    }
    // 插入已知的类型
    fn known_type(&mut self, name: String, candid_type: WrappedCandidType) {
        self.wrapped_types.insert(name, candid_type);
    }

    pub(super) fn parse_service_candid(
        candid: &str,
    ) -> Result<WrappedCandidTypeService, ParsedCandidError> {
        let mut builder = Self::new(candid);
        builder.read_inner_types()?;
        // println!("read inner done");
        builder.read_service()?;
        // println!("read service done");
        builder
            .service
            .ok_or_else(|| ParsedCandidError::Common("can not parse".to_string()))
    }

    // 读取所有的类型
    fn read_inner_types(&mut self) -> Result<(), ParsedCandidError> {
        while self.trim_type()? {
            self.read_inner_type()?;
        }
        Ok(())
    }
    // 读取下一个类型
    fn read_inner_type(&mut self) -> Result<(), ParsedCandidError> {
        let name = self.read_name()?;
        if self.inner_types.contains_key(&name) {
            return Err(ParsedCandidError::ParsedError(format!(
                "candid type: {} is repeated. at: {}",
                name,
                self.remain(None)
            )));
        }
        // 成功读取到类型名称 下面应该是 =
        self.remove_char('=')?;
        // 下面应该是正常的 candid 类型
        let candid_type = self.read_inner_candid_type()?;
        // println!("read inner type -> {} : {:?}", name, candid_type);
        self.inner_types.insert(name, candid_type);
        Ok(())
    }
    fn read_inner_candid_type(&mut self) -> Result<InnerCandidType, ParsedCandidError> {
        self.trim_start_blank_or_newline()?;
        let candid_type = self.read_name()?;
        let candid_type = match &candid_type[..] {
            "bool" => InnerCandidType::Bool,
            "nat" => InnerCandidType::Nat,
            "int" => InnerCandidType::Int,
            "nat8" => InnerCandidType::Nat8,
            "nat16" => InnerCandidType::Nat16,
            "nat32" => InnerCandidType::Nat32,
            "nat64" => InnerCandidType::Nat64,
            "int8" => InnerCandidType::Int8,
            "int16" => InnerCandidType::Int16,
            "int32" => InnerCandidType::Int32,
            "int64" => InnerCandidType::Int64,
            "float32" => InnerCandidType::Float32,
            "float64" => InnerCandidType::Float64,
            "null" => InnerCandidType::Null,
            "text" => InnerCandidType::Text,
            "principal" => InnerCandidType::Principal,
            "blob" => InnerCandidType::Blob,
            "vec" => {
                let inner = self.read_inner_candid_type()?;
                InnerCandidType::Vec(Box::new(inner))
            }
            "opt" => {
                let inner = self.read_inner_candid_type()?;
                InnerCandidType::Opt(Box::new(inner))
            }
            "record" => {
                self.remove_char('{')?;
                self.trim_start_blank_or_newline()?;
                if self.is_next(&['}']) {
                    self.remove_char('}')?;
                    InnerCandidType::Record(Vec::new())
                } else {
                    let cursor = self.cursor;
                    self.read_name()?;
                    self.trim_start_blank_or_newline()?;
                    if self.is_next(&[':']) {
                        self.cursor = cursor;
                        let mut list: Vec<(String, InnerCandidType)> = Vec::new();
                        while !self.is_next(&['}']) {
                            let name = self.read_name()?;
                            self.trim_start_blank_or_newline()?;
                            self.remove_char(':')?;
                            let inner = self.read_inner_candid_type()?;
                            self.trim_start_blank_or_semicolon()?;
                            list.push((name, inner));
                            self.trim_start_blank_or_newline()?;
                        }
                        self.remove_char('}')?;
                        InnerCandidType::Record(self.sort_list(list))
                    } else {
                        // tuple 也在里面
                        self.cursor = cursor;
                        let mut list: Vec<InnerCandidType> = Vec::new();
                        while !self.is_next(&['}']) {
                            let inner = self.read_inner_candid_type()?;
                            self.trim_start_blank_or_semicolon()?;
                            list.push(inner);
                            self.trim_start_blank_or_newline()?;
                        }
                        self.remove_char('}')?;
                        InnerCandidType::Tuple(list)
                    }
                }
            }
            "variant" => {
                self.remove_char('{')?;
                self.trim_start_blank_or_newline()?;
                let mut list: Vec<(String, Option<InnerCandidType>)> = Vec::new();
                while !self.is_next(&['}']) {
                    let name = self.read_name()?;
                    let mut inner = None;
                    self.trim_start_blank_or_newline()?;
                    if self.is_next(&[':']) {
                        self.remove_char(':')?;
                        inner = Some(self.read_inner_candid_type()?);
                    }
                    self.trim_start_blank_or_semicolon()?;
                    list.push((name, inner));
                    self.trim_start_blank_or_newline()?;
                }
                self.remove_char('}')?;
                InnerCandidType::Variant(self.sort_list(list))
            }
            "unknown" => InnerCandidType::Unknown,
            "empty" => InnerCandidType::Empty,
            "reserved" => InnerCandidType::Reserved,
            "func" => InnerCandidType::Func(self.read_inner_func()?),
            "service" => {
                self.trim_start_blank_or_colon()?;
                self.trim_start_blank_or_newline()?;
                let mut args: Vec<InnerCandidType> = Vec::new();
                if self.is_next(&['(']) {
                    self.remove_char('(')?;
                    while !self.is_next(&[')']) {
                        let inner = self.read_inner_candid_type()?;
                        self.trim_start_blank_or_comma()?;
                        args.push(inner);
                        self.trim_start_blank_or_newline()?;
                    }
                    self.remove_char(')')?;
                    self.trim_start_blank_or_newline()?;
                    self.remove_char('-')?;
                    self.remove_char('>')?;
                }
                self.trim_start_blank_or_newline()?;
                self.remove_char('{')?;
                let mut methods: Vec<(String, InnerCandidTypeFunction)> = Vec::new();
                while !self.is_next(&['}']) {
                    let name = self.read_name()?;
                    self.trim_start_blank_or_newline()?;
                    self.remove_char(':')?;
                    let inner = self.read_inner_func()?;
                    self.trim_start_blank_or_semicolon()?;
                    methods.push((name, inner));
                    self.trim_start_blank_or_newline()?;
                }
                self.remove_char('}')?;
                InnerCandidType::Service(InnerCandidTypeService {
                    args,
                    methods: self.sort_list(methods),
                })
            }
            _ => InnerCandidType::Reference(candid_type),
        };
        self.trim_start_blank_or_semicolon()?;
        Ok(candid_type)
    }
    fn read_inner_func(&mut self) -> Result<InnerCandidTypeFunction, ParsedCandidError> {
        self.trim_start_blank_or_newline()?;
        let mut args: Vec<InnerCandidType> = Vec::new();
        self.remove_char('(')?;
        while !self.is_next(&[')']) {
            let inner = self.read_inner_candid_type()?;
            self.trim_start_blank_or_comma()?;
            args.push(inner);
            self.trim_start_blank_or_newline()?;
        }
        self.remove_char(')')?;
        self.trim_start_blank_or_newline()?;
        self.remove_char('-')?;
        self.remove_char('>')?;
        self.trim_start_blank_or_newline()?;
        let mut rets: Vec<InnerCandidType> = Vec::new();
        self.remove_char('(')?;
        while !self.is_next(&[')']) {
            let inner = self.read_inner_candid_type()?;
            self.trim_start_blank_or_comma()?;
            rets.push(inner);
            self.trim_start_blank_or_newline()?;
        }
        self.remove_char(')')?;
        self.trim_start_blank_or_newline()?;
        let mut annotation = None;
        if self.trim_query()? {
            annotation = Some(FunctionAnnotation::Query)
        } else if self.trim_oneway()? {
            annotation = Some(FunctionAnnotation::Oneway)
        }
        Ok(InnerCandidTypeFunction {
            args,
            rets,
            annotation,
        })
    }

    // 读取 Service
    fn read_service(&mut self) -> Result<(), ParsedCandidError> {
        if !self.trim_service()? {
            return Err(ParsedCandidError::ParsedError(format!(
                "next chars must be 'service' at {}",
                self.remain(None)
            )));
        }

        self.trim_start_blank_or_colon()?;
        self.trim_start_blank_or_newline()?;
        let mut args: Vec<WrappedCandidType> = Vec::new();
        if self.is_next(&['(']) {
            self.remove_char('(')?;
            while !self.is_next(&[')']) {
                // 尝试移除命名变量
                let c = self.cursor;
                let _name = self.read_name();
                self.trim_start_blank_or_newline()?;
                if self.is_next(&[':']) {
                    self.trim_start_blank_or_colon()?;
                } else {
                    self.cursor = c;
                }
                let mut rec_record = RecRecord::new();
                let wrapped = self.read_wrapped_candid_type(&mut rec_record)?;
                self.trim_start_blank_or_comma()?;
                args.push(wrapped);
                self.trim_start_blank_or_newline()?;
            }
            self.remove_char(')')?;
            self.trim_start_blank_or_newline()?;
            self.remove_char('-')?;
            self.remove_char('>')?;
        }
        self.trim_start_blank_or_newline()?;
        self.remove_char('{')?;
        self.trim_start_blank_or_newline()?;
        let mut methods: Vec<(String, WrappedCandidTypeFunction)> = Vec::new();
        while !self.is_next(&['}']) {
            let name = self.read_name()?;
            self.trim_start_blank_or_newline()?;
            self.remove_char(':')?;
            let mut rec_record = RecRecord::new();
            let wrapped = self.read_wrapped_func(&mut rec_record)?;
            self.trim_start_blank_or_semicolon()?;
            methods.push((name, wrapped));
            self.trim_start_blank_or_newline()?;
        }
        self.remove_char('}')?;
        self.service = Some(WrappedCandidTypeService {
            args,
            methods: self.sort_list(methods),
        });

        Ok(())
    }

    fn read_wrapped_func(
        &mut self,
        rec_record: &mut RecRecord,
    ) -> Result<WrappedCandidTypeFunction, ParsedCandidError> {
        self.trim_start_blank_or_newline()?;
        let mut args: Vec<WrappedCandidType> = Vec::new();
        self.remove_char('(')?;
        while !self.is_next(&[')']) {
            let wrapped = self.read_wrapped_candid_type(rec_record)?;
            self.trim_start_blank_or_comma()?;
            args.push(wrapped);
            self.trim_start_blank_or_newline()?;
        }
        self.remove_char(')')?;
        self.trim_start_blank_or_newline()?;
        self.remove_char('-')?;
        self.remove_char('>')?;
        self.trim_start_blank_or_newline()?;
        let mut rets: Vec<WrappedCandidType> = Vec::new();
        self.remove_char('(')?;
        while !self.is_next(&[')']) {
            let wrapped = self.read_wrapped_candid_type(rec_record)?;
            self.trim_start_blank_or_comma()?;
            rets.push(wrapped);
            self.trim_start_blank_or_newline()?;
        }
        self.remove_char(')')?;
        self.trim_start_blank_or_newline()?;
        let mut annotation = None;
        if self.trim_query()? {
            annotation = Some(FunctionAnnotation::Query)
        } else if self.trim_oneway()? {
            annotation = Some(FunctionAnnotation::Oneway)
        }
        Ok(WrappedCandidTypeFunction {
            args,
            rets,
            annotation,
        })
    }

    fn read_wrapped_candid_type(
        &mut self,
        rec_record: &mut RecRecord,
    ) -> Result<WrappedCandidType, ParsedCandidError> {
        self.trim_start_blank_or_newline()?;
        let candid_type = self.read_name()?;
        let candid_type = match &candid_type[..] {
            "bool" => WrappedCandidType::Bool,
            "nat" => WrappedCandidType::Nat,
            "int" => WrappedCandidType::Int,
            "nat8" => WrappedCandidType::Nat8,
            "nat16" => WrappedCandidType::Nat16,
            "nat32" => WrappedCandidType::Nat32,
            "nat64" => WrappedCandidType::Nat64,
            "int8" => WrappedCandidType::Int8,
            "int16" => WrappedCandidType::Int16,
            "int32" => WrappedCandidType::Int32,
            "int64" => WrappedCandidType::Int64,
            "float32" => WrappedCandidType::Float32,
            "float64" => WrappedCandidType::Float64,
            "null" => WrappedCandidType::Null,
            "text" => WrappedCandidType::Text,
            "principal" => WrappedCandidType::Principal,
            "blob" => WrappedCandidType::Vec(Box::new(WrappedCandidType::Nat8)),
            "unknown" => WrappedCandidType::Unknown,
            "empty" => WrappedCandidType::Empty,
            "reserved" => WrappedCandidType::Reserved,
            "vec" => {
                let wrapped = self.read_wrapped_candid_type(rec_record)?;
                WrappedCandidType::Vec(Box::new(wrapped))
            }
            "opt" => {
                let wrapped = self.read_wrapped_candid_type(rec_record)?;
                WrappedCandidType::Opt(Box::new(wrapped))
            }
            "record" => {
                self.remove_char('{')?;
                self.trim_start_blank_or_newline()?;
                if self.is_next(&['}']) {
                    self.remove_char('}')?;
                    WrappedCandidType::Record(Vec::new())
                } else {
                    let cursor = self.cursor;
                    self.read_name()?;
                    self.trim_start_blank_or_newline()?;
                    if self.is_next(&[':']) {
                        self.cursor = cursor;
                        let mut list: Vec<(String, WrappedCandidType)> = Vec::new();
                        while !self.is_next(&['}']) {
                            let name = self.read_name()?;
                            self.trim_start_blank_or_newline()?;
                            self.remove_char(':')?;
                            let wrapped = self.read_wrapped_candid_type(rec_record)?;
                            self.trim_start_blank_or_semicolon()?;
                            list.push((name, wrapped));
                            self.trim_start_blank_or_newline()?;
                        }
                        self.remove_char('}')?;
                        WrappedCandidType::Record(self.sort_list(list))
                    } else {
                        // tuple 也在里面
                        self.cursor = cursor;
                        let mut list: Vec<WrappedCandidType> = Vec::new();
                        while !self.is_next(&['}']) {
                            let wrapped = self.read_wrapped_candid_type(rec_record)?;
                            self.trim_start_blank_or_semicolon()?;
                            list.push(wrapped);
                            self.trim_start_blank_or_newline()?;
                        }
                        self.remove_char('}')?;
                        WrappedCandidType::Tuple(list)
                    }
                }
            }
            "variant" => {
                self.remove_char('{')?;
                self.trim_start_blank_or_newline()?;
                let mut list: Vec<(String, Option<WrappedCandidType>)> = Vec::new();
                while !self.is_next(&['}']) {
                    let name = self.read_name()?;
                    let mut wrapped = None;
                    self.trim_start_blank_or_newline()?;
                    if self.is_next(&[':']) {
                        self.remove_char(':')?;
                        wrapped = Some(self.read_wrapped_candid_type(rec_record)?);
                    }
                    self.trim_start_blank_or_semicolon()?;
                    list.push((name, wrapped));
                    self.trim_start_blank_or_newline()?;
                }
                self.remove_char('}')?;
                WrappedCandidType::Variant(self.sort_list(list))
            }
            "func" => WrappedCandidType::Func(self.read_wrapped_func(rec_record)?),
            "service" => {
                self.trim_start_blank_or_colon()?;
                self.trim_start_blank_or_newline()?;
                let mut args: Vec<WrappedCandidType> = Vec::new();
                if self.is_next(&['(']) {
                    self.remove_char('(')?;
                    while !self.is_next(&[')']) {
                        let wrapped = self.read_wrapped_candid_type(rec_record)?;
                        self.trim_start_blank_or_comma()?;
                        args.push(wrapped);
                        self.trim_start_blank_or_newline()?;
                    }
                    self.remove_char(')')?;
                    self.trim_start_blank_or_newline()?;
                    self.remove_char('-')?;
                    self.remove_char('>')?;
                }
                self.trim_start_blank_or_newline()?;
                self.remove_char('{')?;
                let mut methods: Vec<(String, WrappedCandidTypeFunction)> = Vec::new();
                while !self.is_next(&['}']) {
                    let name = self.read_name()?;
                    self.trim_start_blank_or_newline()?;
                    self.remove_char(':')?;
                    let wrapped_func = self.read_wrapped_func(rec_record)?;
                    self.trim_start_blank_or_semicolon()?;
                    methods.push((name, wrapped_func));
                    self.trim_start_blank_or_newline()?;
                }
                self.remove_char('}')?;
                WrappedCandidType::Service(WrappedCandidTypeService {
                    args,
                    methods: self.sort_list(methods),
                })
            }
            _ => match self.read_wrapped_candid_type_by_name(rec_record, candid_type) {
                Ok(candid_type) => candid_type,
                Err(err) => {
                    if !matches!(err, ParsedCandidError::MissingType(_)) {
                        return Err(err);
                    }
                    self.trim_start_blank_or_colon()?;
                    self.read_wrapped_candid_type(rec_record)?
                }
            },
        };
        self.trim_start_blank_or_semicolon()?;
        Ok(candid_type)
    }

    fn read_wrapped_candid_type_by_name(
        &mut self,
        rec_record: &mut RecRecord,
        name: String,
    ) -> Result<WrappedCandidType, ParsedCandidError> {
        // 如果已经有了
        if let Some(candid_type) = self.wrapped_types.get(&name) {
            return Ok(candid_type.clone());
        }
        // 如果没有
        let inner = self.inner_types.get(&name);
        let inner = inner
            .ok_or_else(|| ParsedCandidError::MissingType(name.clone()))?
            .clone();
        rec_record.push(name.clone());
        let mut wrapped = self.read_wrapped_candid_type_by_inner(rec_record, &inner)?;
        if let Some(id) = rec_record.id(&name) {
            wrapped = WrappedCandidType::Rec(WrappedCandidTypeRecursion {
                ty: Box::new(wrapped),
                id,
                name: Some(name.clone()),
            });
            rec_record.remove(&name)?;
        } else {
            self.known_type(name, wrapped.clone());
        }
        rec_record.pop()?;
        Ok(wrapped)
    }
    fn read_wrapped_candid_type_by_inner(
        &mut self,
        rec_record: &mut RecRecord,
        inner: &InnerCandidType,
    ) -> Result<WrappedCandidType, ParsedCandidError> {
        // 如果没有
        let wrapped = match inner {
            InnerCandidType::Bool => WrappedCandidType::Bool,
            InnerCandidType::Nat => WrappedCandidType::Nat,
            InnerCandidType::Int => WrappedCandidType::Int,
            InnerCandidType::Nat8 => WrappedCandidType::Nat8,
            InnerCandidType::Nat16 => WrappedCandidType::Nat16,
            InnerCandidType::Nat32 => WrappedCandidType::Nat32,
            InnerCandidType::Nat64 => WrappedCandidType::Nat64,
            InnerCandidType::Int8 => WrappedCandidType::Int8,
            InnerCandidType::Int16 => WrappedCandidType::Int16,
            InnerCandidType::Int32 => WrappedCandidType::Int32,
            InnerCandidType::Int64 => WrappedCandidType::Int64,
            InnerCandidType::Float32 => WrappedCandidType::Float32,
            InnerCandidType::Float64 => WrappedCandidType::Float64,
            InnerCandidType::Null => WrappedCandidType::Null,
            InnerCandidType::Text => WrappedCandidType::Text,
            InnerCandidType::Principal => WrappedCandidType::Principal,
            InnerCandidType::Blob => WrappedCandidType::Vec(Box::new(WrappedCandidType::Nat8)),
            InnerCandidType::Vec(inner) => WrappedCandidType::Vec(Box::new(
                self.read_wrapped_candid_type_by_inner(rec_record, inner)?,
            )),
            InnerCandidType::Opt(inner) => WrappedCandidType::Opt(Box::new(
                self.read_wrapped_candid_type_by_inner(rec_record, inner)?,
            )),
            InnerCandidType::Record(inners) => {
                let mut list = Vec::new();
                for (name, inner) in inners {
                    list.push((
                        name.clone(),
                        self.read_wrapped_candid_type_by_inner(rec_record, inner)?,
                    ))
                }
                WrappedCandidType::Record(list)
            }
            InnerCandidType::Variant(inners) => {
                let mut list = Vec::new();
                for (name, inner) in inners {
                    let mut inner_type = None;
                    if let Some(inner) = inner {
                        inner_type =
                            Some(self.read_wrapped_candid_type_by_inner(rec_record, inner)?);
                    }
                    list.push((name.clone(), inner_type))
                }
                WrappedCandidType::Variant(list)
            }
            InnerCandidType::Tuple(inners) => {
                let mut list = Vec::new();
                for inner in inners {
                    list.push(self.read_wrapped_candid_type_by_inner(rec_record, inner)?)
                }
                WrappedCandidType::Tuple(list)
            }
            InnerCandidType::Unknown => WrappedCandidType::Unknown,
            InnerCandidType::Empty => WrappedCandidType::Empty,
            InnerCandidType::Reserved => WrappedCandidType::Reserved,
            InnerCandidType::Func(InnerCandidTypeFunction {
                args,
                rets,
                annotation,
            }) => {
                let mut wrapped_args = Vec::new();
                for inner in args {
                    wrapped_args.push(self.read_wrapped_candid_type_by_inner(rec_record, inner)?)
                }
                let mut wrapped_results = Vec::new();
                for inner in rets {
                    wrapped_results.push(self.read_wrapped_candid_type_by_inner(rec_record, inner)?)
                }
                WrappedCandidType::Func(WrappedCandidTypeFunction {
                    args: wrapped_args,
                    rets: wrapped_results,
                    annotation: *annotation,
                })
            }
            InnerCandidType::Service(InnerCandidTypeService { args, methods }) => {
                let mut wrapped_args = Vec::new();
                for inner in args {
                    wrapped_args.push(self.read_wrapped_candid_type_by_inner(rec_record, inner)?)
                }
                let mut wrapped_methods = Vec::new();
                for (name, inner) in methods {
                    let mut wrapped_args = Vec::new();
                    for inner in &inner.args {
                        wrapped_args
                            .push(self.read_wrapped_candid_type_by_inner(rec_record, inner)?)
                    }
                    let mut wrapped_results = Vec::new();
                    for inner in &inner.rets {
                        wrapped_results
                            .push(self.read_wrapped_candid_type_by_inner(rec_record, inner)?)
                    }

                    wrapped_methods.push((
                        name.clone(),
                        WrappedCandidTypeFunction {
                            args: wrapped_args,
                            rets: wrapped_results,
                            annotation: inner.annotation,
                        },
                    ))
                }
                WrappedCandidType::Service(WrappedCandidTypeService {
                    args: wrapped_args,
                    methods: wrapped_methods,
                })
            }
            InnerCandidType::Reference(name) => {
                let id = rec_record.id(name);
                if let Some(id) = id {
                    WrappedCandidType::Reference(id)
                } else if rec_record.contains(name) {
                    let id = rec_record.insert(name.clone())?;
                    WrappedCandidType::Reference(id)
                } else {
                    self.read_wrapped_candid_type_by_name(rec_record, name.clone())?
                }
            }
        };
        Ok(wrapped)
    }
}
