use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  init() {
    return "result"; // Error at 'return': Can't return a value from an initializer.
  }
}

"#;

#[test]
fn test_files_constructor_return_value() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
