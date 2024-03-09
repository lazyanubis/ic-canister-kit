use super::super::business::*;
use super::types::*;

#[allow(unused_variables)]
impl Business for InnerState {
    fn business_test_template_query(&self) -> String {
        self.business.test_template.clone()
    }

    fn business_test_template_update(&mut self, test: String) {
        self.business.test_template = test
    }
}
