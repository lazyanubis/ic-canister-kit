/// motoko 结果

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub enum MotokoResult<T, E> {
    /// 正常
    #[serde(rename = "ok")]
    Ok(T),
    /// 错误
    #[serde(rename = "err")]
    Err(E),
}

impl<T, E> From<Result<T, E>> for MotokoResult<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(ok) => MotokoResult::Ok(ok),
            Err(err) => MotokoResult::Err(err),
        }
    }
}

impl<T, E> From<MotokoResult<T, E>> for Result<T, E> {
    fn from(value: MotokoResult<T, E>) -> Self {
        match value {
            MotokoResult::Ok(ok) => Ok(ok),
            MotokoResult::Err(err) => Err(err),
        }
    }
}
