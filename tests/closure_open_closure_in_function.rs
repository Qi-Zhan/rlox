use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
{
  var local = "local";
  fun f() {
    print local; // expect: local
  }
  f();
}

"#;

#[test]
fn test_files_closure_open_closure_in_function() {
    let expected_output = vec!["local"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}