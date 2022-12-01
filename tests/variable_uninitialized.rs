use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
var a;
print a; // expect: nil

"#;

#[test]
fn test_files_variable_uninitialized() {
    let expected_output = vec!["nil".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
