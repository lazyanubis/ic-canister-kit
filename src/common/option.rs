use std::fmt::Display;

/// 格式化 option 对象
pub fn display_option<T: Display>(value: &Option<T>) -> String {
    if let Some(value) = value {
        return value.to_string();
    }
    "None".into()
}

/// 格式化 option 对象
pub fn display_option_by<T, F: Fn(&T) -> String>(value: &Option<T>, f: F) -> String {
    if let Some(value) = value {
        return f(value);
    }
    "None".into()
}
