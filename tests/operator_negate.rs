use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
print -(3); // expect: -3
print --(3); // expect: 3
print ---(3); // expect: -3

"#;

#[test]
fn test_files_operator_negate() {
    let expected_output = vec!["-3","3","-3"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}