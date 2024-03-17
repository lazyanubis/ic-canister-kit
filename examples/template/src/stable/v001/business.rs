use super::super::business::*;
use super::types::*;

#[allow(unused_variables)]
impl Business for InnerState {
    fn business_example_query(&self) -> String {
        self.heap_state.business.example_data.clone()
    }

    fn business_example_update(&mut self, test: String) {
        self.heap_state.business.example_data = test
    }
}
