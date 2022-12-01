use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
var f;

{
  var local = "local";
  fun f_() {
    print local;
  }
  f = f_;
}

f(); // expect: local

"#;

#[test]
fn test_files_closure_closed_closure_in_function() {
    let expected_output = vec!["local".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
