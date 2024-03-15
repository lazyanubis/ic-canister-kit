use super::super::v000::types::InnerState as LastState;

use super::types::*;

impl From<LastState> for InnerState {
    fn from(value: LastState) -> Self {
        let mut state = InnerState::default(); // ? 初始化

        // 1. 继承之前的数据
        let LastState {
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
