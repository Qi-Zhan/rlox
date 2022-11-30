use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
1 >= "1"; // expect runtime error: Operands must be numbers.

"#;

#[test]
fn test_files_operator_greater_or_equal_num_nonnum() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}