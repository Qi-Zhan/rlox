use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// [line 3] Error at 'c': Expect ')' after parameters.
// [c line 4] Error at end: Expect '}' after block.
fun foo(a, b c, d, e, f) {}

"#;

#[test]
fn test_files_function_missing_comma_in_parameters() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::LexError{..}));
}
