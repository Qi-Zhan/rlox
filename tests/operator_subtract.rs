use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
print 4 - 3; // expect: 1
print 1.2 - 1.2; // expect: 0

"#;

#[test]
fn test_files_operator_subtract() {
    let expected_output = vec!["1","0"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}