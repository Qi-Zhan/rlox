use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
true + "s"; // expect runtime error: Operands must be two numbers or two strings.

"#;

#[test]
fn test_files_operator_add_bool_string() {
    let expected_output = vec![];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
