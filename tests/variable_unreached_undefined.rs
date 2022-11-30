use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
if (false) {
  print notDefined;
}

print "ok"; // expect: ok

"#;

#[test]
fn test_files_variable_unreached_undefined() {
    let expected_output = vec!["ok"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}