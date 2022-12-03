use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
nil.foo; // expect runtime error: Only instances have properties.

"#;

#[test]
fn test_files_field_get_on_nil() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
