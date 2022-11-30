use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
{} // By itself.

// In a statement.
if (true) {}
if (false) {} else {}

print "ok"; // expect: ok

"#;

#[test]
fn test_files_block_empty() {
    let expected_output = vec!["ok"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}