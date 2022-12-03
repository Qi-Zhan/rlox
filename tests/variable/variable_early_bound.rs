use rlox::interpreter::run;
use rlox::result::InterpretResult;

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
#[ignore = "function"]
fn test_files_variable_early_bound() {
    let expected_output = vec!["outer".to_string(),"outer".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
