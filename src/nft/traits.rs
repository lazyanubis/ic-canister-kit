#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct NftTraits(Vec<(String, String)>); // 属性
