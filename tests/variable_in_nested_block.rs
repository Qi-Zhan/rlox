use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
{
  var a = "outer";
  {
    print a; // expect: outer
  }
}
"#;

#[test]
fn test_files_variable_in_nested_block() {
    let expected_output = vec!["outer"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}