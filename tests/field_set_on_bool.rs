use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
true.foo = "value"; // expect runtime error: Only instances have fields.

"#;

#[test]
fn test_files_field_set_on_bool() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
