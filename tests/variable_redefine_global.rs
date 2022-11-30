use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
var a = "1";
var a = "2";
print a; // expect: 2

"#;

#[test]
fn test_files_variable_redefine_global() {
    let expected_output = vec!["2"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}