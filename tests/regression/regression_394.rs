use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
{
  class A {}
  class B < A {}
  print B; // expect: B
}

"#;

#[test]
fn test_files_regression_394() {
    let expected_output = vec!["B".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
