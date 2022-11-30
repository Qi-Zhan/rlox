use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// [line 3] Error at '123': Expect '{' before function body.
// [c line 4] Error at end: Expect '}' after block.
fun f() 123;

"#;

#[test]
fn test_files_function_body_must_be_block() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::LexerError{..}));
}