use candid::CandidType;
use serde::Deserialize;

use crate::stable::Stable;

#[derive(CandidType, Deserialize, Debug, Default)]
pub struct NftTraits {
    traits: Vec<(String, String)>, // 该时间段内,是限制时间
}

pub type NftTraitsState = (Vec<(String, String)>,);

impl Stable<NftTraitsState, NftTraitsState> for NftTraits {
    fn save(&mut self) -> NftTraitsState {
        let traits = std::mem::take(&mut self.traits);
        (traits,)
    }

    fn restore(&mut self, state: NftTraitsState) {
        let _ = std::mem::replace(&mut self.traits, state.0);
    }
}

impl NftTraits {
    pub fn set_traits(&mut self, traits: Vec<(String, String)>) {
        self.traits = traits;
    }
    pub fn get_traits(&self) -> Vec<(String, String)> {
        self.traits.clone()
    }
}
