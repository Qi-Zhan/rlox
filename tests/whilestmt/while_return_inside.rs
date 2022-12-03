use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
fun f() {
  while (true) {
    var i = "i";
    return i;
  }
}

print f();
// expect: i

"#;

#[test]
#[ignore="function"]
fn test_files_while_return_inside() {
    let expected_output = vec!["i".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
