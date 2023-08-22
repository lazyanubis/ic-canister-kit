/// motoko 结果

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub enum MotokoResult<T, E> {
    #[serde(rename = "ok")]
    Ok(T),
    #[serde(rename = "err")]
    Err(E),
}

impl<T, E> From<Result<T, E>> for MotokoResult<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(ok) => MotokoResult::Ok(ok),
            Err(e) => MotokoResult::Err(e),
        }
    }
}

impl<T, E> Into<Result<T, E>> for MotokoResult<T, E> {
    fn into(self) -> Result<T, E> {
        match self {
            MotokoResult::Ok(ok) => Ok(ok),
            MotokoResult::Err(e) => Err(e),
        }
    }
}
