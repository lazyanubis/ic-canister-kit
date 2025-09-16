use std::collections::HashMap;

use super::{error::ParsedCandidError, types::*};

#[derive(Debug, Clone)]
struct InnerCandidTypeFunction {
    args: Vec<InnerCandidType>,
    rets: Vec<InnerCandidType>,
    annotation: Option<FunctionAnnotation>,

    name: Option<String>,
}

#[derive(Debug, Clone)]
struct InnerCandidTypeService {
    args: Vec<InnerCandidType>,
    methods: Vec<(String, InnerCandidTypeFunction)>,

    name: Option<String>,
}

#[derive(Debug, Clone)]
enum InnerCandidType {
    Bool(Option<String>),
    Nat(Option<String>),
    Int(Option<String>),
    Nat8(Option<String>),
    Nat16(Option<String>),
    Nat32(Option<String>),
    Nat64(Option<String>),
    Int8(Option<String>),
    Int16(Option<String>),
    Int32(Option<String>),
    Int64(Option<String>),
    Float32(Option<String>),
    Float64(Option<String>),
    Null(Option<String>),
    Text(Option<String>),
    Principal(Option<String>),
    Blob(Option<String>),
    Vec(Box<InnerCandidType>, Option<String>),
    Opt(Box<InnerCandidType>, Option<String>),
    Record(Vec<(String, InnerCandidType)>, Option<String>),
    Variant(Vec<(String, Option<InnerCandidType>)>, Option<String>),
    Tuple(Vec<InnerCandidType>, Option<String>),
    Unknown(Option<String>),
    Empty(Option<String>),
    Reserved(Option<String>),
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
    inner_types: HashMap<String, InnerCandidType>,     // 临时类型
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
        self.trim_start_blank_or_chars(&['\n'])?;
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

