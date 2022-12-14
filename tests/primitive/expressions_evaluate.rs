use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
// Note: This is just for the expression evaluating chapter which evaluates an
// expression directly.
print (5 - (3 - 1)) + -1;
// expect: 2

"#;

#[test]
fn test_files_expressions_evaluate() {
    let expected_output = vec!["2".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
