use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class A {}

fun f() {
  class B < A {}
  return B;
}

print f(); // expect: B

"#;

#[test]
fn test_files_class_local_inherit_other() {
    let expected_output = vec!["B".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
