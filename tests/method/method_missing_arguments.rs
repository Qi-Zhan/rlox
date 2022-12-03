use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  method(a, b) {}
}

Foo().method(1); // expect runtime error: Expected 2 arguments but got 1.

"#;

#[test]
fn test_files_method_missing_arguments() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
