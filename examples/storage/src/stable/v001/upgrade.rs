use super::super::v000::types::{HeapData as LastHeapData, InnerState as LastState};

use super::types::*;

impl From<Box<LastState>> for Box<InnerState> {
    fn from(value: Box<LastState>) -> Self {
        let mut state = InnerState::default(); // ? 初始化

        // 1. 继承之前的数据
        let LastHeapData {
            pause,
            permissions,
            records,
            schedule,
        } = value.heap;
        state.heap.pause = pause;
        state.heap.permissions = permissions;
        state.heap.records = records;
        state.heap.schedule = schedule;

        Box::new(state)
    }
}
