use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class A {
  say() {
    print "A";
  }
}

class B < A {
  test() {
    super.say();
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

C().test(); // expect: A

"#;

#[test]
fn test_files_super_super_in_inherited_method() {
    let expected_output = vec!["A".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
