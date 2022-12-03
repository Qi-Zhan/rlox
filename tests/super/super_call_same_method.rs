use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Base {
  foo() {
    print "Base.foo()";
  }
}

class Derived < Base {
  foo() {
    print "Derived.foo()";
    super.foo();
  }
}

Derived().foo();
// expect: Derived.foo()
// expect: Base.foo()

"#;

#[test]
fn test_files_super_call_same_method() {
    let expected_output = vec!["Derived.foo()".to_string(),"Base.foo()".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
