use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
print 8 / 2;         // expect: 4
print 12.34 / 12.34;  // expect: 1

"#;

#[test]
fn test_files_operator_divide() {
    let expected_output = vec!["4","1"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}