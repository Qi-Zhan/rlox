use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
print nil; // expect: nil

"#;

#[test]
fn test_files_nil_literal() {
    let expected_output = vec!["nil"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}