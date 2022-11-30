use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
{
  class A {}
  class B < A {}
  print B; // expect: B
}

"#;

#[test]
fn test_files_regression_394() {
    let expected_output = vec!["B"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}