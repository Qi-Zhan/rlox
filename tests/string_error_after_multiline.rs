use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// Tests that we correctly track the line info across multiline strings.
var a = "1
2
3
";

err; // // expect runtime error: Undefined variable 'err'.
"#;

#[test]
fn test_files_string_error_after_multiline() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
