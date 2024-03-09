use super::super::v000::types::InnerState as OldState;

use super::types::*;

impl From<OldState> for InnerState {
    fn from(value: OldState) -> Self {
        let mut state = InnerState::default(); // ? 初始化

        // 1. 继承之前的数据
        let OldState {
            pause,
            permissions,
            records,
            schedule,
        } = value;
        state.pause = pause;
        state.permissions = permissions;
        state.records = records;
        state.schedule = schedule;

        state
    }
}
