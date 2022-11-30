use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
fun f() {
  return "ok";
  print "bad";
}

print f(); // expect: ok

"#;

#[test]
fn test_files_return_in_function() {
    let expected_output = vec!["ok"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}