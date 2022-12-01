use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class A {
  say() {
    print "A";
  }
}

class B < A {
  getClosure() {
    fun closure() {
      super.say();
    }
    return closure;
  }

  say() {
    print "B";
  }
}

class C < B {
  say() {
    print "C";
  }
}

C().getClosure()(); // expect: A

"#;

#[test]
fn test_files_super_super_in_closure_in_inherited_method() {
    let expected_output = vec!["A".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
