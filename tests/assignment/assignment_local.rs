use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
{
  var a = "before";
  print a; // expect: before

  a = "after";
  print a; // expect: after

  print a = "arg"; // expect: arg
  print a; // expect: arg
}

"#;

#[test]
fn test_files_assignment_local() {
    let expected_output = vec!["before".to_string(),"after".to_string(),"arg".to_string(),"arg".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
