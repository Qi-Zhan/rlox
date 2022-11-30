use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
print 5 * 3; // expect: 15
print 12.34 * 0.3; // expect: 3.702

"#;

#[test]
fn test_files_operator_multiply() {
    let expected_output = vec!["15","3.702"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}