use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
var a = "1";
var a = "2";
print a; // expect: 2

"#;

#[test]
fn test_files_variable_redefine_global() {
    let expected_output = vec!["2".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
