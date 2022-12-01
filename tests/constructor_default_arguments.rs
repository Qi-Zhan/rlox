use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {}

var foo = Foo(1, 2, 3); // expect runtime error: Expected 0 arguments but got 3.

"#;

#[test]
fn test_files_constructor_default_arguments() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
