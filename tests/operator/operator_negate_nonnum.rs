use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
-"s"; // expect runtime error: Operand must be a number.

"#;

#[test]
fn test_files_operator_negate_nonnum() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
