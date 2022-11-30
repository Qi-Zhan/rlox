use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
{
  var a = "first";
  print a; // expect: first
}

{
  var a = "second";
  print a; // expect: second
}

"#;

#[test]
fn test_files_variable_scope_reuse_in_different_blocks() {
    let expected_output = vec!["first","second"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}