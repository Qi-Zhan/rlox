use rlox::interpreter::run;
use rlox::error::InterpretResult;

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
    let expected_output = vec!["before","after","arg","arg"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}