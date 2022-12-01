use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// [line 2] Error: Unterminated string.
"this string has no close quote
"#;

#[test]
fn test_files_string_unterminated() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::LexError{..}));
}
