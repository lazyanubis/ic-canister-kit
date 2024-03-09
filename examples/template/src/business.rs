#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// 查询
#[ic_cdk::query(guard = "has_business_test_template_query")]
fn business_test_template_query() -> String {
    with_state(|s| s.business_test_template_query())
}

// 修改
#[ic_cdk::update(guard = "has_business_test_template_set")]
fn business_test_template_set(test: String) {
    let caller = caller();
    let arg_content = format!("set test: {}", test); // * 记录参数内容

    with_mut_state(
        |s| {
            s.business_test_template_update(test);
            (None, ())
        },
        caller,
        RecordTopics::Template.topic(),
        arg_content,
    );
}
