use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  init(a, b) {
    this.a = a;
    this.b = b;
  }
}

var foo = Foo(1, 2, 3, 4); // expect runtime error: Expected 2 arguments but got 4.
"#;

#[test]
fn test_files_constructor_extra_arguments() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}