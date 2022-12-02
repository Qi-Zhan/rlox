use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
print nil; // expect: nil

"#;

#[test]
fn test_files_nil_literal() {
    let expected_output = vec!["nil".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
