use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
fun f() {
  for (;;) {
    var i = "i";
    return i;
  }
}

print f();
// expect: i

"#;

#[test]
fn test_files_for_return_inside() {
    let expected_output = vec!["i".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
