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
}
