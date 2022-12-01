use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
fun foo() {}

foo.bar = "value"; // expect runtime error: Only instances have fields.

"#;

#[test]
fn test_files_field_set_on_function() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
