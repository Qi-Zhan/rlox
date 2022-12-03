use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
if (false) {
  print notDefined;
}

print "ok"; // expect: ok

"#;

#[test]
fn test_files_variable_unreached_undefined() {
    let expected_output = vec!["ok".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
