use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
TBD_SOURCE
"#;

#[test]
fn _test_name_() {
    let expected_output = vec!["TBD_OUTPUT"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}