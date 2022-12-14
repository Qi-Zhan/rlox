use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
123.foo; // expect runtime error: Only instances have properties.

"#;

#[test]
fn test_files_field_get_on_num() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
