use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
var a = "1";
var a;
print a; // expect: nil

"#;

#[test]
fn test_files_variable_redeclare_global() {
    let expected_output = vec!["nil"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}