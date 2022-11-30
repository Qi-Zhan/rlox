use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
fun f() {}
print f(); // expect: nil

"#;

#[test]
fn test_files_function_empty_body() {
    let expected_output = vec!["nil"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}