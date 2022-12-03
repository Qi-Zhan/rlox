use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
fun f() {
  while (true) return "ok";
}

print f(); // expect: ok

"#;

#[test]
fn test_files_return_after_while() {
    let expected_output = vec!["ok".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
