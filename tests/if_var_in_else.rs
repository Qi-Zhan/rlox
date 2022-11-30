use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// [line 2] Error at 'var': Expect expression.
if (true) "ok"; else var foo;

"#;

#[test]
fn test_files_if_var_in_else() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::LexerError{..}));
}