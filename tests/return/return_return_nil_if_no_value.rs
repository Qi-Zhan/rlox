use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
fun f() {
  return;
  print "bad";
}

print f(); // expect: nil

"#;

#[test]
fn test_files_return_return_nil_if_no_value() {
    let expected_output = vec!["nil".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
