use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
fun f() {
  if (false) "no"; else return "ok";
}

print f(); // expect: ok

"#;

#[test]
fn test_files_return_after_else() {
    let expected_output = vec!["ok"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}