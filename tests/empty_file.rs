use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"

"#;

#[test]
fn test_files_empty_file() {
    let expected_output = vec![];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
