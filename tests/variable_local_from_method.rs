use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
var foo = "variable";

class Foo {
  method() {
    print foo;
  }
}

Foo().method(); // expect: variable

"#;

#[test]
fn test_files_variable_local_from_method() {
    let expected_output = vec!["variable".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
