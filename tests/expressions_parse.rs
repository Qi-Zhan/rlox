use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// Note: This is just for the expression parsing chapter which prints the AST.
(5 - (3 - 1)) + -1
// expect: (+ (group (- 5.0 (group (- 3.0 1.0)))) (- 1.0))

"#;

#[test]
fn test_files_expressions_parse() {
    let expected_output = vec!["(+ (group (- 5.0 (group (- 3.0 1.0)))) (- 1.0))".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
