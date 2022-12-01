use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class A {
  method(arg) {
    print "A.method(" + arg + ")";
  }
}

class B < A {
  getClosure() {
    return super.method;
  }

  method(arg) {
    print "B.method(" + arg + ")";
  }
}


var closure = B().getClosure();
closure("arg"); // expect: A.method(arg)

"#;

#[test]
fn test_files_super_bound_method() {
    let expected_output = vec!["A.method(arg)".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
