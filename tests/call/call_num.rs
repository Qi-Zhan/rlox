use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
123(); // expect runtime error: Can only call functions and classes.

"#;

#[test]
fn test_files_call_num() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
