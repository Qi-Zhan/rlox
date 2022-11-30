use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
123.foo = "value"; // expect runtime error: Only instances have fields.

"#;

#[test]
fn test_files_field_set_on_num() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}