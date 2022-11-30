use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {}
var foo = Foo();

foo.bar; // expect runtime error: Undefined property 'bar'.

"#;

#[test]
fn test_files_field_undefined() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}