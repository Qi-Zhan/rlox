use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
var a = "outer";
{
  fun foo() {
    print a;
  }

  foo(); // expect: outer
  var a = "inner";
  foo(); // expect: outer
}

"#;

#[test]
fn test_files_variable_early_bound() {
    let expected_output = vec!["outer","outer"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}