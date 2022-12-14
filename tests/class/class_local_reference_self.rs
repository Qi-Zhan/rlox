use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
{
  class Foo {
    returnSelf() {
      return Foo;
    }
  }

  print Foo().returnSelf(); // expect: Foo
}

"#;

#[test]
fn test_files_class_local_reference_self() {
    let expected_output = vec!["Foo".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
