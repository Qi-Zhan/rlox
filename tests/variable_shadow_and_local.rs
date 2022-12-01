use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
{
  var a = "outer";
  {
    print a; // expect: outer
    var a = "inner";
    print a; // expect: inner
  }
}
"#;

#[test]
fn test_files_variable_shadow_and_local() {
    let expected_output = vec!["outer".to_string(),"inner".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
