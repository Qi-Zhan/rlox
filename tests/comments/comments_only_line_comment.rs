use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
// comment
"#;

#[test]
fn test_files_comments_only_line_comment() {
    let expected_output = vec![];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
