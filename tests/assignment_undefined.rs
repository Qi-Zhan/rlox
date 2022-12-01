use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
unknown = "what"; // expect runtime error: Undefined variable 'unknown'.

"#;

#[test]
fn test_files_assignment_undefined() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
