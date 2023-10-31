#[derive(candid::CandidType, serde::Deserialize, Debug, Default, Clone)]
pub struct NftTraits(Vec<(String, String)>); // 属性
