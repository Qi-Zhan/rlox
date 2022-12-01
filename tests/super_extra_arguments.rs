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
    print "Derived.foo()"; // expect: Derived.foo()
    super.foo("a", "b", "c", "d"); // expect runtime error: Expected 2 arguments but got 4.
  }
}

Derived().foo();

"#;

#[test]
fn test_files_super_extra_arguments() {
    let expected_output = vec![];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
