use super::*;
#[allow(unused)]
pub use ic_canister_kit::identity::self_canister_id;
use ic_canister_kit::stable::pausable::PauseReason;
#[allow(unused)]
pub use ic_canister_kit::types::{CanisterId, UserId};
#[allow(unused)]
pub use std::collections::{HashMap, HashSet};
#[allow(unused)]
pub use std::fmt::Display;

pub trait Business:
    Pausable<PauseReason>
    + ParsePermission
    + Permissable<Permission>
    + Recordable<Record, RecordTopic, RecordSearch>
    + Schedulable
    + ScheduleTask
{
    fn business_test_template_query(&self) -> String;
    fn business_test_template_update(&mut self, test: String);
}

// 业务实现
impl Business for State {
    fn business_test_template_query(&self) -> String {
        self.get().business_test_template_query()
    }
    fn business_test_template_update(&mut self, test: String) {
        self.get_mut().business_test_template_update(test)
    }
}
