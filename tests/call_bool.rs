use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
true(); // expect runtime error: Can only call functions and classes.

"#;

#[test]
fn test_files_call_bool() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}