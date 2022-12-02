use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
print !true;    // expect: false
print !false;   // expect: true
print !!true;   // expect: true

"#;

#[test]
fn test_files_bool_not() {
    let expected_output = vec!["false".to_string(),"true".to_string(),"true".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
