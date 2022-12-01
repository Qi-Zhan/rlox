use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Base {
  foo(a, b) {
    print "Base.foo(" + a + ", " + b + ")";
  }
}

class Derived < Base {
  foo() {
    super.foo(1); // expect runtime error: Expected 2 arguments but got 1.
  }
}

Derived().foo();

"#;

#[test]
fn test_files_super_missing_arguments() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
