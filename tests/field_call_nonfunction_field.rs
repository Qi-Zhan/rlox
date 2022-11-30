use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {}

var foo = Foo();
foo.bar = "not fn";

foo.bar(); // expect runtime error: Can only call functions and classes.

"#;

#[test]
fn test_files_field_call_nonfunction_field() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}