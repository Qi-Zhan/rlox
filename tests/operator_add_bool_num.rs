use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
true + 123; // expect runtime error: Operands must be two numbers or two strings.

"#;

#[test]
fn test_files_operator_add_bool_num() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}