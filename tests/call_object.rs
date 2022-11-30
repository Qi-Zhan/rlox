use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {}

var foo = Foo();
foo(); // expect runtime error: Can only call functions and classes.

"#;

#[test]
fn test_files_call_object() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}