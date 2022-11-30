use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
print !true;    // expect: false
print !false;   // expect: true
print !!true;   // expect: true

"#;

#[test]
fn test_files_bool_not() {
    let expected_output = vec!["false","true","true"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}