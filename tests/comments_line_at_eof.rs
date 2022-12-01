use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
print "ok"; // expect: ok
// comment
"#;

#[test]
fn test_files_comments_line_at_eof() {
    let expected_output = vec!["ok".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
