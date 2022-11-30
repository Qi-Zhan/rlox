use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {}

Foo().unknown(); // expect runtime error: Undefined property 'unknown'.

"#;

#[test]
fn test_files_method_not_found() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}