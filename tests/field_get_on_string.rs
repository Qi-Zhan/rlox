use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
"str".foo; // expect runtime error: Only instances have properties.

"#;

#[test]
fn test_files_field_get_on_string() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}