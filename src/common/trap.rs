/// 拆箱
/// ! 可能中止程序
pub fn trap<T, E: std::fmt::Display>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => ic_cdk::trap(&err.to_string()),
    }
}

/// 拆箱
/// ! 可能中止程序
pub fn trap_string<T, E>(result: Result<T, E>) -> T
where
    E: Into<String>,
{
    match result {
        Ok(value) => value,
        Err(err) => ic_cdk::trap(&err.into()),
    }
}

/// 拆箱
/// ! 可能中止程序
pub fn trap_debug<T, E: std::fmt::Debug>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => ic_cdk::trap(&format!("{err:?}")),
    }
}
