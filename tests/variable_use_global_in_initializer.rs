use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
var a = "value";
var a = a;
print a; // expect: value

"#;

#[test]
fn test_files_variable_use_global_in_initializer() {
    let expected_output = vec!["value"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}