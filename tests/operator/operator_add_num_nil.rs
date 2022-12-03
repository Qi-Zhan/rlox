use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
1 + nil; // expect runtime error: Operands must be two numbers or two strings.

"#;

#[test]
fn test_files_operator_add_num_nil() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
