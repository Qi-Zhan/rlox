use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  returnSelf() {
    return Foo;
  }
}

print Foo().returnSelf(); // expect: Foo

"#;

#[test]
fn test_files_class_reference_self() {
    let expected_output = vec!["Foo"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}