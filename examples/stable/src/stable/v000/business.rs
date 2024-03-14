use super::super::business::*;
use super::types::*;

#[allow(unused_variables)]
impl Business for InnerState {
    #[allow(clippy::panic)] // ? checked
    fn business_example_query(&self) -> String {
        panic!("Not supported operation by this version.")
    }

    #[allow(clippy::panic)] // ? checked
    fn business_example_update(&mut self, test: String) {
        panic!("Not supported operation by this version.")
    }
}
