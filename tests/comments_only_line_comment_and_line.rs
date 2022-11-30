use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// comment

"#;

#[test]
fn test_files_comments_only_line_comment_and_line() {
    let expected_output = vec![];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}