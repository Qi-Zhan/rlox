use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  method() {
    print method; // expect runtime error: Undefined variable 'method'.
  }
}

Foo().method();

"#;

#[test]
fn test_files_method_refer_to_name() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}