    pub(super) fn parse_service_candid(candid: &str) -> Result<WrappedCandidTypeService, ParsedCandidError> {
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
        let candid_type = self.read_inner_candid_type(Some(name.clone()))?;
        // println!("read inner type -> {} : {:?}", name, candid_type);
        self.inner_types.insert(name, candid_type);
        Ok(())
    }
    fn read_inner_candid_type(&mut self, name: Option<String>) -> Result<InnerCandidType, ParsedCandidError> {
        self.trim_start_blank_or_newline()?;
        let candid_type = self.read_name()?;
        let candid_type = match &candid_type[..] {
            "bool" => InnerCandidType::Bool(name),
            "nat" => InnerCandidType::Nat(name),
            "int" => InnerCandidType::Int(name),
            "nat8" => InnerCandidType::Nat8(name),
            "nat16" => InnerCandidType::Nat16(name),
            "nat32" => InnerCandidType::Nat32(name),
            "nat64" => InnerCandidType::Nat64(name),
            "int8" => InnerCandidType::Int8(name),
            "int16" => InnerCandidType::Int16(name),
            "int32" => InnerCandidType::Int32(name),
            "int64" => InnerCandidType::Int64(name),
            "float32" => InnerCandidType::Float32(name),
            "float64" => InnerCandidType::Float64(name),
            "null" => InnerCandidType::Null(name),
            "text" => InnerCandidType::Text(name),
            "principal" => InnerCandidType::Principal(name),
            "blob" => InnerCandidType::Blob(name),
            "vec" => {
                let inner = self.read_inner_candid_type(None)?;
                InnerCandidType::Vec(Box::new(inner), name)
            }
            "opt" => {
                let inner = self.read_inner_candid_type(None)?;
                InnerCandidType::Opt(Box::new(inner), name)
            }
            "record" => {
                self.remove_char('{')?;
                self.trim_start_blank_or_newline()?;
                if self.is_next(&['}']) {
                    self.remove_char('}')?;
                    InnerCandidType::Record(Vec::new(), name)
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
                            let inner = self.read_inner_candid_type(None)?;
                            self.trim_start_blank_or_semicolon()?;
                            list.push((name, inner));
                            self.trim_start_blank_or_newline()?;
                        }
                        self.remove_char('}')?;
                        InnerCandidType::Record(self.sort_list(list), name)
                    } else {
                        // tuple 也在里面
                        self.cursor = cursor;
                        let mut list: Vec<InnerCandidType> = Vec::new();
                        while !self.is_next(&['}']) {
                            let inner = self.read_inner_candid_type(None)?;
                            self.trim_start_blank_or_semicolon()?;
                            list.push(inner);
                            self.trim_start_blank_or_newline()?;
                        }
                        self.remove_char('}')?;
                        InnerCandidType::Tuple(list, name)
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
                        inner = Some(self.read_inner_candid_type(None)?);
                    }
                    self.trim_start_blank_or_semicolon()?;
                    list.push((name, inner));
                    self.trim_start_blank_or_newline()?;
                }
                self.remove_char('}')?;
                InnerCandidType::Variant(self.sort_list(list), name)
            }
            "unknown" => InnerCandidType::Unknown(name),
            "empty" => InnerCandidType::Empty(name),
            "reserved" => InnerCandidType::Reserved(name),
            "func" => InnerCandidType::Func(self.read_inner_func(name)?),
            "service" => {
                self.trim_start_blank_or_colon()?;
                self.trim_start_blank_or_newline()?;
                let mut args: Vec<InnerCandidType> = Vec::new();
                if self.is_next(&['(']) {
                    self.remove_char('(')?;
                    while !self.is_next(&[')']) {
                        let inner = self.read_inner_candid_type(None)?;
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
                    let inner = self.read_inner_func(None)?;
                    self.trim_start_blank_or_semicolon()?;
                    methods.push((name, inner));
                    self.trim_start_blank_or_newline()?;
                }
                self.remove_char('}')?;
                InnerCandidType::Service(InnerCandidTypeService {
                    args,
                    methods: self.sort_list(methods),
                    name,
                })
            }
            _ => InnerCandidType::Reference(candid_type),
        };
        self.trim_start_blank_or_semicolon()?;
        Ok(candid_type)
    }
    fn read_inner_func(&mut self, name: Option<String>) -> Result<InnerCandidTypeFunction, ParsedCandidError> {
        self.trim_start_blank_or_newline()?;
        let mut args: Vec<InnerCandidType> = Vec::new();
        self.remove_char('(')?;
        while !self.is_next(&[')']) {
            let inner = self.read_inner_candid_type(None)?;
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
            let inner = self.read_inner_candid_type(None)?;
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
            name,
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

        self.trim_start_blank_or_newline()?;
        if !self.is_next(&[':']) {
            return Err(ParsedCandidError::ParsedError(format!(
                "next chars must be ':' after 'service' at {}",
                self.remain(None)
            )));
        }
        self.trim_start_blank_or_colon()?; // remove :

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
                let wrapped = self.read_wrapped_candid_type(&mut rec_record, None)?;
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

        if !self.is_next(&['{']) {
            let name = self.read_name()?;
            if let Some(InnerCandidType::Service(InnerCandidTypeService { args, methods, name })) =
                &self.inner_types.get(&name).cloned()
            {
                let mut rec_record = RecRecord::new();
                let mut wrapped_args = Vec::new();
                for inner in args {
                    wrapped_args.push(self.read_wrapped_candid_type_by_inner(&mut rec_record, inner)?)
                }
                let mut wrapped_methods = Vec::new();
                for (name, inner) in methods {
                    let mut wrapped_args = Vec::new();
                    for inner in &inner.args {
                        wrapped_args.push(self.read_wrapped_candid_type_by_inner(&mut rec_record, inner)?)
                    }
                    let mut wrapped_results = Vec::new();
                    for inner in &inner.rets {
                        wrapped_results.push(self.read_wrapped_candid_type_by_inner(&mut rec_record, inner)?)
                    }

                    wrapped_methods.push((
                        name.clone(),
                        WrappedCandidTypeFunction {
                            args: wrapped_args,
                            rets: wrapped_results,
                            annotation: inner.annotation,
                            name: None,
                        },
                    ))
                }

                self.service = Some(WrappedCandidTypeService {
                    args: wrapped_args,
                    methods: wrapped_methods,
                    name: name.clone(),
                });
                return Ok(());
            }
            let ty = self
                .wrapped_types
                .get(&name)
                .ok_or_else(|| ParsedCandidError::ParsedError(format!("can not find service type: {name}")))?;
            if let WrappedCandidType::Service(service) = ty {
                self.service = Some(service.clone());
                return Ok(());
            }
            return Err(ParsedCandidError::ParsedError(format!("type: {name} is not service")));
        }

        self.remove_char('{')?;
        self.trim_start_blank_or_newline()?;
        let mut methods: Vec<(String, WrappedCandidTypeFunction)> = Vec::new();
        while !self.is_next(&['}']) {
            let name = self.read_name()?;
            self.trim_start_blank_or_newline()?;
            self.remove_char(':')?;
            let mut rec_record = RecRecord::new();
            let wrapped = self.read_wrapped_func(&mut rec_record, None)?;
            self.trim_start_blank_or_semicolon()?;
            methods.push((name, wrapped));
            self.trim_start_blank_or_newline()?;
        }
        self.remove_char('}')?;

        self.service = Some(WrappedCandidTypeService {
            args,
            methods: self.sort_list(methods),
            name: None,
        });

        Ok(())
    }

    fn read_wrapped_func(
        &mut self,
        rec_record: &mut RecRecord,
        name: Option<String>,
    ) -> Result<WrappedCandidTypeFunction, ParsedCandidError> {
        self.trim_start_blank_or_newline()?;
        let mut args: Vec<WrappedCandidType> = Vec::new();
        self.remove_char('(')?;
        while !self.is_next(&[')']) {
            let wrapped = self.read_wrapped_candid_type(rec_record, None)?;
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
            let wrapped = self.read_wrapped_candid_type(rec_record, None)?;
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
            name,
        })
    }

    fn read_wrapped_candid_type(
        &mut self,
        rec_record: &mut RecRecord,
        name: Option<String>,
    ) -> Result<WrappedCandidType, ParsedCandidError> {
        self.trim_start_blank_or_newline()?;
        let candid_type = self.read_name()?;
        let candid_type = match &candid_type[..] {
            "bool" => WrappedCandidType::Bool(WrappedCandidTypeName::from(name)),
            "nat" => WrappedCandidType::Nat(WrappedCandidTypeName::from(name)),
            "int" => WrappedCandidType::Int(WrappedCandidTypeName::from(name)),
            "nat8" => WrappedCandidType::Nat8(WrappedCandidTypeName::from(name)),
            "nat16" => WrappedCandidType::Nat16(WrappedCandidTypeName::from(name)),
            "nat32" => WrappedCandidType::Nat32(WrappedCandidTypeName::from(name)),
            "nat64" => WrappedCandidType::Nat64(WrappedCandidTypeName::from(name)),
            "int8" => WrappedCandidType::Int8(WrappedCandidTypeName::from(name)),
            "int16" => WrappedCandidType::Int16(WrappedCandidTypeName::from(name)),
            "int32" => WrappedCandidType::Int32(WrappedCandidTypeName::from(name)),
            "int64" => WrappedCandidType::Int64(WrappedCandidTypeName::from(name)),
            "float32" => WrappedCandidType::Float32(WrappedCandidTypeName::from(name)),
            "float64" => WrappedCandidType::Float64(WrappedCandidTypeName::from(name)),
            "null" => WrappedCandidType::Null(WrappedCandidTypeName::from(name)),
            "text" => WrappedCandidType::Text(WrappedCandidTypeName::from(name)),
            "principal" => WrappedCandidType::Principal(WrappedCandidTypeName::from(name)),
            "blob" => WrappedCandidType::Vec(WrappedCandidTypeSubtype {
                subtype: Box::new(WrappedCandidType::Nat8(WrappedCandidTypeName::default())),
                name,
            }),
            "unknown" => WrappedCandidType::Unknown(WrappedCandidTypeName::from(name)),
            "empty" => WrappedCandidType::Empty(WrappedCandidTypeName::from(name)),
            "reserved" => WrappedCandidType::Reserved(WrappedCandidTypeName::from(name)),
            "vec" => {
                let wrapped = self.read_wrapped_candid_type(rec_record, None)?;
                WrappedCandidType::Vec(WrappedCandidTypeSubtype {
                    subtype: Box::new(wrapped),
                    name,
                })
            }
            "opt" => {
                let wrapped = self.read_wrapped_candid_type(rec_record, None)?;
                WrappedCandidType::Opt(WrappedCandidTypeSubtype {
                    subtype: Box::new(wrapped),
                    name,
                })
            }
            "record" => {
                self.remove_char('{')?;
                self.trim_start_blank_or_newline()?;
                if self.is_next(&['}']) {
                    self.remove_char('}')?;
                    WrappedCandidType::Record(WrappedCandidTypeRecord {
                        subitems: Vec::new(),
                        name,
                    })
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
                            let wrapped = self.read_wrapped_candid_type(rec_record, None)?;
                            self.trim_start_blank_or_semicolon()?;
                            list.push((name, wrapped));
                            self.trim_start_blank_or_newline()?;
                        }
                        self.remove_char('}')?;
                        WrappedCandidType::Record(WrappedCandidTypeRecord {
                            subitems: self.sort_list(list),
                            name,
                        })
                    } else {
                        // tuple 也在里面
                        self.cursor = cursor;
                        let mut list: Vec<WrappedCandidType> = Vec::new();
                        while !self.is_next(&['}']) {
                            let wrapped = self.read_wrapped_candid_type(rec_record, None)?;
                            self.trim_start_blank_or_semicolon()?;
                            list.push(wrapped);
                            self.trim_start_blank_or_newline()?;
                        }
                        self.remove_char('}')?;
                        WrappedCandidType::Tuple(WrappedCandidTypeTuple { subitems: list, name })
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
                        wrapped = Some(self.read_wrapped_candid_type(rec_record, None)?);
                    }
                    self.trim_start_blank_or_semicolon()?;
                    list.push((name, wrapped));
                    self.trim_start_blank_or_newline()?;
                }
                self.remove_char('}')?;
                WrappedCandidType::Variant(WrappedCandidTypeVariant {
                    subitems: self.sort_list(list),
                    name,
                })
            }
            "func" => WrappedCandidType::Func(self.read_wrapped_func(rec_record, name)?),
            "service" => {
                self.trim_start_blank_or_colon()?;
                self.trim_start_blank_or_newline()?;
                let mut args: Vec<WrappedCandidType> = Vec::new();
                if self.is_next(&['(']) {
                    self.remove_char('(')?;
                    while !self.is_next(&[')']) {
                        let wrapped = self.read_wrapped_candid_type(rec_record, None)?;
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
                    let wrapped_func = self.read_wrapped_func(rec_record, None)?;
                    self.trim_start_blank_or_semicolon()?;
                    methods.push((name, wrapped_func));
                    self.trim_start_blank_or_newline()?;
                }
                self.remove_char('}')?;
                WrappedCandidType::Service(WrappedCandidTypeService {
                    args,
                    methods: self.sort_list(methods),
                    name,
                })
            }
            _ => match self.read_wrapped_candid_type_by_name(rec_record, candid_type.clone()) {
                Ok(candid_type) => candid_type,
                Err(err) => {
                    match err {
                        ParsedCandidError::MissingType(n) if n == candid_type => {}
                        err => return Err(err),
                    }
                    self.trim_start_blank_or_colon()?;
                    match self.read_wrapped_candid_type(rec_record, Some(candid_type.clone())) {
                        Ok(candid_type) => candid_type,
                        Err(err) => {
                            if let ParsedCandidError::ParsedError(err) = &err
                                && err.contains("can not read string")
                            {
                                return Err(ParsedCandidError::MissingType(candid_type));
                            }
                            return Err(err);
                        }
                    }
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
        }
        self.known_type(name, wrapped.clone());
        rec_record.pop()?;
        Ok(wrapped)
    }
    fn read_wrapped_candid_type_by_inner(
        &mut self,
        rec_record: &mut RecRecord,
        inner: &InnerCandidType,
    ) -> Result<WrappedCandidType, ParsedCandidError> {
        // 如果没有
        let wrapped = match inner.clone() {
            InnerCandidType::Bool(name) => WrappedCandidType::Bool(WrappedCandidTypeName::from(name)),
            InnerCandidType::Nat(name) => WrappedCandidType::Nat(WrappedCandidTypeName::from(name)),
            InnerCandidType::Int(name) => WrappedCandidType::Int(WrappedCandidTypeName::from(name)),
            InnerCandidType::Nat8(name) => WrappedCandidType::Nat8(WrappedCandidTypeName::from(name)),
            InnerCandidType::Nat16(name) => WrappedCandidType::Nat16(WrappedCandidTypeName::from(name)),
            InnerCandidType::Nat32(name) => WrappedCandidType::Nat32(WrappedCandidTypeName::from(name)),
            InnerCandidType::Nat64(name) => WrappedCandidType::Nat64(WrappedCandidTypeName::from(name)),
            InnerCandidType::Int8(name) => WrappedCandidType::Int8(WrappedCandidTypeName::from(name)),
            InnerCandidType::Int16(name) => WrappedCandidType::Int16(WrappedCandidTypeName::from(name)),
            InnerCandidType::Int32(name) => WrappedCandidType::Int32(WrappedCandidTypeName::from(name)),
            InnerCandidType::Int64(name) => WrappedCandidType::Int64(WrappedCandidTypeName::from(name)),
            InnerCandidType::Float32(name) => WrappedCandidType::Float32(WrappedCandidTypeName::from(name)),
            InnerCandidType::Float64(name) => WrappedCandidType::Float64(WrappedCandidTypeName::from(name)),
            InnerCandidType::Null(name) => WrappedCandidType::Null(WrappedCandidTypeName::from(name)),
            InnerCandidType::Text(name) => WrappedCandidType::Text(WrappedCandidTypeName::from(name)),
            InnerCandidType::Principal(name) => WrappedCandidType::Principal(WrappedCandidTypeName::from(name)),
            InnerCandidType::Blob(name) => WrappedCandidType::Vec(WrappedCandidTypeSubtype {
                subtype: Box::new(WrappedCandidType::Nat8(WrappedCandidTypeName::default())),
                name,
            }),
            InnerCandidType::Vec(inner, name) => WrappedCandidType::Vec(WrappedCandidTypeSubtype {
                subtype: Box::new(self.read_wrapped_candid_type_by_inner(rec_record, &inner)?),
                name,
            }),
            InnerCandidType::Opt(inner, name) => WrappedCandidType::Opt(WrappedCandidTypeSubtype {
                subtype: Box::new(self.read_wrapped_candid_type_by_inner(rec_record, &inner)?),
                name,
            }),
            InnerCandidType::Record(inners, name) => {
                let mut list = Vec::new();
                for (name, inner) in inners {
                    list.push((
                        name.clone(),
                        self.read_wrapped_candid_type_by_inner(rec_record, &inner)?,
                    ))
                }
                WrappedCandidType::Record(WrappedCandidTypeRecord { subitems: list, name })
            }
            InnerCandidType::Variant(inners, name) => {
                let mut list = Vec::new();
                for (name, inner) in inners {
                    let mut inner_type = None;
                    if let Some(inner) = inner {
                        inner_type = Some(self.read_wrapped_candid_type_by_inner(rec_record, &inner)?);
                    }
                    list.push((name.clone(), inner_type))
                }
                WrappedCandidType::Variant(WrappedCandidTypeVariant { subitems: list, name })
            }
            InnerCandidType::Tuple(inners, name) => {
                let mut list = Vec::new();
                for inner in inners {
                    list.push(self.read_wrapped_candid_type_by_inner(rec_record, &inner)?)
                }
                WrappedCandidType::Tuple(WrappedCandidTypeTuple { subitems: list, name })
            }
            InnerCandidType::Unknown(name) => WrappedCandidType::Unknown(WrappedCandidTypeName::from(name)),
            InnerCandidType::Empty(name) => WrappedCandidType::Empty(WrappedCandidTypeName::from(name)),
            InnerCandidType::Reserved(name) => WrappedCandidType::Reserved(WrappedCandidTypeName::from(name)),
            InnerCandidType::Func(InnerCandidTypeFunction {
                args,
                rets,
                annotation,
                name,
            }) => {
                let mut wrapped_args = Vec::new();
                for inner in args {
                    wrapped_args.push(self.read_wrapped_candid_type_by_inner(rec_record, &inner)?)
                }
                let mut wrapped_results = Vec::new();
                for inner in rets {
                    wrapped_results.push(self.read_wrapped_candid_type_by_inner(rec_record, &inner)?)
                }
                WrappedCandidType::Func(WrappedCandidTypeFunction {
                    args: wrapped_args,
                    rets: wrapped_results,
                    annotation,
                    name,
                })
            }
            InnerCandidType::Service(InnerCandidTypeService { args, methods, name }) => {
                let mut wrapped_args = Vec::new();
                for inner in args {
                    wrapped_args.push(self.read_wrapped_candid_type_by_inner(rec_record, &inner)?)
                }
                let mut wrapped_methods = Vec::new();
                for (name, inner) in methods {
                    let mut wrapped_args = Vec::new();
                    for inner in &inner.args {
                        wrapped_args.push(self.read_wrapped_candid_type_by_inner(rec_record, inner)?)
                    }
                    let mut wrapped_results = Vec::new();
                    for inner in &inner.rets {
                        wrapped_results.push(self.read_wrapped_candid_type_by_inner(rec_record, inner)?)
                    }

                    wrapped_methods.push((
                        name.clone(),
                        WrappedCandidTypeFunction {
                            args: wrapped_args,
                            rets: wrapped_results,
                            annotation: inner.annotation,
                            name: None,
                        },
                    ))
                }
                WrappedCandidType::Service(WrappedCandidTypeService {
                    args: wrapped_args,
                    methods: wrapped_methods,
                    name,
                })
            }
            InnerCandidType::Reference(name) => {
                let id = rec_record.id(&name);
                if let Some(id) = id {
                    WrappedCandidType::Reference(WrappedCandidTypeReference {
                        id,
                        name: Some(name.clone()),
                    })
                } else if rec_record.contains(&name) {
                    let id = rec_record.insert(name.clone())?;
                    WrappedCandidType::Reference(WrappedCandidTypeReference {
                        id,
                        name: Some(name.clone()),
                    })
                } else {
                    self.read_wrapped_candid_type_by_name(rec_record, name.clone())?
                }
            }
        };
        Ok(wrapped)
    }
}
