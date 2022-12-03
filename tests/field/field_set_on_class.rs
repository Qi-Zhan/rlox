use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Foo {}
Foo.bar = "value"; // expect runtime error: Only instances have fields.

"#;

#[test]
fn test_files_field_set_on_class() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
