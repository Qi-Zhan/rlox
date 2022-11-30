use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  method() {
    return "ok";
    print "bad";
  }
}

print Foo().method(); // expect: ok

"#;

#[test]
fn test_files_return_in_method() {
    let expected_output = vec!["ok"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}