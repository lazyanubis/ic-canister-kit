use super::super::business::*;
use super::types::*;

#[allow(clippy::panic)] // ? SAFETY
#[allow(unused_variables)]
impl Business for InnerState {
    fn business_example_query(&self) -> String {
        panic!("Not supported operation by this version.")
    }
    fn business_example_update(&mut self, test: String) {
        panic!("Not supported operation by this version.")
    }

    fn business_example_cell_query(&self) -> crate::stable::ExampleCell {
        panic!("Not supported operation by this version.")
    }
    fn business_example_cell_update(&mut self, test: String) {
        panic!("Not supported operation by this version.")
    }

    fn business_example_vec_query(&self) -> Vec<crate::stable::ExampleVec> {
        panic!("Not supported operation by this version.")
    }
    fn business_example_vec_push(&mut self, test: u64) {
        panic!("Not supported operation by this version.")
    }

    fn business_example_vec_pop(&mut self) -> Option<crate::stable::ExampleVec> {
        panic!("Not supported operation by this version.")
    }
    fn business_example_map_query(&self) -> HashMap<u64, String> {
        panic!("Not supported operation by this version.")
    }
    fn business_example_map_update(&mut self, key: u64, value: Option<String>) -> Option<String> {
        panic!("Not supported operation by this version.")
    }

    fn business_example_log_query(&self) -> Vec<String> {
        panic!("Not supported operation by this version.")
    }
    fn business_example_log_update(&mut self, item: String) -> u64 {
        panic!("Not supported operation by this version.")
    }

    fn business_example_priority_queue_query(&self) -> Vec<crate::stable::ExampleVec> {
        panic!("Not supported operation by this version.")
    }
    fn business_example_priority_queue_push(&mut self, item: u64) {
        panic!("Not supported operation by this version.")
    }
    fn business_example_priority_queue_pop(&mut self) -> Option<crate::stable::ExampleVec> {
        panic!("Not supported operation by this version.")
    }
}
