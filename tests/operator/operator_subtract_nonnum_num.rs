use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
"1" - 1; // expect runtime error: Operands must be numbers.

"#;

#[test]
fn test_files_operator_subtract_nonnum_num() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
