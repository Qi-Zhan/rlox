use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
-"s"; // expect runtime error: Operand must be a number.

"#;

#[test]
fn test_files_operator_negate_nonnum() {
    let expected_output = vec![];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
