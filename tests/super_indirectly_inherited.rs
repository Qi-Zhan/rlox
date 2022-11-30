use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class A {
  foo() {
    print "A.foo()";
  }
}

class B < A {}

class C < B {
  foo() {
    print "C.foo()";
    super.foo();
  }
}

C().foo();
// expect: C.foo()
// expect: A.foo()

"#;

#[test]
fn test_files_super_indirectly_inherited() {
    let expected_output = vec!["C.foo()","A.foo()"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}