use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Foo {}

var foo = Foo();
foo.bar = "not fn";

foo.bar(); // expect runtime error: Can only call functions and classes.

"#;

#[test]
fn test_files_field_call_nonfunction_field() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
