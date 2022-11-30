use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Base {
  foo() {
    print "Base.foo()";
  }
}

class Derived < Base {
  bar() {
    print "Derived.bar()";
    super.foo();
  }
}

Derived().bar();
// expect: Derived.bar()
// expect: Base.foo()

"#;

#[test]
fn test_files_super_call_other_method() {
    let expected_output = vec!["Derived.bar()","Base.foo()"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}