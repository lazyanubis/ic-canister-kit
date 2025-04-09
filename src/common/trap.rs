/// 拆箱
/// ! 可能中止程序
pub fn trap<T, E>(result: Result<T, E>) -> T
where
    E: Into<String>,
{
    match result {
        Ok(value) => value,
        Err(err) => ic_cdk::trap(&err.into()),
    }
}
