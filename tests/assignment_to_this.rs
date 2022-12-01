use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  Foo() {
    this = "value"; // Error at '=': Invalid assignment target.
  }
}

Foo();

"#;

#[test]
fn test_files_assignment_to_this() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
