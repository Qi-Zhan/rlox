use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
{
  var a = "local";
  {
    var a = "shadow";
    print a; // expect: shadow
  }
  print a; // expect: local
}

"#;

#[test]
fn test_files_variable_shadow_local() {
    let expected_output = vec!["shadow","local"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}